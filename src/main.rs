use json::object;

fn main() {
    let mut _time: &str = "4:20 PM";
    println!(
        "{start: >spaces$} |, {hello} {first} {pogram}, at {_time}",
        spaces = 5,
        start = 1,
        first = "is the first",
        hello = "Hello, world!",
        pogram = "pogram you write on start"
    );

    let colors: json::JsonValue = object! {
      None: "\x1b[0m",
      Forground: {
        Black: "\x1B[30m",
        Red: "\x1B[31m",
        Green: "\x1B[32m",
        Orange: "\x1B[33m",
        Blue: "\x1B[34m",
        Magenta: "\x1B[35m",
        Cyan: "\x1B[36m",
        White: "\x1B[37m"
      },
      Background: {
        Black: "\x1B[40m",
        Red: "\x1B[41m",
        Green: "\x1B[42m",
        Orange: "\x1B[43m",
        Blue: "\x1B[44m",
        Magenta: "\x1B[45m",
        Cyan: "\x1B[46m",
        White: "\x1B[47m"
      }
    };

    println!("{}Hello, world!", colors["Forground"]["Red"])
}
