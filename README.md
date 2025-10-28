
# Termux Package Inspector (`tpi`)

A **TUI (Text User Interface)** application for browsing and inspecting installed packages across **multiple package managers** in **Termux**:

- `pkg` (Termux native)
- `apt` (Debian packages)
- `pip` (Python packages)

Built with **Rust**, **ratatui**, and **crossterm** — lightweight, fast, and fully interactive.

---

## Features

- **Unified view** of packages from `pkg`, `apt`, and `pip`
- **Live switching** between package managers with `Tab`
- **Scrollable package details** (`J`/`K`)
- **Resizable detail pane** (`+`/`-`)
- **Vim-style navigation** (`j/k`, `g/G`, `Home/End`)
- **Clean, responsive TUI** with syntax-aware parsing

---

## Screenshot

*(Coming soon — run it to see!)*

---

## Installation

### Prerequisites

- [Termux](https://termux.dev/) (Android)
- `rust` and `cargo` installed in Termux:

```bash
pkg install rust -y
```

### Build & Install

```bash
git clone https://github.com/yourusername/termux-pkg-inspector.git
cd termux-pkg-inspector
cargo build --release
cp target/release/tpi ~/.local/bin/
```

> Ensure `~/.local/bin` is in your `$PATH`:
>
> ```bash
> echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
> source ~/.bashrc
> ```

---

## Usage

```bash
tpi
```

### Key Bindings

| Key | Action |
|-----|--------|
| `q` or `Esc` | Quit |
| `j` / `↓` | Next package |
| `k` / `↑` | Previous package |
| `g` / `Home` | Jump to first |
| `G` / `End` | Jump to last |
| `Tab` | Switch package manager (`pkg` → `apt` → `pip` → ...) |
| `J` | Scroll details **down** |
| `K` | Scroll details **up** |
| `+` | Increase details pane (max 80%) |
| `-` | Decrease details pane (min 10%) |

---

## Supported Package Managers

| Manager | Command Used | Notes |
|--------|--------------|-------|
| `pkg` | `pkg list-installed` | Native Termux |
| `apt` | `apt list --installed` | Debian/dpkg |
| `pip` | `pip list` | Python packages |

> Details fetched via:
> - `pkg show <name>`
> - `apt show <name>`
> - `pip show <name>`

---

## Project Structure

```
termux-pkg-inspector/
├── Cargo.toml
├── src/
│   └── main.rs         # Core TUI logic
└── README.md
```

---

## Development

```bash
# Run in dev mode
cargo run

# Build optimized binary
cargo build --release
```

---

## Contributing

Pull requests are welcome! Feel free to:

- Improve output parsing
- Add support for `dpkg`, `npm`, `gem`, etc.
- Enhance UI/UX
- Add search/filtering
- Add local caching for speed, to reduce description calls.

---

## License

[MIT License](LICENSE) — Free to use, modify, and distribute.

---

## Motivation

Made with ❤️ for the **Termux** community.

---

