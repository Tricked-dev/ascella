
use ascella::{take_ss, ScreenshotKind};
use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use home::home_dir;
use iced::{
    button, scrollable, slider, text_input, Button, Column, Container, Element, Length, Radio, Sandbox, Settings, Text,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        _ => Styling::run(Settings::default()),
    }
}

pub fn screenshot(t: ScreenshotKind) -> iced::Result {
    let mut path = home_dir().unwrap();

    path.extend(&[".ascella", "images"]);
    std::fs::create_dir_all(&path).unwrap();
    let filename = chrono::offset::Local::now()
        .format("%Y-%m-%d_%H-%M-%S.png")
        .to_string();
    path.extend(&[&filename]);

    take_ss(
        t,
        path.clone().into_os_string().into_string().unwrap(),
        true,
    );

    let form = multipart::Form::new().file("photo", path).unwrap();

    let mut headers = HeaderMap::new();
    let mut write_path = home_dir().unwrap();

    write_path.extend(&[".ascella", "config.toml"]);
    let config_raw = if let Ok(config_raw) = std::fs::read_to_string(write_path) {
        config_raw
    } else {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("config not detected please upload your config")
            .set_text("config not detected please upload your config\n\nPlease add a config file you can do this using the gui")
            .show_alert()
            .unwrap();
        println!("config not detected please upload your config");
        return Ok(());
    };

    let config: AscellaConfig = if let Ok(config) = toml::from_str(&config_raw) {
        config
    } else {
        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("invalid config")
            .set_text("Your config is invalid please use a valid ascella config")
            .show_alert()
            .unwrap();
        println!("Your config is invalid please use a valid ascella config");
        return Ok(());
    };
    headers.insert(
        "x-user-id",
        HeaderValue::from_str(&config.id.unwrap()).unwrap(),
    );
    headers.insert(
        "x-user-token",
        HeaderValue::from_str(&config.token.unwrap()).unwrap(),
    );
    headers.insert(
        "user-agent",
        HeaderValue::from_str("Ascella-uploader").unwrap(),
    );

    let client = reqwest::Client::new();
    let mut resp = client
        .post("https://ascella.wtf/v2/ascella/upload")
        .headers(headers)
        .multipart(form)
        .send()
        .unwrap();

    let text = resp.text().unwrap();

    let r: Value = serde_json::from_str(&text).unwrap();
    let url = r["url"].as_str().unwrap();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(url.to_owned()).unwrap();
    Ok(())
}

