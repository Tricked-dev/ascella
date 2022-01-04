//Macro to create the app so it can be used for the ascella-backend
#[macro_export]
macro_rules! create_main (
    () => {
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
    };
);
