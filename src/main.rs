mod terminal;
use terminal::chalk;

fn main() {
    let (_colors, chalk) = chalk();

    /*
    let mut _time: &str = "4:20 PM";
    println!(
        "{start: >spaces$} |, {hello} {first} {pogram}, at {_time}",
        spaces = 5,
        start = 1,
        first = "is the first",
        hello = "Hello, world!",
        pogram = "pogram you write on start"
    );
    */

    println!("{}", (chalk.foreground.rgb)(252, 106, 142, "Hello, world!"))
}
