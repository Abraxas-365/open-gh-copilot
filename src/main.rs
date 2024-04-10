#![allow(dead_code)]

use clap::ArgMatches;
use cli::init_clap;
use langchain_rust::{
    language_models::llm::LLM,
    llm::{Claude, OpenAI},
    tools::OpenAIConfig,
};
use util::{intro, shared::SharedState, AppConfig};

mod chains;
mod cli;
mod commands;
mod util;

#[tokio::main]
async fn main() {
    let matches = init_clap();
    let mut open_ai = Some(OpenAI::default());
    let mut anthropic = None;
    match read_config() {
        Some(cfg) => {
            match cfg.llm.llm_type.as_str() {
                "openai" => {
                    let mut llm = OpenAI::default().with_config(OpenAIConfig::default());
                    let mut config = OpenAIConfig::default();
                    if let Some(api_key) = cfg.llm.api_key {
                        config = config.with_api_key(api_key);
                    }
                    if let Some(api_base) = cfg.llm.api_base {
                        llm = llm.with_config(config.with_api_base(api_base));
                    }
                    if let Some(model) = cfg.llm.model {
                        llm = llm.with_model(&model);
                    }
                    open_ai = Some(llm);
                }
                "ollama" => {
                    let mut llm = OpenAI::default();
                    let mut config = OpenAIConfig::default();
                    config = config.with_api_key(cfg.llm.api_key.unwrap_or("ollama".to_string()));
                    config = config.with_api_base(
                        cfg.llm
                            .api_base
                            .unwrap_or("http://localhost:11434/v1".to_string()),
                    );
                    llm = llm.with_model(&cfg.llm.model.unwrap_or("llama2".to_string()));
                    llm = llm.with_config(config);

                    open_ai = Some(llm);
                }

                "anthropic" => {
                    let mut llm = Claude::default();
                    if let Some(api_key) = cfg.llm.api_key {
                        llm = llm.with_api_key(api_key);
                    }
                    if let Some(model) = cfg.llm.model {
                        llm = llm.with_model(&model);
                    }
                    anthropic = Some(llm);
                }

                // Handle other LLM types here
                _ => panic!("Unsupported LLM type"),
            }
        }
        None => open_ai = Some(OpenAI::default()),
    };

    let os = std::env::consts::OS;

    match anthropic {
        Some(anthropic) => {
            let shared_state = util::shared::SharedState::new(anthropic, os);
            init(matches, &shared_state).await;
        }
        None => {
            let shared_state = util::shared::SharedState::new(open_ai.unwrap(), os);
            init(matches, &shared_state).await;
        }
    }
}

async fn init(matches: ArgMatches, shared_state: &SharedState<impl LLM + Clone + 'static>) {
    intro();

    match matches.subcommand() {
        Some(("explain", args)) => {
            let command = args.value_of("command").unwrap();
            commands::explain_command(&shared_state, command)
                .await
                .unwrap();
        }
        Some(("suggest", args)) => {
            let input = args.value_of("input");
            commands::suggest_command(&shared_state, input).await;
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}

fn read_config() -> Option<AppConfig> {
    let mut config_path = home::home_dir()?;
    config_path.push(".free_copilot_cli");

    let config_contents = std::fs::read_to_string(config_path).ok()?;
    let config: AppConfig = toml::from_str(&config_contents).ok()?;

    Some(config)
}
