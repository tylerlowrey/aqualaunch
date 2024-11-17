use iced::{Background, Border, Color, Theme};
use iced::widget::text_input::{Status, Style};

pub fn search_input_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default().color(Color::TRANSPARENT),
        icon: Default::default(),
        placeholder: Color::from_rgba8(100, 100, 100, 0.5),
        value: Color::WHITE,
        selection: Default::default(),
    }
}