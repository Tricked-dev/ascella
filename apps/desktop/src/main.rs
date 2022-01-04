use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use iced::{Application, Settings};

use ascella::create_main;
use ascella::ui::app::AscellaDesktop;
use ascella::util::screenshot;
use ascella::ScreenshotKind;

create_main!();
