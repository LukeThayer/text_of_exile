# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

tui_arpg is a Rust-based Terminal User Interface Action Role Playing Game.

## Development Environment

This project uses Nix Flakes for reproducible development. Enter the development shell before running commands:

```bash
nix develop
```

This provides: Rust stable toolchain, rust-analyzer, cargo-watch, cargo-edit, and git.

## Common Commands

```bash
cargo build          # Build the project
cargo run            # Run the application
cargo test           # Run all tests
cargo test <name>    # Run a specific test
cargo clippy         # Run lints
cargo fmt            # Format code
cargo check          # Quick compilation check
cargo watch -x run   # Auto-rebuild and run on file changes
```

## Architecture

*Note: This section should be updated as the codebase develops.*

The project is in early stages. The `config/` directory is reserved for game configuration files.
