use crate::providers::{author::name::AUTHOR_KEY, VariableProvider};
use std::env;
use tera::Context;

/// Detects the global git author.
pub struct CargoAuthorVariableProvider;

impl VariableProvider for CargoAuthorVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        if !ctx.contains_key(AUTHOR_KEY) {
            let author = env::var("CARGO_NAME");
            if let Ok(author) = author {
                ctx.insert(AUTHOR_KEY, &author);
                return true;
            }
        }

        false
    }

    fn priority(&self) -> u8 {
        0
    }
}

inventory::submit! {
    &CargoAuthorVariableProvider as &dyn VariableProvider
}
