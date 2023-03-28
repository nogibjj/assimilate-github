/* Here is the explanation for the code above:
1. We import the io and time modules.
2. We define an array of colors that we will use.
3. We iterate through the string "Hello, World!" and print it one letter at a time.
4. We print a space.
5. We iterate through each color in the colors array.
6. We print a space with the color on it.
7. We print the same letter again.
8. We sleep for 100 milliseconds.
9. We print a newline to end the line. */

use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    print_colored_hello_world();
}

fn print_colored_hello_world() {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_colors_length() {
        let colors = [
            ansi_term::Color::Red,
            ansi_term::Color::Green,
            ansi_term::Color::Yellow,
            ansi_term::Color::Blue,
            ansi_term::Color::Purple,
            ansi_term::Color::Cyan,
            ansi_term::Color::Fixed(231),
        ];

        assert_eq!(colors.len(), 7);
    }
}
