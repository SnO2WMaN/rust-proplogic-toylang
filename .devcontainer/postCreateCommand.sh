#!/usr/bin/env bash

# Enable Nix flake
echo "extra-experimental-features = nix-command flakes" | sudo tee /etc/nix/nix.conf >/dev/null

# Allow direnv
direnv allow