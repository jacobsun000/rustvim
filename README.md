# RustVim

A lightweight and efficient Vim-like text editor implemented in Rust. This project aims to replicate the core features of Vim while leveraging the safety and performance of Rust.

## Features

- **Modal Editing**: Supports Normal, Insert, and Visual modes.
- **Basic Text Manipulation**: Commands for navigation, deletion, insertion, and undo.
- **Syntax Highlighting**: Extensible syntax support for various languages.
- **Search Functionality**: Incremental search and pattern matching.
- **Key Mappings**: Customizable keybindings for user preferences.
- **Lightweight**: Minimal dependencies and optimized for speed.

## Installation

### Prerequisites
- Rust (version 1.70 or later)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/jacobsun000/rustvim.git
   cd rustvim
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the binary:
   ```bash
   ./target/release/rustvim
   ```

## Usage

### Basic Commands
- **Navigation**:
  - `h`: Move left
  - `j`: Move down
  - `k`: Move up
  - `l`: Move right
- **Insert Mode**:
  - `i`: Enter insert mode
  - `<Esc>`: Exit to normal mode
- **Search**:
  - `/pattern`: Search forward
  - `?pattern`: Search backward

### Customization
Edit the `config.toml` file in the project directory to customize settings like key mappings, themes, and plugins.

## Contributing

Contributions are welcome! If you have ideas for new features or improvements, feel free to open an issue or submit a pull request.

### Development Setup
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rustvim.git
   ```
3. Run the project in development mode:
   ```bash
   cargo run
   ```

## Roadmap

- [ ] Add support for more text manipulation commands
- [ ] Implement plugins and extensions
- [ ] Improve syntax highlighting for additional languages
- [ ] Add multi-window support

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original Vim editor by Bram Moolenaar.
- Special thanks to the Rust community for their support and contributions.

