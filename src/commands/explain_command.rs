use langchain_rust::{chain::Chain, language_models::llm::LLM, prompt_args};

use crate::{
    chains::explain_command_chain,
    util::{apply_styles_to_backticks, shared::SharedState},
};

pub async fn explain_command<LLMType: LLM + Clone + 'static>(
    shared_state: &SharedState<LLMType>, // Explicit lifetimes
    command: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let explain_chain = explain_command_chain(shared_state.llm());
    let explanation = explain_chain
        .invoke(prompt_args! {
            "os" => shared_state.os(),
            "command" => command
        })
        .await?; // Handling errors correctly

    println!("Explanation:\n");
    println!("{}\n", apply_styles_to_backticks(&explanation));

    Ok(())
}
