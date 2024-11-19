use iced::{window, Point, Size};
use iced::window::Position;
use crate::app::LauncherApp;

pub mod app;
pub mod styles;
mod config;

fn main() -> iced::Result {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    iced::application("aqualauncher", LauncherApp::update, LauncherApp::view)
        .window(window::Settings {
            size: Size::new(800.0, 60.0),
            position: Position::SpecificWith(position_window),
            decorations: false,
            ..window::Settings::default()
        })
        .subscription(LauncherApp::subscription)
        .settings(iced::Settings {
            ..iced::Settings::default()
        })
        .run()
}

pub fn position_window(window_size: Size, monitor_resolution: Size) -> Point {
    if monitor_resolution != Size::ZERO {
        Point::new(
            monitor_resolution.width / 2.0 - window_size.width / 2.0,
            monitor_resolution.height / 2.0 - (monitor_resolution.height / 6.0),
        )
    } else {
        Point::new(0.0, 0.0)
    }
}
