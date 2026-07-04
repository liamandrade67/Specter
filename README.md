# Specter

A terminal-based cybersecurity toolkit, built from scratch in Rust as a learning project.

Specter is a full-screen TUI (terminal user interface) that organizes a growing set of recon, web, crypto, and utility tools behind a clean dashboard — navigable entirely with the keyboard, no mouse required.

> **Status:** early development (v0.1). Most tools are placeholders right now — see [Roadmap](#roadmap) for what's planned and what's already working.

---

## What it looks like

```
┌────────────────────────────┐
│ Specter v0.1                │
├────────────────────────────┤
│ > Recon                     │
│   Web                       │
│   Crypto                    │
│   Utilities                 │
├────────────────────────────┤
│ ↑↓ Navigate                 │
│ Enter Select                │
│ q Quit                      │
└────────────────────────────┘
```

## Why

This project exists as a hands-on way to learn Rust — ownership, modules, structs, enums, state machines, threading, and eventually async — by building something real rather than working through isolated exercises. The architecture is intentionally incremental: the navigation framework was built and proven solid *before* any actual tool logic was added, so that new tools can be dropped in as self-contained modules without touching the core app.

## Features

### Working
- Full-screen terminal UI ([ratatui](https://github.com/ratatui-org/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm))
- Keyboard navigation (arrow keys, Enter, Esc, `q` to quit)
- Dashboard → Category → Tool navigation flow
- Modular tool structure (Recon, Web, Crypto, Utilities)

### Planned
- Base64 encode/decode, hash generation
- Persistent notes and settings
- DNS lookup, banner grabbing
- Multi-threaded TCP port scanner
- HTTP header inspection, directory enumeration

See the full [Roadmap](#roadmap) below for the phase-by-phase plan.

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) (stable toolchain).

```bash
git clone https://github.com/liamandrade67/specter.git
cd specter
cargo run
```

## Usage

| Key       | Action                  |
|-----------|-------------------------|
| `↑` / `↓` | Navigate list           |
| `Enter`   | Select / drill in       |
| `Esc`     | Go back                 |
| `q`       | Quit                    |

## Project structure

```
specter
├── Cargo.toml
└── src
    ├── main.rs       # entry point, terminal setup/teardown
    ├── app.rs        # application state + state machine
    ├── ui.rs         # rendering
    ├── events.rs     # keyboard input handling
    ├── screens.rs    # Screen enum (Dashboard / Category / Tool)
    └── modules
        ├── mod.rs        # category list + tool lookup
        ├── recon.rs
        ├── web.rs
        ├── crypto.rs
        └── utilities.rs
```

Each category is its own module exposing a `tools()` function. Adding a new tool means extending a module — not modifying the navigation core.

## Roadmap

Specter is being built in phases, each one a tagged release:

| Version | Focus                                      |
|---------|---------------------------------------------|
| v0.1    | TUI skeleton, navigation, placeholders ✅   |
| v0.2    | Real logic for Base64 / Hash / Notes tools  |
| v0.3    | Persistence (notes & settings saved to disk)|
| v0.4    | First networked tool (DNS lookup)           |
| v0.5    | Threading — non-blocking network calls      |
| v0.6    | Multi-threaded port scanner                 |
| v0.7    | Web recon tools (headers, dir enumeration)  |
| v0.8    | Polish pass — errors, tests, UI consistency |
| v0.9    | Async migration (tokio)                     |
| v1.0    | First public-ready release                  |

## ⚠️ Responsible use

Specter includes (and will include more) tools capable of scanning and probing network hosts. These are intended strictly for use against **systems you own or are explicitly authorized to test** — your own machine, your own local network, or a deliberately vulnerable lab environment (e.g. Metasploitable). Scanning or probing systems without authorization may violate computer-fraud laws in most jurisdictions, regardless of intent.

## License

All rights reserved. This code is provided for viewing purposes only — no permission is granted to use, copy, modify, or distribute it without explicit permission from the author.

## Author

Built by [liamandrade67](https://github.com/liamandrade67) as a Rust learning project.
