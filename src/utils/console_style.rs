use console::Emoji;
use indicatif::ProgressStyle;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SPINNER_STYLE: ProgressStyle = create_spinner_style();
}

fn create_spinner_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
}

pub static SUCCESS_EMOJI: Emoji<'_, '_> = Emoji("✅", "");
pub static ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌", "");
