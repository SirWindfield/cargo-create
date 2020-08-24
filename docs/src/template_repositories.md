# Template repositories

`jen` supports template files written for the [Tera](https://github.com/Keats/tera) engine. For an overview about its features, syntax and some shorter examples, please take a look at the [official documentation](https://tera.netlify.app/docs).

## Configuration file

Each template repository should contain a configuration file called `jen.toml` that is used to declare all features that the template repository does provide:

```toml
[[features]]
name = "ci-workflow"
files = [".github/workflows/ci.yml.tera"]

[[features]]
name = "docs-mdbook-workflow"
files = [
    { from = ".github/workflows/docs_mdbook.yml", to = ".github/workflows/docs.yml" }
]

[[features]]
name = "docs-crates-workflow"
files = [
    { from = ".github/workflows/docs_crates.yml", to = ".github/workflows/docs.yml" }
]

[[features]]
name = "workflows"
include = ["ci-workflow", "docs-mdbook-workflow"]
```

Features can include files by adding the filename relative to the root of the repository to the array. Features can be composed together to create so-called `super-features`.

Renaming files is supported to allow multiple features to include the same filename. Renaming is done by using an `inline table` instead of a simple string. See the example above for a simple use-case.

## Template files

By default `jen` does simply copy every single file into the generated repository. If a file ends with `.tera`, `jen` will run the tempolate engine on it, populating that file with all [variables][variables] it knows and remove the `.tera` extension after the population step was successful.

> __Note:__ If a template file does try to access a variable that `jen` does not know about, the CLI will exit with a panic message.

[variables]: ./variables.md
