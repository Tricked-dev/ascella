use std::path::PathBuf;
use std::{fs, thread, time};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use iced::{Application, Settings};

use ascella_desktop::ui::app::AscellaDesktop;
use ascella_desktop::util::{screenshot, update_config, upload};
use ascella_desktop::ScreenshotKind;

pub fn main() -> iced::Result {
  let app = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .subcommand(
      App::new("area")
        .about("Screenshot a area")
        .arg(Arg::new("delay").help("delay to take the screenshot").short('d').long("delay").takes_value(true)),
    )
    .subcommand(
      App::new("window")
        .about("screenshot the current window")
        .arg(Arg::new("delay").help("delay to take the screenshot").short('d').long("delay").takes_value(true)),
    )
    .subcommand(
      App::new("full")
        .about("screenshot all screens")
        .arg(Arg::new("delay").help("delay to take the screenshot").short('d').long("delay").takes_value(true)),
    )
    .subcommand(
      App::new("upload")
        .about("upload a file")
        .arg(Arg::new("file").help("path to image to upload").required(true).takes_value(true)),
    )
    .subcommand(
      App::new("config")
        .about("set the config file")
        .arg(Arg::new("config").help("path to the config file ( ascella.sxcu )").required(true).takes_value(true)),
    );

  let matches = app.get_matches();

  match matches.subcommand() {
    Some(("area", matches)) => make_screenshot(ScreenshotKind::Area, matches),
    Some(("window", matches)) => make_screenshot(ScreenshotKind::Window, matches),
    Some(("full", matches)) => make_screenshot(ScreenshotKind::Full, matches),
    Some(("upload", args)) => {
      let file = PathBuf::from(args.value_of("file").unwrap());
      let full_path = fs::canonicalize(&file).expect("File not found");
      println!("{}", upload(full_path).expect("Failed to upload file"));
      println!("\nFile uploaded");
      println!("Have a nice day!");
      Ok(())
    }
    Some(("config", args)) => {
      let file = PathBuf::from(args.value_of("config").unwrap());
      match update_config(fs::canonicalize(&file).unwrap()) {
        Ok(()) => {
          println!("Updated your config check ascella --help for more commands");
          println!("Have a nice day!");
        }
        Err(e) => {
          println!("Failed to update config please use a valid ascella config file,\n\n\nError {:?}\n", e);
          println!("Have a nice day!");
        }
      };
      Ok(())
    }
    _ => AscellaDesktop::run(Settings::default()),
  }
}

fn make_screenshot(kind: ScreenshotKind, matches: &ArgMatches) -> iced::Result {
  if let Some(val) = matches.value_of("delay") {
    let delay = time::Duration::from_secs(val.parse::<i32>().unwrap().try_into().unwrap());

    thread::sleep(delay);
  }
  screenshot(kind)
}
