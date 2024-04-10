use langchain_rust::{
    chain::{LLMChain, LLMChainBuilder},
    language_models::llm::LLM,
    prompt::HumanMessagePromptTemplate,
    template_jinja2,
};

const PROMPT: &str = r#"Explain the following {{os}} command: {{command}}.
You should explain the command using bullet points, in  markdown format.

Exmaple:
command: git reset --soft HEAD~1 git rm mistake.md git commit -c ORIG_HEAD

explanation:
• `git reset` resets the current branch to a previous commit.
    • `--soft` means that we keep the changes made to the files in the working directory.
• HEAD~1 specifies that we reset to the commit one before the current one.
• `git rm` removes a file from Git.
    • `mistake.md` is the name of file to be removed.
• `git commit` creates a new commit.
    • `-c` ORIG_HEAD specifies that we copy the commit message from the last commit.


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
