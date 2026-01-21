default:
  @just --list

run:
  cd src-tauri && cargo run

build:
  cd src-tauri && cargo build

release:
  cd src-tauri && cargo build --release
  @echo ""
  @echo "Binary: src-tauri/target/release/insertgif"
