#!/bin/sh

sudo apt install git

sudo apt install curl

sudo apt install build-essential

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source "$HOME/.cargo/env"


