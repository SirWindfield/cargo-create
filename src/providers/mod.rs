use crate::config::user::UserConfig;
use serde::Serialize;
use tera::Context;

mod author;
mod misc;

pub trait Registry {
    fn contains(&self, key: &str) -> bool;
    fn get<T>(&self, key: &str) -> Option<T>;
    fn insert<T: Serialize>(&mut self, key: &str, value: T);
}

/// Provides variables that can be inserted into templates.
pub trait VariableProvider {
    /// Populates the context with the variables.
    ///
    /// Implementations should check if the value is already been inserted if
    /// they are part of set of variable providers (like `author_name`).
    fn populate(&self, ctx: &mut Context) -> bool;
    /// Returns the priority that the provider has.
    ///
    /// The lower the priority is, the earlier the provider will be called.
    fn priority(&self) -> u8;
}

pub trait ConfigurableVariableProvider {
    /// Populates the context with the variables.
    ///
    /// Implementations should check if the value is already been inserted if
    /// they are part of set of variable providers (like `author_name`).
    fn populate(&self, ctx: &mut Context, user_config: &UserConfig) -> bool;
}

inventory::collect!(&'static dyn ConfigurableVariableProvider);
inventory::collect!(&'static dyn VariableProvider);
