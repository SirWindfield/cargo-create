// //! The default provider overrides every variable that has already been
// populated.
//
// use crate::providers::VariableProvider;
// use std::env::consts::{ARCH, FAMILY, OS};
// use tera::Context;
//
// pub struct SystemVariableProvider;
// impl VariableProvider for SystemVariableProvider {
//     fn populate(&self, ctx: &mut Context) -> bool {
//         ctx.insert(ARCH_KEY, ARCH);
//         ctx.insert(OS_KEY, OS);
//         ctx.insert(OS_FAMILY_KEY, FAMILY);
//
//         // These are always defined!
//         true
//     }
//
//     fn priority(&self) -> u8 {
//         std::u8::MAX
//     }
// }
//
// inventory::submit! {
//     &SystemVariableProvider as &dyn VariableProvider
// }
