use crate::providers::{author::name::AUTHOR_KEY, VariableProvider};
use git2::Config;
use std::env;
use tera::Context;

/// Detects the global git author.
pub struct GitAuthorVariableProvider;

impl VariableProvider for GitAuthorVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        if !ctx.contains_key(AUTHOR_KEY) {
            let author = env::var("GIT_AUTHOR_NAME")
                .or_else(|_| env::var("GIT_COMMITTER_NAME"))
                .or_else(|_| {
                    let git_config = Config::open_default();
                    match git_config {
                        Ok(git_config) => git_config.get_string("user.name"),
                        Err(e) => Err(e),
                    }
                });
            if let Ok(author) = author {
                ctx.insert(AUTHOR_KEY, &author);
                return true;
            }
        }

        false
    }

    fn priority(&self) -> u8 {
        10
    }
}

inventory::submit! {
    &GitAuthorVariableProvider as &dyn VariableProvider
}
