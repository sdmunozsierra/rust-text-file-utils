#[cfg(feature = "tui")]
use cursive::views::TextView;
#[cfg(feature = "tui")]
use cursive::Cursive;

#[cfg(feature = "tui")]
pub fn run_tui() {
    let mut siv = Cursive::default();
    siv.add_layer(TextView::new("Welcome to TextFileUtils TUI"));
    siv.run();
}
