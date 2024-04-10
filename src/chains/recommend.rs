use langchain_rust::{
    chain::{LLMChain, LLMChainBuilder},
    language_models::llm::LLM,
    prompt::HumanMessagePromptTemplate,
    template_jinja2,
};

const PROMPT: &str = r#"Recommend a terminal command for {{os}} to : {{command}}.
You sould just return the command or command, nothing more.
Example1:
ls

Example2:
git init 
git add .
git commit -m "first commit"
git remote add origin <repository_url>
git push -u origin main
"#;

/// This function creates a chain that explains a command.
pub fn recomend_command_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT, "os", "command"
        )))
        .build()
        .unwrap() //safe to unwrap
}

const PROMPT_GIT: &str = r#"Recommend a git command or commands to : {{command}}.
You sould just return the command or command, nothing more, give all the necesary command to acomplish.
Example:
git init 
git add .
git commit -m "first commit"
git remote add origin <repository_url>
git push -u origin main
"#;

/// This function creates a chain that explains a command.
pub fn recomend_command_git_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT_GIT, "command"
        )))
        .build()
        .unwrap() //safe to unwrap
}

const PROMPT_GITHUB: &str = r#"Recommend a github cli command or commands to : {{command}}.
You sould just return the command or command, nothing more, give all the necesary command to acomplish.

Example:
gh issue list
"#;

/// This function creates a chain that explains a command.
pub fn recomend_command_github_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT_GITHUB,
            "command"
        )))
        .build()
        .unwrap() //safe to unwrap
}
