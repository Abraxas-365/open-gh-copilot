# Open GitHub Copilot CLI Clone

- GitHub Copilot CLI Clone: An innovative CLI tool inspired by GitHub Copilot.
- Enhances developer productivity with code suggestions and automation.
- Open-source and supports a broad range of language models.
- Compatible with:

  - `OpenAI`
  - `Azure OpenAI`
  - `Anthropic's Claude`
  - `Local models through Ollama`

https://github.com/Abraxas-365/open-gh-copilot/assets/63959220/74e2b2db-ca5e-4830-9439-b637cc8d6301

# Installation Steps

1. Clone the Repository: Start by cloning this project to your local machine. Open
   a terminal and run the following git command:

```bash
   git clone https://github.com/Abraxas-365/open-gh-copilot
```

2. Navigate to the Project Directory: Change into the project's directory with cd:

```bash
cd open-gh-copilot
```

3. Compile the Project: Use Cargo, Rust's build system and package manager, to compile the project:

```bash
cargo build --release
```

4. Run the CLI Tool: After compiling, you can start the tool directly through Cargo or by executing the binary directly. To run it through Cargo, use:

```rust
cargo run -- [arguments]
```

Alternatively, you can run the compiled binary found in target/release/:

```bash
./target/release/gh_copilot_rs [arguments]
```

Replace [arguments] with any arguments or flags your tool accepts.
`I recomend to move the binary to you local bin`

# Usage

By default it uses OpenAI Gpt3.5 turbo

```bash
export OPENAI_API_KEY=sk-key
```

if you want to use your own model or config use

```bash
  gh_copilot_rs config
```

The Open GitHub Copilot CLI Clone provides a range of commands designed to assist
developers by suggesting, explaining, and executing commands based on large language
models. Below are the available commands and their descriptions:

## Available Commands

- `explain`: Breaks down and explains what a specific command does. This feature is
  invaluable for learning new commands or understanding more complex operations.

  ```bash
  gh_copilot_rs explain <command>
  ```

  Example:

  ```bash
  gh_copilot_rs explain "git reset --soft HEAD~1\n git rm mistake.md\n git commit -c ORIG_HEAD"
  ```

  Replace <command> with the command you want to explain.

- `suggest`: Provides recommendations for commands based on a provided input query.
  Useful for discovering commands related to specific tasks.

  ```bash
  gh_copilot_rs suggest "how to list all files"
  ```

  You can use `suggest` without arguments, it will open an interactive prompt

  ```bash
  gh_copilot_rs suggest
  ```

- `config`: Launches an interactive configuration wizard that allows you to choose
  and configure your preferred LLM provider (e.g., OpenAI, Anthropic, Ollama). This step is critical to personalize the tool according to your preferences and available services.

  If you dont config, it will use OpenAi gpt3.5 as default

  ```bash
  gh_copilot_rs config
  ```

- `commit`: Creates a automatic commit message of the staged files.
  _`commit` is in testing._

  - Automaitc commit

  ```bash
  gh_copilot_rs commit
  ```

  - Excluding files and context
    - flag `-e` are the excluded files
    - flag `-c` is some context for the commit

  ```bash
  gh_copilot_rs commit -e excluded1.txt excluded2.txt -c "Refactored authentication module"
  ```

Follow the prompts to select your LLM provider and configure additional settings,
such as API keys and model preferences.
Additional Options for Suggestions

After obtaining a suggestion, further actions can be performed, such as copying the
result to your clipboard, executing the recommended command directly, revising the
command, or obtaining a detailed explanation:

- Copy to Clipboard: Quickly copy the suggested command to your clipboard for easy
  pasting into your terminal or script.

- Execute Command: Directly execute the suggested command within your environment—ideal
  for efficiency and streamlining workflows.

- Explain Command: Gain insights into what the suggested command does, enhancing
  understanding and confidence before execution.

- Revise Command: Modify the initial suggestion to better fit your requirements or
  correct any inaccuracies.

# Recomendations

- Move the binary to your local bin folder.
- Rename the bin to an small command like gh-cop
