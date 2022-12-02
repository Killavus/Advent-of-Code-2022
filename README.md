# Advent of Code 2022

My solutions to problems in 2022 edition of [Advent of Code](https://adventofcode.com/2022), written in Rust.

## How to use this repo:

The best way to see how solution is created is to refer to git history - either by navigating on GitHub, or using `git log` command and doing `git checkout <commit-id>` to see a historic version of code. You can always go back to the most up-to-date version of this repository by issuing `git checkout main` command.

## How to run examples:

You can either go to the problem folder and use `cargo run --release`:

```
cd 2-rock-paper-scissors/
cargo run --release
```

Or from main workspace folder:

```
# Notice you need to drop numbering, so rock-paper-scissors, NOT 2-rock-paper-scissors!
cargo run --bin rock-paper-scissors --release
```

All solutions accept input as standard input, so you need to pipe it:

```
cargo run --release < input
cargo run --bin rock-paper-scissors --release < 2-rock-paper-scissors/input

# Or in PowerShell:

Get-Content ./input | cargo run --release
Get-Content ./2-rock-paper-scissors/input | cargo run --bin rock-paper-scissors --release
```
