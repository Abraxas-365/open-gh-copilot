use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use langchain_rust::{
    language_models::{llm::LLM, GenerateResult, LLMError},
    llm::{AzureConfig, Claude, OpenAI},
    schemas::{Message, StreamData},
    tools::OpenAIConfig,
};

#[derive(Clone)]
pub enum LLMVariant {
    OpenAI(OpenAI<OpenAIConfig>),
    AzureOpenAI(OpenAI<AzureConfig>),
    Anthropic(Claude),
}

#[async_trait]
impl LLM for LLMVariant {
    async fn generate(&self, messages: &[Message]) -> Result<GenerateResult, LLMError> {
        match self {
            LLMVariant::OpenAI(llm) => llm.generate(messages).await,
            LLMVariant::Anthropic(llm) => llm.generate(messages).await,
            LLMVariant::AzureOpenAI(llm) => llm.generate(messages).await,
        }
    }

    async fn stream(
        &self,
        _messages: &[Message],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamData, LLMError>> + Send>>, LLMError> {
        match self {
            LLMVariant::OpenAI(llm) => llm.stream(_messages).await,
            LLMVariant::Anthropic(llm) => llm.stream(_messages).await,
            LLMVariant::AzureOpenAI(llm) => llm.stream(_messages).await,
        }
    }
}
