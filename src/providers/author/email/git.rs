use crate::providers::{VariableProvider};
use std::env;
use tera::{Context, Value};
use crate::providers::author::email::EMAIL_KEY;
use git2::Config;

const GIT_AUTHOR_EMAIL_ENV_NAME: &str = "GIT_AUTHOR_EMAIL";
const GIT_COMMITTER_EMAIL_ENV_NAME: &str = "GIT_COMMITTER_EMAIL";

/// Detects the global git author.
pub struct GitEmailVariableProvider;

impl VariableProvider for GitEmailVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        if !ctx.contains_key(EMAIL_KEY) {
            let author = env::var(GIT_AUTHOR_EMAIL_ENV_NAME)
                .or_else(|_| env::var(GIT_COMMITTER_EMAIL_ENV_NAME)).or_else(|_| {
                let git_config = Config::open_default();
                match git_config {
                    Ok(git_config) => {
                        let username = git_config.get_string("user.email");
                        return username;
                    },
                    Err(e) => Err(e)
                }
            });
            match author {
                Ok(email) => {
                    ctx.insert(EMAIL_KEY, &email);
                    return true;
                },
                Err(e) => eprintln!("{}", e)
            }
        }

        false
    }

    fn priority(&self) -> u8 {
        10
    }
}

inventory::submit! {
    &GitEmailVariableProvider as &dyn VariableProvider
}

#[cfg(test)]
mod tests {
    use std::env;
    use tera::{to_value, Context};
    use crate::providers::author::email::git::{GIT_AUTHOR_EMAIL_ENV_NAME, GitEmailVariableProvider, GIT_COMMITTER_EMAIL_ENV_NAME};
    use crate::providers::author::email::EMAIL_KEY;
    use crate::providers::VariableProvider;

    #[test]
    fn test_git_author_email() {
        let email = "test@rust.com";
        env::set_var(GIT_AUTHOR_EMAIL_ENV_NAME, email);

        let provider = GitEmailVariableProvider;
        let mut ctx = Context::new();
        provider.populate(&mut ctx);

        assert_eq!(ctx.get(EMAIL_KEY).unwrap(), &to_value(email).unwrap());
    }

    #[test]
    fn test_git_committer_email() {
        let email = "test@rust.com";
        env::set_var(GIT_COMMITTER_EMAIL_ENV_NAME, email);

        let provider = GitEmailVariableProvider;
        let mut ctx = Context::new();
        provider.populate(&mut ctx);

        assert_eq!(ctx.get(EMAIL_KEY).unwrap(), &to_value(email).unwrap());
    }
}
