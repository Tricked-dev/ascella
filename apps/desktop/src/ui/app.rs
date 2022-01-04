use home::home_dir;
use iced::svg::Handle;
use iced::{
    button, executor, scrollable, slider, svg, text_input, Application, Button, Column, Command,
    Container, Element, Length, Radio, Row, Rule, Svg, Text, Toggler,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use serde_json::Value;

use crate::ui::style;
use crate::util::{screenshot, update_config, AscellaConfig};
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
    TogglerToggled(bool),
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
            Message::TogglerToggled(dark) => {
                self.toggler_value = dark;

                if dark {
                    self.theme = *style::Theme::ALL
                        .iter()
                        .find(|x| format!("{:?}", x) == "Dark".to_owned())
                        .unwrap();
                } else {
                    self.theme = *style::Theme::ALL
                        .iter()
                        .find(|x| format!("{:?}", x) != "Dark".to_owned())
                        .unwrap();
                }
            }
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

                match update_config(&path) {
                    Ok(()) => {
                        let path_string = path.into_os_string().into_string().unwrap();
                        MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Config updated")
                            .set_text(&(&path_string).to_string())
                            .show_alert()
                            .unwrap();
                    }
                    Err(e) => {
                        let path_string = path.into_os_string().into_string().unwrap();
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title(&format!("OOh no failed to update config {:?}", e))
                            .set_text(&(&path_string).to_string())
                            .show_alert()
                            .unwrap();
                    }
                }
            }
            Message::ButtonPressed => {
                screenshot(ScreenshotKind::Area).unwrap();
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let darkmode = Toggler::new(
            self.toggler_value,
            String::from("Darkmode"),
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10)
        .style(self.theme);

        let row = if cfg!(target_os = "linux") {
            let button = Button::new(&mut self.button, Text::new("Take screenshot"))
                .padding(10)
                .on_press(Message::ButtonPressed)
                .style(self.theme);

            let config = Button::new(&mut self.config, Text::new("Upload config"))
                .padding(10)
                .on_press(Message::NewConfig)
                .style(self.theme);

            Row::new().push(button).push(config).spacing(20)
        } else {
            let button = Button::new(&mut self.button, Text::new("Take screenshot"))
                .padding(10)
                .on_press(Message::ButtonPressed)
                .style(self.theme);

            let text = Text::new("use ascella config <ascella.sxcu file> to set your config");
            Row::new().push(button).push(text).spacing(20)
        };

        let text = Text::new("© Ascella - image uploader");

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(darkmode)
            .push(row)
            .push(Rule::horizontal(38).style(self.theme))
            .push(text);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