#[derive(Default)]
struct Styling {
    theme: style::Theme,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    button: button::State,
    config: button::State,
    slider: slider::State,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AscellaConfig {
    #[serde(rename = "x-user-id")]
    id: Option<String>,
    #[serde(rename = "x-user-token")]
    token: Option<String>,
    #[serde(rename = "user-agent")]
    headers: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(style::Theme),
    NewConfig,
    ButtonPressed,
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("Ascella - gui")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::NewConfig => {
                let path = FileDialog::new()
                    .add_filter("SXCU File", &["sxcu", "json"])
                    .show_open_single_file()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return,
                };
                let r: Value = std::fs::read_to_string(&path)
                    .map(|r| serde_json::from_str(&r))
                    .unwrap()
                    .unwrap();

                let config: AscellaConfig =
                    serde_json::from_str(&serde_json::to_string(&r["Headers"]).unwrap()).unwrap();

                let mut write_path = home_dir().unwrap();

                write_path.extend(&[".ascella", "config.toml"]);
                std::fs::write(&write_path, toml::to_string_pretty(&config).unwrap()).unwrap();
                // toml::toml! {
                //     user_id = r["Headers"]["x-user-id"].as_str()
                //     user_token = r["Headers"]["x-user-token"].as_str()
                //     effects = r["Headers"]["x-image-effects"].as_str()
                // }
                let path_string = path.into_os_string().into_string().unwrap();
                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title("Config updated")
                    .set_text(&(&path_string).to_string())
                    .show_alert()
                    .unwrap();
            }
            Message::ButtonPressed => {
                screenshot(ScreenshotKind::Area).unwrap();
            }
        };
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        let button = Button::new(&mut self.button, Text::new("Take screenshot"))
            .padding(10)
            .on_press(Message::ButtonPressed)
            .style(self.theme);

        let config = Button::new(&mut self.config, Text::new("Upload config"))
            .padding(10)
            .on_press(Message::NewConfig)
            .style(self.theme);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(choose_theme)
            .push(button)
            .push(config);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }

    fn background_color(&self) -> iced::Color {
        iced::Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

mod style {
    use iced::{
        button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input,
        toggler,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Light,
        Dark,
    }

    impl Theme {
        pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Light
        }
    }

    impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Container.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn radio::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Radio.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn text_input::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::TextInput.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => light::Button.into(),
                Theme::Dark => dark::Button.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn scrollable::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Scrollable.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn slider::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Slider.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::ProgressBar.into(),
            }
        }
    }

    impl<'a> From<Theme> for Box<dyn checkbox::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Checkbox.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn toggler::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Toggler.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn rule::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Rule.into(),
            }
        }
    }

    mod light {
        use iced::{button, Color, Vector};

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                    border_radius: 12.0,
                    shadow_offset: Vector::new(1.0, 1.0),
                    text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
        }
    }

    mod dark {
        use iced::{
            button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input,
            toggler, Color,
        };

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                    text_color: Color::WHITE.into(),
                    ..container::Style::default()
                }
            }
        }

        pub struct Radio;

        impl radio::StyleSheet for Radio {
            fn active(&self) -> radio::Style {
                radio::Style {
                    background: SURFACE.into(),
                    dot_color: ACTIVE,
                    border_width: 1.0,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self) -> radio::Style {
                radio::Style {
                    background: Color { a: 0.5, ..SURFACE }.into(),
                    ..self.active()
                }
            }
        }

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: SURFACE.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: ACCENT,
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1.0,
                    border_color: Color { a: 0.3, ..ACCENT },
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.4, 0.4, 0.4)
            }

            fn value_color(&self) -> Color {
                Color::WHITE
            }

            fn selection_color(&self) -> Color {
                ACTIVE
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: ACTIVE.into(),
                    border_radius: 3.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: HOVERED.into(),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1.0,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }

        pub struct Scrollable;

        impl scrollable::StyleSheet for Scrollable {
            fn active(&self) -> scrollable::Scrollbar {
                scrollable::Scrollbar {
                    background: SURFACE.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    scroller: scrollable::Scroller {
                        color: ACTIVE,
                        border_radius: 2.0,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> scrollable::Scrollbar {
                let active = self.active();

                scrollable::Scrollbar {
                    background: Color { a: 0.5, ..SURFACE }.into(),
                    scroller: scrollable::Scroller {
                        color: HOVERED,
                        ..active.scroller
                    },
                    ..active
                }
            }

            fn dragging(&self) -> scrollable::Scrollbar {
                let hovered = self.hovered();

                scrollable::Scrollbar {
                    scroller: scrollable::Scroller {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..hovered.scroller
                    },
                    ..hovered
                }
            }
        }

        pub struct Slider;

        impl slider::StyleSheet for Slider {
            fn active(&self) -> slider::Style {
                slider::Style {
                    rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 9.0 },
                        color: ACTIVE,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }

            fn hovered(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: HOVERED,
                        ..active.handle
                    },
                    ..active
                }
            }

            fn dragging(&self) -> slider::Style {
                let active = self.active();

                slider::Style {
                    handle: slider::Handle {
                        color: Color::from_rgb(0.85, 0.85, 0.85),
                        ..active.handle
                    },
                    ..active
                }
            }
        }

        pub struct ProgressBar;

        impl progress_bar::StyleSheet for ProgressBar {
            fn style(&self) -> progress_bar::Style {
                progress_bar::Style {
                    background: SURFACE.into(),
                    bar: ACTIVE.into(),
                    border_radius: 10.0,
                }
            }
        }

        pub struct Checkbox;

        impl checkbox::StyleSheet for Checkbox {
            fn active(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: if is_checked { ACTIVE } else { SURFACE }.into(),
                    checkmark_color: Color::WHITE,
                    text_color: Color::BLACK,
                    border_radius: 2.0,
                    border_width: 1.0,
                    border_color: ACTIVE,
                }
            }

            fn hovered(&self, is_checked: bool) -> checkbox::Style {
                checkbox::Style {
                    background: Color {
                        a: 0.8,
                        ..if is_checked { ACTIVE } else { SURFACE }
                    }
                    .into(),
                    ..self.active(is_checked)
                }
            }
        }

        pub struct Toggler;

        impl toggler::StyleSheet for Toggler {
            fn active(&self, is_active: bool) -> toggler::Style {
                toggler::Style {
                    background: if is_active { ACTIVE } else { SURFACE },
                    background_border: None,
                    foreground: if is_active { Color::WHITE } else { ACTIVE },
                    foreground_border: None,
                }
            }

            fn hovered(&self, is_active: bool) -> toggler::Style {
                toggler::Style {
                    background: if is_active { ACTIVE } else { SURFACE },
                    background_border: None,
                    foreground: if is_active {
                        Color {
                            a: 0.5,
                            ..Color::WHITE
                        }
                    } else {
                        Color { a: 0.5, ..ACTIVE }
                    },
                    foreground_border: None,
                }
            }
        }

        pub struct Rule;

        impl rule::StyleSheet for Rule {
            fn style(&self) -> rule::Style {
                rule::Style {
                    color: SURFACE,
                    width: 2,
                    radius: 1.0,
                    fill_mode: rule::FillMode::Padded(15),
                }
            }
        }
    }
}
