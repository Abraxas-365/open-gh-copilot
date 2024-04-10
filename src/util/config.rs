use std::{fs::File, io::Write};

use dialoguer::{console::style, theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};

use crate::COPILOT_PATH;

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMConfig {
    pub llm_type: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub api_version: Option<String>,
    pub deployment: Option<String>,
    pub model: Option<String>,
}

impl LLMConfig {
    pub fn new_ollama() -> Self {
        LLMConfig {
            llm_type: "ollama".to_string(),
            api_key: Some("ollama".to_string()),
            api_base: Some("http://localhost:11434/v1".to_string()),
            api_version: None,
            deployment: None,
            model: Some("llama2".to_string()),
        }
    }

    pub fn new_openai() -> Self {
        LLMConfig {
            llm_type: "openai".to_string(),
            api_key: None,
            api_base: None,
            api_version: None,
            deployment: None,
            model: Some("gpt-3.5-turbo".to_string()),
        }
    }

    pub fn new_anthropic() -> Self {
        LLMConfig {
            llm_type: "anthropic".to_string(),
            api_key: None,
            api_base: None,
            api_version: None,
            deployment: None,
            model: Some("claude-3-opus-20240229".to_string()),
        }
    }

    pub async fn new_config() {
        let opciones = vec!["OpenAi", "Ollama", "Anthropic", "Exit"];

        let instrucciones = style("[Use arrows to move, type to filter]")
            .yellow()
            .to_string();
        let prompt = format!("Select an option  {}", instrucciones);

        let seleccion = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt)
            .default(0)
            .items(&opciones[..])
            .interact()
            .unwrap();

        match seleccion {
            0 => {
                let mut config = LLMConfig::new_openai();

                let model_prompt = "Enter the model (press Enter for default 'gpt-3.5-turbo'):";
                let model_input: String = Input::new()
                    .with_prompt(model_prompt)
                    .default("gpt-3.5-turbo".into())
                    .interact_text()
                    .unwrap();
                config.model = Some(model_input);

                let key_prompt =
                    "Enter your OpenAi key (press Enter for use your env $OPENAI_API_KEY):";
                let key: String = Input::new()
                    .with_prompt(key_prompt)
                    .default("None".into())
                    .interact_text()
                    .unwrap();
                if key != "None" {
                    config.api_key = Some(key);
                }

                save_config(&config)
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
            }
            1 => {
                let mut config = LLMConfig::new_ollama();
                let model_prompt = "Enter the model (press Enter for default 'llama2'):";
                let model_input: String = Input::new()
                    .with_prompt(model_prompt)
                    .default("llama2".into())
                    .interact_text()
                    .unwrap();
                config.model = Some(model_input);

                let key_prompt = "Enter your OpenAi key (press Enter for use default 'ollama'):";
                let key: String = Input::new()
                    .with_prompt(key_prompt)
                    .default("ollama".into())
                    .interact_text()
                    .unwrap();
                config.api_key = Some(key);

                let base_prompt =
                    "Enter the base URL (press Enter for default 'http://localhost:11434/v1'):";
                let base: String = Input::new()
                    .with_prompt(base_prompt)
                    .default("http://localhost:11434/v1".into())
                    .interact_text()
                    .unwrap();
                config.api_base = Some(base);

                save_config(&config)
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
            }
            2 => {
                let mut config = LLMConfig::new_anthropic();
                let model_prompt =
                    "Enter the model (press Enter for default 'claude-3-opus-20240229'):";
                let model_input: String = Input::new()
                    .with_prompt(model_prompt)
                    .default("claude-3-opus-20240229".into())
                    .interact_text()
                    .unwrap();
                config.model = Some(model_input);

                let key_prompt =
                    "Enter your Anthropic key (press Enter for use your env $CLOUDE_API_KEY):";
                let key: String = Input::new()
                    .with_prompt(key_prompt)
                    .default("ollama".into())
                    .interact_text()
                    .unwrap();
                config.api_key = Some(key);

                save_config(&config)
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
            }
            3 => std::process::exit(0),
            _ => eprintln!("Invalid option."),
        }
    }
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
