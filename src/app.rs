use std::collections::HashMap;
use std::process::Command;
use iced::{event, keyboard, window, Element, Length, Padding, Pixels, Subscription};
use iced::alignment::Vertical;
use iced::keyboard::Key;
use iced::keyboard::key::Named;
use iced::widget::{row, image, text_input, container};
use crate::config::load_config_from_home_directory;
use crate::styles::search_input_style;

pub struct LauncherApp {
    state: LauncherState,
}

struct LauncherState {
    search_text: String,
    search_input_id: text_input::Id,
    commands_to_application_map: toml::Table
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    Submitted,
    EventOccurred(iced::Event),
    Close
}

impl Default for LauncherApp {
    fn default() -> Self {
        Self::new()
    }
}

impl LauncherApp {
    
    pub fn new() -> Self {

        Self {
            state: LauncherState {
                search_text: String::new(),
                search_input_id: text_input::Id::new("search_input"),
                commands_to_application_map: load_config_from_home_directory()
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    pub fn title(&self, window_id: window::Id) -> String {
        format!("Window: {}", window_id)
    }
    
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ContentChanged(new_text) => {
                self.state.search_text = new_text;
                iced::Task::none()
            },
            Message::EventOccurred(event) => self.handle_event(event),
            Message::Submitted => self.handle_search_submit(),
            _ => iced::Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let text_input = text_input("Search...", &self.state.search_text)
            .size(24.0)
            .style(search_input_style)
            .on_input(Message::ContentChanged)
            .on_submit(Message::Submitted)
            .id(self.state.search_input_id.clone());

        row![
            container(
                image("assets/search_icon.png")
                    .width(Length::from(Pixels(40.0)))
                    .height(Length::from(Pixels(40.0))),
            ).padding(Padding::from([0.0, 10.0])),
            text_input
        ].height(Length::Fill).align_y(Vertical::Center).into()
    }

    pub fn handle_event(&self, event: iced::Event) -> iced::Task<Message> {
        match event {
            iced::Event::Window(window::Event::Opened { .. }) => {
                text_input::focus(self.state.search_input_id.clone())
            }
            iced::Event::Window(window::Event::Unfocused) => {
                iced::Task::none()
            },
            iced::Event::Keyboard(keyboard_event) => {
                if let keyboard::Event::KeyPressed{ key: Key::Named(Named::Escape), ..} = keyboard_event {
                    return iced::exit()
                }
                iced::Task::none()
            }
            _ => iced::Task::none()
        }
    }

    pub fn handle_search_submit(&self) -> iced::Task<Message> {
        match self.state.search_text.as_str() {
            "term" => {
                let result = Command::new("open")
                    .arg("/Applications/iTerm.app")
                    .spawn();
                if let Err(error) = result {
                    log::error!("Error while trying to run term command: {:?}", error);
                }

                iced::exit()
            },
            _ => iced::Task::none(),
        }
    }
}