#![allow(dead_code)]

use cli::init_clap;
use util::{
    config::LLMConfig,
    shared::{intro, SharedState},
};

mod chains;
mod cli;
mod commands;
mod util;
pub const COPILOT_PATH: &str = ".free_copilot_cli";

#[tokio::main]
async fn main() {
    let matches = init_clap();
    let shared_state = SharedState::default();

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

        Some(("config", _)) => LLMConfig::new_config().await,

        Some(("commit", args)) => {
            let context = args.value_of("context");
            let exclude = args.values_of("exclude").map(|v| v.collect::<Vec<&str>>());

            commands::git_commit_command(&shared_state, context, exclude.as_deref()).await;
        }

        _ => {
            println!("No subcommand was used");
        }
    }
}
