use dialoguer::console::style;
use langchain_rust::{chain::Chain, prompt_args};

use crate::{
    chains::recomend_command_chain,
    cli::{choose_options, choose_sugestion_options},
    util::shared::SharedState,
};

pub async fn suggest_command(shared_state: &SharedState, command: Option<&str>) {
    let recommend_chian = recomend_command_chain(shared_state.llm());
    if let Some(input) = command {
        let suggestion = recommend_chian
            .invoke(prompt_args! {
                "os"=>shared_state.os(),
                "command"=>input
            })
            .await
            .unwrap();
        println!("Sugestion:\n");
        println!("{}\n", style(suggestion.clone()).yellow().bold());
        choose_options(shared_state, &suggestion).await;
    } else {
        choose_sugestion_options(shared_state).await
    }
}
