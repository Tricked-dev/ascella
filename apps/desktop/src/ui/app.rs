use home::home_dir;
use iced::{
    button, executor, scrollable, slider, text_input, Application, Button, Column, Command,
    Container, Element, Length, Radio, Text,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use serde_json::Value;

use crate::ui::style;
use crate::util::{screenshot, AscellaConfig};
use crate::ScreenshotKind;

#[derive(Default)]
pub struct AscellaDesktop {
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

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    NewConfig,
    ButtonPressed,
}

impl Application for AscellaDesktop {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Ascella - gui".to_owned()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::NewConfig => {
                let path = FileDialog::new()
                    .add_filter("SXCU File", &["sxcu", "json"])
                    .show_open_single_file()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return Command::none(),
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

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
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
}
