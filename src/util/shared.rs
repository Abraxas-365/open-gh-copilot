use dialoguer::console::{style, StyledObject};
use langchain_rust::{
    llm::{AzureConfig, Claude, OpenAI},
    tools::OpenAIConfig,
};
use regex::Regex;

use crate::COPILOT_PATH;

use super::{config::LLMConfig, llm::LLMVariant};

pub struct SharedState {
    llm: LLMVariant,
    os: String,
}

impl SharedState {
    pub fn new(llm: LLMVariant, os: &str) -> Self {
        SharedState {
            llm,
            os: os.to_string(),
        }
    }

    pub fn llm(&self) -> LLMVariant {
        self.llm.clone()
    }

    pub fn os(&self) -> &str {
        &self.os
    }
}

impl Default for SharedState {
    fn default() -> Self {
        let os = std::env::consts::OS;
        match read_config() {
            Some(cfg) => {
                match cfg.llm_type.as_str() {
                    "openai" => {
                        let mut llm = OpenAI::default().with_config(OpenAIConfig::default());
                        let mut config = OpenAIConfig::default();
                        if let Some(api_key) = cfg.api_key {
                            config = config.with_api_key(api_key);
                        }
                        if let Some(api_base) = cfg.api_base {
                            llm = llm.with_config(config.with_api_base(api_base));
                        }
                        if let Some(model) = cfg.model {
                            llm = llm.with_model(&model);
                        }
                        return SharedState::new(LLMVariant::OpenAI(llm), os);
                    }
                    "ollama" => {
                        let mut llm = OpenAI::default();
                        let mut config = OpenAIConfig::default();
                        config = config.with_api_key(cfg.api_key.unwrap_or("ollama".to_string()));
                        config = config.with_api_base(
                            cfg.api_base
                                .unwrap_or("http://localhost:11434/v1".to_string()),
                        );
                        llm = llm.with_model(&cfg.model.unwrap_or("llama2".to_string()));
                        llm = llm.with_config(config);

                        return SharedState::new(LLMVariant::OpenAI(llm), os);
                    }

                    "anthropic" => {
                        let mut llm = Claude::default();
                        if let Some(api_key) = cfg.api_key {
                            llm = llm.with_api_key(api_key);
                        }
                        if let Some(model) = cfg.model {
                            llm = llm.with_model(&model);
                        }
                        return SharedState::new(LLMVariant::Anthropic(llm), os);
                    }

                    "azure_openai" => {
                        let azure_config = AzureConfig::default()
                            .with_api_key(cfg.api_key.expect("Azure API key not found"))
                            .with_api_base(cfg.api_base.expect("Azure API base not found"))
                            .with_api_version(cfg.api_version.expect("Azure API version not found"))
                            .with_deployment_id(
                                cfg.deployment.expect("Azure deployment ID not found"),
                            );
                        let llm = OpenAI::new(azure_config);
                        return SharedState::new(LLMVariant::AzureOpenAI(llm), os);
                    }

                    // Handle other LLM types here
                    _ => panic!("Unsupported LLM type"),
                }
            }
            None => SharedState {
                llm: LLMVariant::OpenAI(OpenAI::default()),
                os: os.to_string(),
            },
        }
    }
}

fn read_config() -> Option<LLMConfig> {
    let mut config_path = home::home_dir()?;
    config_path.push(COPILOT_PATH);

    let config_contents = std::fs::read_to_string(config_path).ok()?;
    let config: Option<LLMConfig> = toml::from_str(&config_contents)
        .map_err(|e| {
            println!("Error reading config: {}", e);
        })
        .ok();
    if config.is_none() {
        return None;
    }
    Some(config.unwrap())
}

pub fn intro() {
    println!(
        "\n{}\n",
        style("Welcome to GitHub Copilot in the CLI!")
            .white()
            .bright()
            .bold()
    );
    println!("{}\n",
    style("I'm powered by AI, so surprises and mistakes are possible. Make sure to verify any generated code or suggestions, and share feedback so that we can learn and improve.").white().dim());
}

pub fn apply_styles_to_backticks(text: &str) -> String {
    let re = Regex::new(r"`([^`]*)`").unwrap();
    let mut result_text = text.to_string();

    for cap in re.captures_iter(text) {
        let matched_text: &str = &cap[1];
        let styled: StyledObject<String> = style(matched_text.to_string()).yellow().bold();
        result_text = result_text.replacen(&format!("`{}`", matched_text), &styled.to_string(), 1);
    }

    result_text
}
