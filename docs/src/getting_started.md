# Getting Started

## Installation

To install `jen`, please run the following command:

```text
cargo install cargo-create --locked
```

## Usage

After installation, the CLI is provided under two names: `jen` and as a `cargo` subcommand called `cargo-generate`.

```text
jen -h
```

The most common way to use `jen` is by simply cloning a template repository for a new project:

```text
jen -g SirWindfield/zerotask-rust-bin-template -b template -f workflows -n some_project
```

The above command will clone the repository, enable the `workflows` feature (which does add GitHub workflow files to the generated project) and populate all templates.
