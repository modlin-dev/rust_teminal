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

#[allow(dead_code)]
pub struct ChalkGround {
    pub rgb: Box<dyn Fn(u8, u8, u8, &str) -> String>,
    pub black: Box<dyn Fn(&str) -> String>,
    pub red: Box<dyn Fn(&str) -> String>,
    pub green: Box<dyn Fn(&str) -> String>,
    pub orange: Box<dyn Fn(&str) -> String>,
    pub blue: Box<dyn Fn(&str) -> String>,
    pub magenta: Box<dyn Fn(&str) -> String>,
    pub cyan: Box<dyn Fn(&str) -> String>,
    pub white: Box<dyn Fn(&str) -> String>,
}

#[allow(dead_code)]
pub struct Chalk {
    pub custom: Box<dyn Fn(&str, &str) -> String>,
    pub foreground: ChalkGround,
    pub background: ChalkGround,
}

pub struct Main {
    pub chalk: Chalk,
    pub colors: Colors,
}

pub fn main() -> Main {
    let colors: Colors = Colors {
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
    };

    let chalk: Chalk = Chalk {
        custom: Box::new(|string, message| format!("\x1B[{string}m{message}\x1b[0m")),
        foreground: ChalkGround {
            rgb: Box::new(|r, g, b, message| {
                format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, message)
            }),
            black: Box::new(|message| format!("\x1B[30m{}\x1B[0m", message)),
            red: Box::new(|message| format!("\x1B[31m{}\x1B[0m", message)),
            green: Box::new(|message| format!("\x1B[32m{}\x1B[0m", message)),
            orange: Box::new(|message| format!("\x1B[33m{}\x1B[0m", message)),
            blue: Box::new(|message| format!("\x1B[34m{}\x1B[0m", message)),
            magenta: Box::new(|message| format!("\x1B[35m{}\x1B[0m", message)),
            cyan: Box::new(|message| format!("\x1B[36m{}\x1B[0m", message)),
            white: Box::new(|message| format!("\x1B[37m{}\x1B[0m", message)),
        },
        background: ChalkGround {
            rgb: Box::new(|r, g, b, message| {
                format!("\x1B[48;2;{};{};{}m{}\x1B[0m", r, g, b, message)
            }),
            black: Box::new(|message| format!("\x1B[40m{}\x1B[0m", message)),
            red: Box::new(|message| format!("\x1B[41m{}\x1B[0m", message)),
            green: Box::new(|message| format!("\x1B[42m{}\x1B[0m", message)),
            orange: Box::new(|message| format!("\x1B[43m{}\x1B[0m", message)),
            blue: Box::new(|message| format!("\x1B[44m{}\x1B[0m", message)),
            magenta: Box::new(|message| format!("\x1B[45m{}\x1B[0m", message)),
            cyan: Box::new(|message| format!("\x1B[46m{}\x1B[0m", message)),
            white: Box::new(|message| format!("\x1B[47m{}\x1B[0m", message)),
        },
    };

    return Main { chalk, colors };
}
