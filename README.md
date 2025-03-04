# 🚀 Git Commit Generator

<div align="center">
<img alt="DeepSeek AI" src="https://img.shields.io/badge/DeepSeek-AI-%23FF6F61.svg?style=for-the-badge">
<img alt="GitHub" src="https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&amp;logo=github&amp;logoColor=white">
<img alt="Rust" src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&amp;logo=rust&amp;logoColor=white">
<img alt="AI" src="https://img.shields.io/badge/AI-powered-%23FF6F61.svg?style=for-the-badge">

![GitHub](https://img.shields.io/github/license/Emin017/git-commit-generator)
![GitHub workflows](https://img.shields.io/github/actions/workflow/status/Emin017/git-commit-generator/build.yml)
![GitHub issues](https://img.shields.io/github/issues/Emin017/git-commit-generator)
![GitHub pull requests](https://img.shields.io/github/issues-pr/Emin017/git-commit-generator)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/Emin017/git-commit-generator)

![GitHub stars](https://img.shields.io/github/stars/Emin017/git-commit-generator?style=social)
![GitHub forks](https://img.shields.io/github/forks/Emin017/git-commit-generator?style=social)

[![English](https://img.shields.io/badge/English-README-2ea44f?style=for-the-badge)](README.md)
[![中文](https://img.shields.io/badge/中文-介绍-FF6F61?style=for-the-badge)](README_CN.md)

</div>

🤖 Automatically generate high-quality Git commit messages powered by AI

## ✨ Features

- 🔍 Automatically analyze staged Git changes

- 📝 Generate semantic commit messages using DeepSeek AI

- 🎨 Support for multiple commit message formats (Conventional, Bracketed, etc.)

- ⚙️ Configurable AI model parameters

- 🚀 Simple and easy-to-use command line interface

## 📋 Prerequisites

- 💻 Rust and Cargo installed

- 🔑 DeepSeek API key

- 📦 Git

### 🛠️ Installation

#### From source

```shell
# Clone the repository
git clone https://github.com/Emin017/git-commit-generator.git
# Change to the project directory
cd git-commit-generator
# Build the binary
cargo build --release
# Install the binary
cargo install --path .
# Run the binary
git-commit-generator --help
```

#### Using Nix

```shell
nix run github:Emin017/git-commit-generator#git-commit-generator
```

## ⚙️ Configure DeepSeek API Key

Create a .env file in your project directory:
```
DEEPSEEK_API_KEY=your_api_key_here
```

## 🚀 Usage

```shell
# Add files to commit
git add .

# Generate commit message
git-commit-generator

# Use different commit format
git-commit-generator --format conventional

# Use different model
git-commit-generator --model deepseek-coder
```

### 💡 Available Options

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| --format | -f | Commit message format | conventional |
| --model | -m | DeepSeek model | deepseek-chat |

### 📝 Commit Message Formats

Supported formats include:

- Conventional: `feat: add new feature`

- Bracketed: `[feat]: add new feature`

- Plain: `add new feature`

### 🔄 Workflow

```mermaid
graph TD
    A[Start] --> B[Read Configuration]
    B --> C[Get Staged Changes]
    C --> D{Any Changes?}
    D -->|Yes| E[Call DeepSeek API]
    D -->|No| F[Show Error]
    E --> G[Generate Commit Message]
    G --> H[End]
    F --> H
```

## 🛡️ License


## 🙏 Acknowledgments

Thanks to DeepSeek AI for providing powerful AI models and the Rust community for excellent development tools and libraries!

## 🤝 Contributions
Contributions are welcome! Feel free to submit [Issues](https://github.com/Emin017/git-commit-generator/issues) and [Pull Requests](https://github.com/Emin017/git-commit-generator/pulls)!
