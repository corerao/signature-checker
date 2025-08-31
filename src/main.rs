// #![windows_subsystem = "windows"]
use iced::Font;
use signature_checker::App;
// read this page for example
// https://jl710.github.io/iced-guide/runtime/blocking_code/blocking_code.html

fn main() -> iced::Result {
    iced::application("signature-checker", App::update, App::view)
        .default_font(Font::with_name("微软雅黑"))
        // .font(iced_fonts::REQUIRED_FONT_BYTES)
        .window(iced::window::Settings {
            size: iced::Size {
                width: 520.0,
                height: 200.0,
            },
            position:iced::window::Position::Centered,
            ..Default::default()
        })
        .run()
}
