pub mod app;

use iced::{Application, Error, Settings};

fn main() -> Result<(), Error> {
    app::VedasSDK::run(Settings::default())
}
