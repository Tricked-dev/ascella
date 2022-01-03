use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use iced::{Application, Settings};

use ascella::ui::app::AscellaDesktop;
use ascella::util::screenshot;
use ascella::ScreenshotKind;


pub fn main() -> iced::Result {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(App::new("area").about("Screenshot a area"))
        .subcommand(App::new("window").about("screenshot the current window"))
        .subcommand(App::new("full").about("screenshot all screens"));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("area", _)) => screenshot(ScreenshotKind::Area),
        Some(("window", _)) => screenshot(ScreenshotKind::Window),
        Some(("full", _)) => screenshot(ScreenshotKind::Full),
        _ => AscellaDesktop::run(Settings::default()),
    }
}
