#!/bin/bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Install unzip
sudo dnf install -y unzip gcc cmake

# Unzip the file to the home directory
mkdir $HOME/project
unzip /tmp/deploy.zip -d $HOME/project

# Change directory to the project and build the Rust project
cd $HOME/project
cargo build --release

# Run the Rust project
screen -S rust_project -d -m ./target/release/boa-checker-snek