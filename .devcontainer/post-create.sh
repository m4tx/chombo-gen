#!/bin/bash

rustup target add wasm32-unknown-unknown
cargo install trunk
pre-commit install
