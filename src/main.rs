use crate::chalk::BackgroundType;
use crate::chalk::ForegroundType;

pub mod chalk;
pub mod logger;

fn main() {
    logger::log(
        format!(
            "{} {}",
            chalk::Foreground::rgb(242, 96, 242, "Hello, world"),
            chalk::Background::rgb(242, 96, 242, "Hello, world")
        )
        .as_str(),
    )
}
