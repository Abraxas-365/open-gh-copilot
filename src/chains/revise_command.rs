use langchain_rust::{
    chain::{LLMChain, LLMChainBuilder},
    language_models::llm::LLM,
    prompt::HumanMessagePromptTemplate,
    template_jinja2,
};

const PROMPT: &str = r#"
Giving this command:{{command}}, modify it to {{to}}.
You sould just return the command, nothing more.
"#;

/// This function creates a chain that explains a command.
pub fn revise_command_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT, "to", "command"
        )))
        .build()
        .unwrap() //safe to unwrap
}
