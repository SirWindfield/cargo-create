use crate::providers::VariableProvider;
use chrono::Local;
use tera::Context;

const CURRENT_TIME_KEY: &str = "current_time";

pub struct DatetimeVariableProvider;
impl VariableProvider for DatetimeVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        ctx.insert(CURRENT_TIME_KEY, &Local::now());
        true
    }

    fn priority(&self) -> u8 {
        10
    }
}

inventory::submit! {
    &DatetimeVariableProvider as &dyn VariableProvider
}
