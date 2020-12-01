pub mod app;

use iced::{Error, Sandbox, Settings};

fn main() -> Result<(), Error> {
    app::VedasSDK::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
