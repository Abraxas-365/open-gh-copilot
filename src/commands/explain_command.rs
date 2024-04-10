use langchain_rust::{chain::Chain, prompt_args};

use crate::{
    chains::explain_command_chain,
    util::shared::{apply_styles_to_backticks, SharedState},
};

pub async fn explain_command(
    shared_state: &SharedState,
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let explain_chain = explain_command_chain(shared_state.llm());
    let explanation = explain_chain
        .invoke(prompt_args! {
            "os" => shared_state.os(),
            "command" => command
        })
        .await?;

    println!("Explanation:\n");
    println!("{}\n", apply_styles_to_backticks(&explanation));

    Ok(())
}
