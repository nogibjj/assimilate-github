use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let colors = [
        ansi_term::Color::Red,
        ansi_term::Color::Green,
        ansi_term::Color::Yellow,
        ansi_term::Color::Blue,
        ansi_term::Color::Purple,
        ansi_term::Color::Cyan,
        ansi_term::Color::Fixed(231),
    ];

    for letter in "Hello, World!".chars() {
        print!("{}", letter);
        io::stdout().flush().unwrap();
        for color in &colors {
            print!("{}", ansi_term::Color::Fixed(231).on(*color).paint(" "));
            io::stdout().flush().unwrap();
            print!("{}", letter);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
