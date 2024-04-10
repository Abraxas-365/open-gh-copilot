use std::process::Command;

use async_recursion::async_recursion;
use clap::{App, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{console::style, theme::ColorfulTheme, Input, Select};
use langchain_rust::{chain::Chain, language_models::llm::LLM, prompt_args};

use crate::{
    chains::{
        explain_command_chain, recomend_command_chain, recomend_command_git_chain,
        recomend_command_github_chain, revise_command_chain,
    },
    util::{apply_styles_to_backticks, shared::SharedState},
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
        .get_matches()
}

pub async fn choose_sugestion_options<LLMType: LLM + Clone + 'static>(
    shared_state: &SharedState<LLMType>,
) {
    let opciones = vec!["Generic Shell Command", "Git Command", "GitHub Command"];

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

        _ => eprintln!("Invalid option."),
    }
}

#[async_recursion]
pub async fn choose_options<LLMType: LLM + Clone + 'static>(
    shared_state: &SharedState<LLMType>,
    input: &str,
) {
    let opciones = vec![
        "Copy to clipboard",
        "Execute Command",
        "Explain Command",
        "Revise  Command",
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

        _ => eprintln!("Invalid option."),
    }
}
