#![allow(dead_code)]

use cli::{choose_model_config, init_clap};
use util::{intro, shared::SharedState};

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

        Some(("config", _)) => choose_model_config().await,
        _ => {
            println!("No subcommand was used");
        }
    }
}
