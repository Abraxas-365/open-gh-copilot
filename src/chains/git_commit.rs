use langchain_rust::chain::{LLMChain, LLMChainBuilder};
use langchain_rust::language_models::llm::LLM;
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::template_jinja2;

const PROMPT: &str = r#"
"{{input}}

You Should:
- Create a conventional commit message reflecting these modifications.
- Ensure the message adheres to the conventional commit format,
  including identifying the type of change (feat, fix, docs, style, refactor, test, chore),
  a brief and imperative summary of the changes, and optionally a scope.
- YOU SHOULD ONLY RETURN ONE COMMIT MESSAGE.

Answer Example:
feat(parser): add ability to parse JSON files.
"#;

const PROMPT_WITH_CONTEXT: &str = r#"
Given file changes described as "{{input}}" and additional context provided by "{{context}}"

You Should:
- Create a conventional commit message reflecting these modifications.
- Ensure the message adheres to the conventional commit format, which includes identifying the type of
  change (feat, fix, docs, style, refactor, test, chore), providing a brief and imperative
  summary of the changes, specifying a scope (if applicable), and incorporating the
  provided context to better explain the modifications. 
- YOU SHOULD ONLY RETURN ONE COMMIT MESSAGE.

Answer Example:
feat(parser):add ability to parse JSON files.

"#;
pub fn git_commit_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT, "input"
        )))
        .build()
        .unwrap() //safe to unwrap
}

pub fn git_commit_chain_with_context<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT_WITH_CONTEXT,
            "input",
            "context"
        )))
        .build()
        .unwrap() //safe to unwrap
}

const PROMPT_REVISE_GIT: &str = r#"
Giving this git commit :{{commit}}, modify it to {{to}}.
You sould just return the complete commit message, nothing more.

Example input Commit:
feat(parser):add ability to parse JSON files.

Example To change:
other option

Example anwer:
feat(parser):add new JSON parse function to parse files

"#;
pub fn revise_commit_chain<VALUE: LLM + 'static>(llm: VALUE) -> LLMChain {
    LLMChainBuilder::new()
        .llm(llm)
        .prompt(HumanMessagePromptTemplate::new(template_jinja2!(
            PROMPT_REVISE_GIT,
            "to",
            "commit"
        )))
        .build()
        .unwrap() //safe to unwrap
}
