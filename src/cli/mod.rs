use std::process::Command;

use async_recursion::async_recursion;
use clap::{App, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{console::style, theme::ColorfulTheme, Input, Select};
use langchain_rust::{chain::Chain, prompt_args};

use crate::{
    chains::{
        explain_command_chain, recomend_command_chain, recomend_command_git_chain,
        recomend_command_github_chain, revise_command_chain, revise_commit_chain,
    },
    util::shared::{apply_styles_to_backticks, SharedState},
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
        .subcommand(
            clap::Command::new("commit")
                .about("Auto Commit Message")
                .arg(
                    Arg::new("excluded")
                        .long("excluded")
                        .short('e')
                        .help("Files or paths to exclude from the commit")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(false),
                )
                .arg(
                    Arg::new("context")
                        .long("context")
                        .short('c')
                        .help("Optional context for the commit")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .get_matches()
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
            choose_options(shared_state, &input).await
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

#[async_recursion]
pub async fn choose_options_for_commit(shared_state: &SharedState, input: &str) {
    let opciones = vec![
        "Copy to clipboard",
        "Commit message",
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
        1 => match Command::new("sh")
            .arg("-c")
            .arg(format!(r#"git commit -m "{}""#, input))
            .status()
        {
            Ok(status) if status.success() => println!("Command executed successfully."),
            _ => eprintln!("Error executing command."),
        },
        2 => {
            let revised = Input::<String>::new()
                .with_prompt(style("> ").blue().bold().to_string())
                .interact()
                .unwrap();
            let revise_chain = revise_commit_chain(shared_state.llm());
            let revised_command = revise_chain
                .invoke(prompt_args! {
                    "to"=>revised,
                    "commit"=>input
                })
                .await
                .unwrap();
            println!("Sugestion:\n");
            println!("{}\n", style(revised_command.clone()).yellow().bold());
            choose_options_for_commit(shared_state, &revised_command).await
        }
        3 => std::process::exit(0),

        _ => eprintln!("Invalid option."),
    }
}
