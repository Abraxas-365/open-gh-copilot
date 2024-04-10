use std::process::Command;

use async_recursion::async_recursion;
use clap::{App, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{console::style, theme::ColorfulTheme, Input, Select};
use langchain_rust::{chain::Chain, prompt_args};

use crate::{
    chains::{
        explain_command_chain, recomend_command_chain, recomend_command_git_chain,
        recomend_command_github_chain, revise_command_chain,
    },
    util::{
        apply_styles_to_backticks,
        shared::{save_config, SharedState},
        LLMConfig,
    },
};

pub fn init_clap() -> clap::ArgMatches {
    App::new("Free GitHub Copilot")
        .version("0.1.0")
        .author("Luis Fernando luisfmiranda8@gmail.com")
        .about("GitHub Copilot Clone")
        .subcommand(
            clap::Command::new("explain")
                .about("Exmplain a command")
                .arg(
                    Arg::new("command")
                        .help("The command to explain")
                        .required(true)
                        .takes_value(true),
                ),
        ) // Aquí se usan los nuevos métodos.
        .subcommand(
            clap::Command::new("suggest")
                .about("Recommend a command")
                .arg(Arg::new("input").help("The input to suggest a command for")),
        )
        .subcommand(clap::Command::new("config").about("Configure your LLM provider"))
        .get_matches()
}

pub async fn choose_model_config() {
    let opciones = vec!["OpenAi", "Ollama", "Anthropic", "Exit"];

    let instrucciones = style("[Use arrows to move, type to filter]")
        .yellow()
        .to_string();
    let prompt = format!("Select an option  {}", instrucciones);

    let seleccion = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt)
        .default(0)
        .items(&opciones[..])
        .interact()
        .unwrap();

    match seleccion {
        0 => {
            let mut config = LLMConfig::new_openai();

            let model_prompt = "Enter the model (press Enter for default 'gpt-3.5-turbo'):";
            let model_input: String = Input::new()
                .with_prompt(model_prompt)
                .default("gpt-3.5-turbo".into())
                .interact_text()
                .unwrap();
            config.model = Some(model_input);

            let key_prompt =
                "Enter your OpenAi key (press Enter for use your env $OPENAI_API_KEY):";
            let key: String = Input::new()
                .with_prompt(key_prompt)
                .default("None".into())
                .interact_text()
                .unwrap();
            if key != "None" {
                config.api_key = Some(key);
            }

            save_config(&config)
                .map_err(|e| eprintln!("{}", e))
                .unwrap();
        }
        1 => {
            let mut config = LLMConfig::new_ollama();
            let model_prompt = "Enter the model (press Enter for default 'llama2'):";
            let model_input: String = Input::new()
                .with_prompt(model_prompt)
                .default("llama2".into())
                .interact_text()
                .unwrap();
            config.model = Some(model_input);

            let key_prompt = "Enter your OpenAi key (press Enter for use default 'ollama'):";
            let key: String = Input::new()
                .with_prompt(key_prompt)
                .default("ollama".into())
                .interact_text()
                .unwrap();
            config.api_key = Some(key);

            let base_prompt =
                "Enter the base URL (press Enter for default 'http://localhost:11434/v1'):";
            let base: String = Input::new()
                .with_prompt(base_prompt)
                .default("http://localhost:11434/v1".into())
                .interact_text()
                .unwrap();
            config.api_base = Some(base);

            save_config(&config)
                .map_err(|e| eprintln!("{}", e))
                .unwrap();
        }
        2 => {
            let mut config = LLMConfig::new_anthropic();
            let model_prompt =
                "Enter the model (press Enter for default 'claude-3-opus-20240229'):";
            let model_input: String = Input::new()
                .with_prompt(model_prompt)
                .default("claude-3-opus-20240229".into())
                .interact_text()
                .unwrap();
            config.model = Some(model_input);

            let key_prompt =
                "Enter your Anthropic key (press Enter for use your env $CLOUDE_API_KEY):";
            let key: String = Input::new()
                .with_prompt(key_prompt)
                .default("ollama".into())
                .interact_text()
                .unwrap();
            config.api_key = Some(key);

            save_config(&config)
                .map_err(|e| eprintln!("{}", e))
                .unwrap();
        }
        3 => std::process::exit(0),
        _ => eprintln!("Invalid option."),
    }
}

