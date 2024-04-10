use dialoguer::console::{style, StyledObject};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod shared;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub llm: LLMConfig,
}

#[derive(Serialize, Deserialize)]
pub struct LLMConfig {
    pub llm_type: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub api_version: Option<String>,
    pub deployment: Option<String>,
    pub model: Option<String>,
}

pub fn intro() {
    println!(
        "\n{}\n",
        style("Welcome to GitHub Copilot in the CLI!")
            .white()
            .bright()
            .bold()
    );
    println!("{}\n",
    style("I'm powered by AI, so surprises and mistakes are possible. Make sure to verify any g enerated code or suggestions, and share feedback so that we can learn and improve.").white().dim());
}

pub fn apply_styles_to_backticks(text: &str) -> String {
    let re = Regex::new(r"`([^`]*)`").unwrap();
    let mut result_text = text.to_string();

    for cap in re.captures_iter(text) {
        let matched_text: &str = &cap[1];
        let styled: StyledObject<String> = style(matched_text.to_string()).yellow().bold();
        result_text = result_text.replacen(&format!("`{}`", matched_text), &styled.to_string(), 1);
    }

    result_text
}
