# 📜 script-list (sl)

A fast, modern Rust CLI tool to list and display npm scripts from `package.json`.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🚀 **Fast** - Written in Rust for maximum performance
- 🎨 **Beautiful output** - Colorized table, list, or JSON formats
- 🔍 **Filter support** - Search scripts by name
- 📦 **Zero dependencies** - Single binary, no runtime requirements
- 🖥️ **Cross-platform** - Works on macOS, Linux, and Windows
- ⚡ **Short command** - Just type `sl`

## 📦 Installation

### From Source (Cargo)

```bash
cargo install --git https://github.com/yfeelib/script-list
```

### From Release

Download pre-built binaries from [Releases](https://github.com/yfeelib/script-list/releases).

## 🚀 Usage

### Basic

```bash
# List all scripts in current directory
sl

# Output:
# 📦 my-project
# My awesome project
#
# Script       Command
# ────────────────────────────────────────────────────
# build        cargo build --release
# test         cargo test
# dev          npm run dev
#
# ℹ️  Found 3 script(s)
```

### Filter Scripts

```bash
# Show only scripts containing "test"
sl --filter test

# Or
sl -f build
```

### Different Formats

```bash
# Table format (default)
sl --format table

# Simple list
sl --format list

# JSON output
sl --format json
```

### Other Options

```bash
# Show only script names
sl --names-only

# Use custom package.json path
sl --path ./path/to/package.json
```

## 🛠️ Development

```bash
# Clone
git clone https://github.com/yfeelib/script-list
cd script-list

# Build
cargo build --release

# Run
cargo run

# Test
cargo test
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Credits

Inspired by the [npm package `script-list`](https://www.npmjs.com/package/script-list).
