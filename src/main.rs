use iced::{window, Sandbox, Settings};

mod gui;
mod registry;

fn main() -> iced::Result {
    let size = (310, 555);
    let settings = Settings {
        window: window::Settings {
            size,
            min_size: Some(size),
            ..window::Settings::default()
        },
        default_text_size: 14,
        ..Settings::default()
    };

    gui::Gui::run(settings)
}
