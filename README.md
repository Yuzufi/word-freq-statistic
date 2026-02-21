# Word Frequency Statistic Tool 📊

![Word Frequency Statistic](https://img.shields.io/badge/Download-Release-blue?style=flat&logo=github)

Welcome to the **Word Frequency Statistic** repository! This high-performance tool is designed for efficient Chinese corpus word frequency analysis. With this application, you can process up to 1 billion characters in under a minute, focusing on two-character words. 

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [Support](#support)

## Introduction

Analyzing word frequency in large text corpora is crucial for various natural language processing (NLP) tasks. This tool streamlines the process of counting two-character words in Chinese texts. Built with Rust, it ensures high performance and efficiency, making it suitable for both researchers and developers.

## Features

- **High Performance**: Processes 1 billion characters in less than a minute.
- **Command-Line Interface**: Easy to use from the terminal.
- **Support for Chinese NLP**: Specifically designed for Chinese text analysis.
- **Statistics Generation**: Provides detailed statistics on word frequency.
- **Word Cloud Generation**: Visualize word frequency data in an engaging format.
- **Lightweight**: Minimal resource usage while maintaining speed.

## Installation

To install the Word Frequency Statistic tool, you can download the latest release from the [Releases section](https://github.com/Yuzufi/word-freq-statistic/releases). 

1. Go to the Releases section.
2. Download the appropriate binary for your operating system.
3. Follow the instructions to execute the binary.

### System Requirements

- Rust installed (if you wish to build from source).
- A modern operating system (Windows, macOS, or Linux).

## Usage

Once you have installed the tool, you can start using it from the command line. Here’s a basic command structure:

```bash
word-freq-statistic <input_file> <output_file>
```

- `<input_file>`: The path to your Chinese text file.
- `<output_file>`: The path where you want to save the output statistics.

### Command-Line Options

- `--help`: Display help information.
- `--version`: Show the version of the tool.

## Examples

Here are some examples to help you get started:

### Example 1: Basic Usage

```bash
word-freq-statistic my_corpus.txt output_stats.txt
```

This command processes the `my_corpus.txt` file and saves the statistics to `output_stats.txt`.

### Example 2: Help Command

```bash
word-freq-statistic --help
```

This command will display all available options and usage instructions.

## Contributing

We welcome contributions to improve this tool. If you have ideas, bug fixes, or enhancements, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Make your changes.
4. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Support

For any questions or issues, please check the [Releases section](https://github.com/Yuzufi/word-freq-statistic/releases) for the latest updates. You can also open an issue in the repository for assistance.

## Acknowledgments

We appreciate the contributions of the open-source community and the support of users who help improve this tool. Your feedback is invaluable in enhancing its capabilities.

---

Thank you for checking out the Word Frequency Statistic tool! We hope it serves your needs well in analyzing Chinese text.