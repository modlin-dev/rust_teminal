pub struct LogTypes {
    pub log: Box<dyn Fn(&str) -> ()>,
}

pub fn new() -> LogTypes {
    return LogTypes {
        log: Box::new(|message| {
            println!(
                "{0} {1} {2}",
                "\x1B[38;2;243;238;248m[📄 Info!]\x1B[0m", "\x1B[30m>\x1b[0m", message
            )
        }),
    };
}

pub fn main() -> LogTypes {
    return new();
}
