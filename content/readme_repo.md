# md.rs
*A web‑based Markdown preview tool, built in Rust*

[![Rust](https://img.shields.io/badge/rust-1.70+-orange)](https://www.rust-lang.org/)

---


## Introduction

**md.rs** is a fast, reactive web‑based Markdown previewer implemented in Rust.  
It’s ideal as a lightweight tool for writing notes, documentation, or experimenting with Markdown.

---

## Features

- 🖋️ **Live preview** — updates instantly as you type  
- **Clean and minimal UI** — designed for focus  
- **Safe rendering** — handles Markdown to HTML in a controlled manner  
- **Rust-powered** — low overhead, high performance  

---

## Getting Started

### Prerequisites

- Rust (version 1.70+ recommended)  
- `cargo` and `rustc`  
- A modern browser  

### Installation

Clone the repository:

```bash
git clone https://github.com/DavidBalishyan/markdown-previewer-rust.git
cd markdown-previewer-rust
```

Then build it:

```bash
cargo build --release
```

You can also use the provided scripts:

```bash
./run.sh # or use `just` if you are familiar with it
```

### Running Locally

To start the application locally:

```bash
cargo run
```

By default, it may start on `localhost:PORT` (check logs or code for the exact port). Then visit it in your browser to begin typing Markdown and seeing the preview.

---

## Contributing

Contributions are very welcome! If you’d like to help:

1. Fork this repository  
2. Create a new branch (`git checkout -b feature/YourFeature`)  
3. Make your changes, add tests / examples  
4. Submit a pull request with a clear description  

Please ensure your code adheres to Rust formatting and style conventions.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
