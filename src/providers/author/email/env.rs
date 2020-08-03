use crate::providers::VariableProvider;
use std::env;
use tera::{Context, Value};
use crate::providers::author::email::EMAIL_KEY;

const EMAIL_ENV_NAME: &str = "EMAIL";

/// Detects the global git author.
pub struct EnvEmailVariableProvider;

impl VariableProvider for EnvEmailVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        if !ctx.contains_key(EMAIL_KEY) {
            let author = env::var(EMAIL_ENV_NAME);
            if let Ok(author) = author {
                ctx.insert(EMAIL_KEY, &author);
                return true;
            }
        }

        false
    }

    fn priority(&self) -> u8 {
        20
    }
}

inventory::submit! {
    &EnvEmailVariableProvider as &dyn VariableProvider
}

#[cfg(test)]
mod tests {
    use std::env;
    use tera::{to_value, Context};
    use crate::providers::author::email::env::{EMAIL_ENV_NAME, EnvEmailVariableProvider};
    use crate::providers::author::email::EMAIL_KEY;
    use crate::providers::VariableProvider;

    #[test]
    fn test_git_committer_email() {
        let email = "test@rust.com";
        env::set_var(EMAIL_ENV_NAME, email);

        let provider = EnvEmailVariableProvider;
        let mut ctx = Context::new();
        provider.populate(&mut ctx);

        assert_eq!(ctx.get(EMAIL_KEY).unwrap(), &to_value(email).unwrap());
    }
}
