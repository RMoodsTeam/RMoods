#!/bin/sh

cd $(git rev-parse --show-toplevel)
chmod +x .github/hooks/*
cp .github/hooks/* .git/hooks
echo "Hooks installed successfully"

