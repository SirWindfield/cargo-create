use crate::providers::{author::email::EMAIL_KEY, VariableProvider};
use std::env;
use tera::{Context, Value};

const CARGO_EMAIL_ENV_NAME: &str = "CARGO_EMAIL";

/// Detects the global git author.
pub struct CargoEmailVariableProvider;

impl VariableProvider for CargoEmailVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        if !ctx.contains_key(EMAIL_KEY) {
            let email = env::var(CARGO_EMAIL_ENV_NAME);
            if let Ok(email) = email {
                println!("inserted: {}", email);
                ctx.insert(EMAIL_KEY, &email);
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
    &CargoEmailVariableProvider as &dyn VariableProvider
}

#[cfg(test)]
mod tests {
    use super::CargoEmailVariableProvider;
    use crate::providers::{
        author::email::{cargo::CARGO_EMAIL_ENV_NAME, EMAIL_KEY},
        VariableProvider,
    };
    use std::env;
    use tera::{to_value, Context};

    #[test]
    fn test_cargo_email() {
        let email = "test@rust.com";
        env::set_var(CARGO_EMAIL_ENV_NAME, email);

        let provider = CargoEmailVariableProvider;
        let mut ctx = Context::new();
        provider.populate(&mut ctx);

        assert_eq!(ctx.get(EMAIL_KEY).unwrap(), &to_value(email).unwrap());
    }
}
