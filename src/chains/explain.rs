use langchain_rust::{
    chain::{LLMChain, LLMChainBuilder},
    language_models::llm::LLM,
    prompt::HumanMessagePromptTemplate,
    template_jinja2,
};

const PROMPT: &str = r#"Explain the following {{os}} command: {{command}}.
You should explain the command using bullet points, in  markdown format.

Exmaple Command: ls -la

Example Result
• `ls`  Lists the contents of a directory. displays the names of files and directories in the current directory.
  • `-l` (part of `-la`) Displays the listing in long format, providing detailed information such as permissions, number of links, owner, group, size, and timestamp for each file and directory.
  • `-a` (part of `-la`) Includes entries that start with a dot (.), showing hidden files alongside the regular listings.


"#;

/// This function creates a chain that explains a command.
pub fn explain_command_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT, "os", "command"
        )))
        .build()
        .unwrap() //safe to unwrap
}
