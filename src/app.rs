use iced::{Element, Length, Padding, Pixels};
use iced::alignment::Vertical;
use iced::widget::{row, image, text_input, container};
use crate::styles::borderless_input;

pub struct LauncherApp {
    state: LauncherState,
}

struct LauncherState {
    search_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String)
}

impl Default for LauncherApp {
    fn default() -> Self {
        Self::new()
    }
}

impl LauncherApp {
    
    pub fn new() -> Self {
        Self {
            state: LauncherState { search_text: String::new() },
        }
    }
    
    pub fn update(&mut self, message: Message){
        match message {
            Message::ContentChanged(new_text) => self.state.search_text = new_text
        };
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![
            container(
                image("assets/search_icon.png")
                    .width(Length::from(Pixels(40.0)))
                    .height(Length::from(Pixels(40.0))),
            ).padding(Padding::from([0.0, 10.0])),
            text_input("Search...", &self.state.search_text)
                .size(24.0)
                .style(borderless_input)
                .on_input(Message::ContentChanged)
        ].height(Length::Fill).align_y(Vertical::Center).into()
    }
}