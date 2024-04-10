# Open GitHub Copilot CLI Clone

Open GitHub Copilot CLI Clone is an innovative and versatile command-line interface
(CLI) tool, inspired by the functionality of GitHub Copilot. Designed to enhance
developer productivity through code suggestion and automation, this open-source project
goes above and beyond by supporting a wide array of language models. Whether you're
coding with the latest from OpenAI, benefiting from the intelligence of Anthropic's
Claude, or utilizing local models via Ollama, this CLI tool adapts to your preferred
coding assistant seamlessly.

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

- config: Launches an interactive configuration wizard that allows you to choose
  and configure your preferred LLM provider (e.g., OpenAI, Anthropic, Ollama). This step is critical to personalize the tool according to your preferences and available services.

  If you dont config, it will use OpenAi gpt3.5 as default

  ```bash
  gh_copilot_rs config
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
