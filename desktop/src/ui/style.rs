use crate::ui::{dark_theme, light_theme};
use iced::{button, checkbox, container, progress_bar, radio, rule, scrollable, slider, text_input, toggler};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Theme {
  Light,
  Dark,
}

impl Theme {
  pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
}

impl Default for Theme {
  fn default() -> Self {
    Theme::Light
  }
}

impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Container.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn radio::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Radio.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn text_input::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::TextInput.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => light_theme::Button.into(),
      Theme::Dark => dark_theme::Button.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn scrollable::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Scrollable.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn slider::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Slider.into(),
    }
  }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::ProgressBar.into(),
    }
  }
}

impl<'a> From<Theme> for Box<dyn checkbox::StyleSheet + 'a> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Checkbox.into(),
    }
  }
}

impl From<Theme> for Box<dyn toggler::StyleSheet> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Toggler.into(),
    }
  }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Light => Default::default(),
      Theme::Dark => dark_theme::Rule.into(),
    }
  }
}
