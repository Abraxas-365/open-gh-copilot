use langchain_rust::language_models::llm::LLM;

pub struct SharedState<LLMType: LLM + Clone> {
    llm: LLMType,
    os: String,
}

impl<LLMType: LLM + Clone> SharedState<LLMType> {
    pub fn new(llm: LLMType, os: &str) -> Self {
        SharedState {
            llm,
            os: os.to_string(),
        }
    }

    pub fn llm(&self) -> LLMType {
        self.llm.clone()
    }

    pub fn os(&self) -> &str {
        &self.os
    }
}
