pub struct ColorGround {
    pub black: String,
    pub red: String,
    pub green: String,
    pub orange: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[allow(dead_code)]
pub struct Colors {
    pub none: String,
    pub forground: ColorGround,
    pub background: ColorGround,
}

pub fn colors() -> Colors {
    Colors {
        none: String::from("\x1b[0m"),
        forground: ColorGround {
            black: String::from("\x1B[30m"),
            red: String::from("\x1B[31m"),
            green: String::from("\x1B[32m"),
            orange: String::from("\x1B[33m"),
            blue: String::from("\x1B[34m"),
            magenta: String::from("\x1B[35m"),
            cyan: String::from("\x1B[36m"),
            white: String::from("\x1B[37m"),
        },
        background: ColorGround {
            black: String::from("\x1B[40m"),
            red: String::from("\x1B[41m"),
            green: String::from("\x1B[42m"),
            orange: String::from("\x1B[43m"),
            blue: String::from("\x1B[44m"),
            magenta: String::from("\x1B[45m"),
            cyan: String::from("\x1B[46m"),
            white: String::from("\x1B[47m"),
        },
    }
}

pub struct Foreground;
pub trait ForegroundType {
    fn rgb(r: u8, g: u8, b: u8, message: &str) -> String;
    fn black(message: &str) -> String;
    fn red(message: &str) -> String;
    fn green(message: &str) -> String;
    fn orange(message: &str) -> String;
    fn blue(message: &str) -> String;
    fn magenta(message: &str) -> String;
    fn cyan(message: &str) -> String;
    fn white(message: &str) -> String;
}
impl ForegroundType for Foreground {
    fn rgb(r: u8, g: u8, b: u8, message: &str) -> String {
        format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, message)
    }
    fn black(message: &str) -> String {
        format!("\x1B[30m{}\x1B[0m", message)
    }
    fn red(message: &str) -> String {
        format!("\x1B[31m{}\x1B[0m", message)
    }
    fn green(message: &str) -> String {
        format!("\x1B[32m{}\x1B[0m", message)
    }
    fn orange(message: &str) -> String {
        format!("\x1B[33m{}\x1B[0m", message)
    }
    fn blue(message: &str) -> String {
        format!("\x1B[34m{}\x1B[0m", message)
    }
    fn magenta(message: &str) -> String {
        format!("\x1B[35m{}\x1B[0m", message)
    }
    fn cyan(message: &str) -> String {
        format!("\x1B[36m{}\x1B[0m", message)
    }
    fn white(message: &str) -> String {
        format!("\x1B[37m{}\x1B[0m", message)
    }
}

pub struct Background;
pub trait BackgroundType {
    fn rgb(r: u8, g: u8, b: u8, message: &str) -> String;
    fn black(message: &str) -> String;
    fn red(message: &str) -> String;
    fn green(message: &str) -> String;
    fn orange(message: &str) -> String;
    fn blue(message: &str) -> String;
    fn magenta(message: &str) -> String;
    fn cyan(message: &str) -> String;
    fn white(message: &str) -> String;
}
impl BackgroundType for Background {
    fn rgb(r: u8, g: u8, b: u8, message: &str) -> String {
        format!("\x1B[48;2;{};{};{}m{}\x1B[0m", r, g, b, message)
    }
    fn black(message: &str) -> String {
        format!("\x1B[40m{}\x1B[0m", message)
    }
    fn red(message: &str) -> String {
        format!("\x1B[41m{}\x1B[0m", message)
    }
    fn green(message: &str) -> String {
        format!("\x1B[42m{}\x1B[0m", message)
    }
    fn orange(message: &str) -> String {
        format!("\x1B[43m{}\x1B[0m", message)
    }
    fn blue(message: &str) -> String {
        format!("\x1B[44m{}\x1B[0m", message)
    }
    fn magenta(message: &str) -> String {
        format!("\x1B[45m{}\x1B[0m", message)
    }
    fn cyan(message: &str) -> String {
        format!("\x1B[46m{}\x1B[0m", message)
    }
    fn white(message: &str) -> String {
        format!("\x1B[47m{}\x1B[0m", message)
    }
}
