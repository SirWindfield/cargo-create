use crate::providers::VariableProvider;
use std::env::consts::{ARCH, FAMILY, OS};
use tera::Context;

const ARCH_KEY: &str = "arch";
const OS_KEY: &str = "os";
const OS_FAMILY_KEY: &str = "os_family";

pub struct SystemVariableProvider;
impl VariableProvider for SystemVariableProvider {
    fn populate(&self, ctx: &mut Context) -> bool {
        ctx.insert(ARCH_KEY, ARCH);
        ctx.insert(OS_KEY, OS);
        ctx.insert(OS_FAMILY_KEY, FAMILY);

        // These are always defined!
        true
    }

    fn priority(&self) -> u8 {
        10
    }
}

inventory::submit! {
    &SystemVariableProvider as &dyn VariableProvider
}
