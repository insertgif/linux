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

install-desktop-app:
  just release
  cp insertgif-template.desktop ~/.local/share/applications/insertgif.desktop
  @echo "Icon={{justfile_directory()}}/src-tauri/icons/icon.png" >> ~/.local/share/applications/insertgif.desktop
  @echo "Exec={{justfile_directory()}}/src-tauri/target/release/insertgif" >> ~/.local/share/applications/insertgif.desktop
  -update-desktop-database ~/.local/share/applications/
  @echo ""
  @echo "Desktop app installed"
  @echo "Binary: {{justfile_directory()}}/src-tauri/target/release/insertgif"
