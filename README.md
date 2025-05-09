# Trowel

> [!caution]
> This app is experimental and untested. Do not trust its output!

![animated demo of trowel](demo.gif)

Trowel is a TUI wrapper for OpenTofu that lets you inspect plans interactively.

## How Does It Work?

When you run `trowel` without any arguments, it creates a tempfile and runs `tofu plan -out=<tempfile-path>`, writing a binary plan to that file. Then it runs `tofu show -json <tempfile-path>`, captures the JSON output, parses it, and presents it in a TUI interface.

> [!warning]
> Please note that unless you use the `--plan-file` argument to supply an existing plan file, `trowel` will write potentially sensitive data to `/tmp`.

## Installation

Currently, `trowel` is packaged via a Nix Flake (see `flake.nix`) and via Cargo. It is not yet hosted on any external package repositories.
