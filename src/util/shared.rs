use std::{fs::File, io::Write, pin::Pin};

use async_trait::async_trait;
use futures::Stream;
use langchain_rust::{
    language_models::{llm::LLM, GenerateResult, LLMError},
    llm::{Claude, OpenAI},
    schemas::{Message, StreamData},
    tools::OpenAIConfig,
};

use crate::COPILOT_PATH;

use super::{AppConfig, LLMConfig};

#[derive(Clone)]
pub enum LLMVariant {
    OpenAI(OpenAI<OpenAIConfig>),
    Anthropic(Claude),
}

#[async_trait]
impl LLM for LLMVariant {
    async fn generate(&self, messages: &[Message]) -> Result<GenerateResult, LLMError> {
        match self {
            LLMVariant::OpenAI(llm) => llm.generate(messages).await,
            LLMVariant::Anthropic(llm) => llm.generate(messages).await,
        }
    }

    async fn stream(
        &self,
        _messages: &[Message],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamData, LLMError>> + Send>>, LLMError> {
        match self {
            LLMVariant::OpenAI(llm) => llm.stream(_messages).await,
            LLMVariant::Anthropic(llm) => llm.stream(_messages).await,
        }
    }
}

pub struct SharedState {
    llm: LLMVariant,
    os: String,
}

impl SharedState {
    pub fn new(llm: LLMVariant, os: &str) -> Self {
        SharedState {
            llm,
            os: os.to_string(),
        }
    }

    pub fn llm(&self) -> LLMVariant {
        self.llm.clone()
    }

    pub fn os(&self) -> &str {
        &self.os
    }
}

impl Default for SharedState {
    fn default() -> Self {
        let os = std::env::consts::OS;
        match read_config() {
            Some(cfg) => {
                match cfg.llm.llm_type.as_str() {
                    "openai" => {
                        println!("AAAAAAAAAAAAAAAAA");
                        let mut llm = OpenAI::default().with_config(OpenAIConfig::default());
                        let mut config = OpenAIConfig::default();
                        if let Some(api_key) = cfg.llm.api_key {
                            config = config.with_api_key(api_key);
                        }
                        if let Some(api_base) = cfg.llm.api_base {
                            llm = llm.with_config(config.with_api_base(api_base));
                        }
                        if let Some(model) = cfg.llm.model {
                            llm = llm.with_model(&model);
                        }
                        return SharedState::new(LLMVariant::OpenAI(llm), os);
                    }
                    "ollama" => {
                        println!("AAAAAAAAAAAAAAAAA");
                        let mut llm = OpenAI::default();
                        let mut config = OpenAIConfig::default();
                        config =
                            config.with_api_key(cfg.llm.api_key.unwrap_or("ollama".to_string()));
                        config = config.with_api_base(
                            cfg.llm
                                .api_base
                                .unwrap_or("http://localhost:11434/v1".to_string()),
                        );
                        llm = llm.with_model(&cfg.llm.model.unwrap_or("llama2".to_string()));
                        llm = llm.with_config(config);

                        return SharedState::new(LLMVariant::OpenAI(llm), os);
                    }

                    "anthropic" => {
                        let mut llm = Claude::default();
                        if let Some(api_key) = cfg.llm.api_key {
                            llm = llm.with_api_key(api_key);
                        }
                        if let Some(model) = cfg.llm.model {
                            llm = llm.with_model(&model);
                        }
                        return SharedState::new(LLMVariant::Anthropic(llm), os);
                    }

                    // Handle other LLM types here
                    _ => panic!("Unsupported LLM type"),
                }
            }
            None => SharedState {
                llm: LLMVariant::OpenAI(OpenAI::default()),
                os: os.to_string(),
            },
        }
    }
}

fn read_config() -> Option<AppConfig> {
    let mut config_path = home::home_dir()?;
    config_path.push(COPILOT_PATH);

    let config_contents = std::fs::read_to_string(config_path).ok()?;
    let config: Option<LLMConfig> = toml::from_str(&config_contents)
        .map_err(|e| {
            println!("Error reading config: {}", e);
        })
        .ok();
    if config.is_none() {
        return None;
    }
    Some(AppConfig {
        llm: config.unwrap(),
    })
}

pub fn save_config(config: &LLMConfig) -> std::io::Result<()> {
    let toml_string = toml::to_string_pretty(&config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let mut config_path = home::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;
    config_path.push(COPILOT_PATH);

    let mut file = File::create(config_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