pub async fn choose_sugestion_options(shared_state: &SharedState) {
    let opciones = vec![
        "Generic Shell Command",
        "Git Command",
        "GitHub Command",
        "Exit",
    ];

    let instrucciones = style("[Use arrows to move, type to filter]")
        .yellow()
        .to_string();
    let prompt = format!("Select an option  {}", instrucciones);

    let seleccion = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt)
        .default(0)
        .items(&opciones[..])
        .interact()
        .unwrap();

    match seleccion {
        0 => {
            println!(
                "\n{}\n",
                style("What would you like the command to do?")
                    .white()
                    .bold()
                    .bright()
                    .to_string(),
            );
            let command = Input::<String>::new()
                .with_prompt(style("> ").blue().bold().to_string())
                .interact()
                .unwrap();
            let suggest_chain = recomend_command_chain(shared_state.llm());
            let suggestion = suggest_chain
                .invoke(prompt_args! {
                    "os"=>shared_state.os(),
                    "command"=>command
                })
                .await
                .unwrap();
            println!("Sugestion:\n");
            println!("{}\n", style(suggestion.clone()).yellow().bold());
            choose_options(shared_state, &suggestion).await
        }
        1 => {
            println!(
                "\n{}\n",
                style("What would you like the git command to do?")
                    .white()
                    .bold()
                    .bright()
                    .to_string(),
            );
            let git_command = Input::<String>::new()
                .with_prompt(style("> ").blue().bold().to_string())
                .interact()
                .unwrap();
            let suggest_chain = recomend_command_git_chain(shared_state.llm());
            let suggestion = suggest_chain
                .invoke(prompt_args! {
                    "command"=>git_command
                })
                .await
                .unwrap();
            println!("Sugestion:\n");
            println!("{}\n", style(suggestion.clone()).yellow().bold());
            choose_options(shared_state, &suggestion).await
        }
        2 => {
            println!(
                "\n{}\n",
                style("What would you like the GitHub command to do?")
                    .white()
                    .bold()
                    .bright()
                    .to_string(),
            );
            let github_command = Input::<String>::new()
                .with_prompt(style("> ").blue().bold().to_string())
                .interact()
                .unwrap();
            let suggest_chain = recomend_command_github_chain(shared_state.llm());
            let suggestion = suggest_chain
                .invoke(prompt_args! {
                    "command"=>github_command
                })
                .await
                .unwrap();
            println!("Sugestion:\n");
            println!("{}\n", style(suggestion.clone()).yellow().bold());
            choose_options(shared_state, &suggestion).await
        }
        3 => std::process::exit(0),

        _ => eprintln!("Invalid option."),
    }
}

#[async_recursion]
pub async fn choose_options(shared_state: &SharedState, input: &str) {
    let opciones = vec![
        "Copy to clipboard",
        "Execute Command",
        "Explain Command",
        "Revise  Command",
        "Exit",
    ];

    let instrucciones = style("[Use arrows to move, type to filter]")
        .yellow()
        .to_string();
    let prompt = format!("Select an option  {}", instrucciones);

    let seleccion = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt)
        .default(0)
        .items(&opciones[..])
        .interact()
        .unwrap();

    match seleccion {
        0 => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(input.into()).unwrap();
        }
        1 => match Command::new("sh").arg("-c").arg(input).status() {
            Ok(status) if status.success() => println!("Command executed successfully."),
            _ => eprintln!("Error executing command."),
        },
        2 => {
            let explain_chian = explain_command_chain(shared_state.llm());
            let explanation = explain_chian
                .invoke(prompt_args! {
                    "os"=>shared_state.os(),
                    "command"=>input
                })
                .await
                .unwrap();
            println!("Explanation:\n");
            println!("{}\n", apply_styles_to_backticks(&explanation));
        }
        3 => {
            let revised = Input::<String>::new()
                .with_prompt(style("> ").blue().bold().to_string())
                .interact()
                .unwrap();
            let revise_chain = revise_command_chain(shared_state.llm());
            let revised_command = revise_chain
                .invoke(prompt_args! {
                    "to"=>revised,
                    "command"=>input
                })
                .await
                .unwrap();
            println!("Sugestion:\n");
            println!("{}\n", style(revised_command.clone()).yellow().bold());
            choose_options(shared_state, &revised_command).await
        }
        4 => std::process::exit(0),

        _ => eprintln!("Invalid option."),
    }
}
