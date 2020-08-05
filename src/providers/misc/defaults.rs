// //! The default provider overrides every variable that has already been
// populated.

use crate::{config::user::UserConfig, providers::ConfigurableVariableProvider};
use tera::Context;

pub struct DefaultsVariableProvider;

impl ConfigurableVariableProvider for DefaultsVariableProvider {
    fn populate(&self, ctx: &mut Context, user_config: &UserConfig) -> bool {
        if let Some(values) = &user_config.defaults {
            for (&key, &value) in values {
                ctx.insert(key, value);
            }
        }

        true
    }
}

inventory::submit! {
    &DefaultsVariableProvider as &dyn ConfigurableVariableProvider
}
