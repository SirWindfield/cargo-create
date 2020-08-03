use crate::steps::NUMBER_OF_STEPS;
use console::{style, Emoji};

/// Used when applying all specified features.
pub(crate) const APPLYING_EMOJI: &Emoji = &Emoji("🔧", ">");
/// Used when interactively asking for variable values.
pub(crate) const ASKING_EMOJI: &Emoji = &Emoji("✏", ">");
/// Used when checking out the repository.
pub(crate) const CHECKOUT_EMOJI: &Emoji = &Emoji("🚧", ">");
/// Used when cleaning up the generated project.
pub(crate) const CLEANING_EMOJI: &Emoji = &Emoji("🚿", ">");
/// Used when downloading the repository.
pub(crate) const DOWNLOAD_EMOJI: &Emoji = &Emoji("⏬", ">");
/// Used when filling out the variable placeholders.
pub(crate) const FILLING_EMOJI: &Emoji = &Emoji("🏗", ">");
/// Used when indexing the repository.
pub(crate) const INDEX_EMOJI: &Emoji = &Emoji("🔍", ">");

/// Prints a message indicating the current step.
#[inline]
pub(crate) fn print_step(step: usize, msg: &str) {
    let step_indicator = style(format!("[{}/{}]", step, NUMBER_OF_STEPS))
        .bold()
        .dim();
    println!("{} {}", step_indicator, msg);
}
