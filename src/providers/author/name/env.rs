use crate::providers::VariableProvider;
use std::env;
use tera::Context;

/// Detects the global git author.
pub struct EnvAuthorVariableProvider;

impl VariableProvider for EnvAuthorVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        let author = env::var("GIT_AUTHOR_NAME").or_else(|_| env::var("GIT_COMMITTER_NAME"));

        match author {
            Ok(author) => {
                ctx.insert("author", &author);
                true
            }
            Err(_) => false,
        }
    }

    fn priority(&self) -> u8 {
        20
    }
}

inventory::submit! {
    &EnvAuthorVariableProvider as &dyn VariableProvider
}
