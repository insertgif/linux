# insertgif

Simple Linux GUI for searching and copying GIF URLs from Tenor.

## Features

- Search GIFs via Tenor API
- Navigate with arrow keys or Tab
- Copy URL, Markdown, or HTML to clipboard
- Keyboard shortcuts:
  - `Enter` - Search
  - `←/→` or `Tab/Shift+Tab` - Navigate
  - `Shift+Enter` - Copy URL
  - `Esc` - Close window

## Build

Requires Rust and system dependencies:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies (Debian/Ubuntu)
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Build
cd ~/Documents/code/insertgif/src-tauri
cargo build --release

# Binary location
./target/release/insertgif
```

## Dev Mode

```bash
cd ~/Documents/code/insertgif/src-tauri
cargo run
```

## Icon

Place a `icon.png` in `src-tauri/icons/` (recommended 512x512px).
