use std::{
    io::{self, BufRead},
    process::{Command, Stdio},
};

use dialoguer::console::style;
use langchain_rust::{chain::Chain, prompt_args};

use crate::{
    chains::{git_commit_chain, git_commit_chain_with_context},
    cli::choose_options_for_commit,
    util::shared::SharedState,
};

pub async fn git_commit_command(
    shared_state: &SharedState,
    context: Option<&str>,
    exclude: Option<&[&str]>,
) {
    let input = execute_git_diff_command(exclude.unwrap_or(&[]))
        .map_err(|e| {
            eprintln!("Error: {}", e);
        })
        .unwrap();
    let suggestion = match context {
        Some(context) => git_commit_chain_with_context(shared_state.llm())
            .invoke(prompt_args! {
                "input"=>input,
                "context"=>context
            })
            .await
            .unwrap(),
        None => git_commit_chain(shared_state.llm())
            .invoke(prompt_args! {
                "input"=>input
            })
            .await
            .unwrap(),
    };

    println!("Sugestion:\n");
    println!("{}\n", style(suggestion.clone()).yellow().bold());
    choose_options_for_commit(shared_state, &suggestion).await;
}

fn execute_git_diff_command(excludes: &[&str]) -> io::Result<String> {
    let exclude_pattern = excludes.iter().fold(String::new(), |acc, &file| {
        if acc.is_empty() {
            format!("grep -vE '^{}$'", file)
        } else {
            format!("{} | grep -vE '^{}$'", acc, file)
        }
    });

    let git_root_command_output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    let git_root_path = String::from_utf8_lossy(&git_root_command_output.stdout)
        .trim()
        .to_string();

    let shell_command = if exclude_pattern.is_empty() {
        format!("cd \"{}\" && git diff --cached --name-only --diff-filter=ACM | while read -r file; do echo \"\\n---------------------------\\n name:$file\"; git diff --cached \"$file\" | sed 's/^/+/'; done", git_root_path)
    } else {
        format!(
            "cd \"{}\" && git diff --cached --name-only --diff-filter=ACM | {} | while read -r file; do echo \"\\n---------------------------\\n name:$file\"; git diff --cached \"$file\" | sed 's/^/+/'; done",
            git_root_path, exclude_pattern
        )
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(&shell_command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not capture stdout."))?;

    let reader = io::BufReader::new(output);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    Ok(lines.join("\n"))
}
