pub mod chalk;
pub mod logger;

fn main() {
    let chalk = chalk::main().chalk;
    let logging = logger::new();
    let value: &str = &(chalk.foreground.red)("Hello, world!");

    (logging.log)(value);
}
