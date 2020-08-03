//! Contains providers for automatically detecting the user's name.
//!
//! `AuthorVariableProvider` does check the following types (in order):
//! - CARGO_ env variables.
//! - GIT_ env variables followed by git global config file followed by
//!   GIT_COMMITTER
//! - USER env
//! - USERNAME env
//! - NAME env

const AUTHOR_KEY: &str = "author_name";

mod cargo;
mod env;
mod git;
