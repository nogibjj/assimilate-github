/*
A hello world program in Rust that prints hello world in the ten most popular languages.
*/

use rand::Rng;

//create a global variable that contains all 10 language greetings
static LANGUAGES: [&str; 10] = [
    "Hello World",
    "Hola Mundo",
    "Bonjour le monde",
    "Hallo Welt",
    "Ciao mondo",
    "Hej världen",
    "Olá Mundo",
    "Привет, мир",
    "こんにちは世界",
    "你好，世界",
];

//build a function that prints the language greetings randomly
fn random_greeting() {
    //randomly select a language from the LANGUAGES array
    let random_index = rand::thread_rng().gen_range(0..10);
    println!("{}", LANGUAGES[random_index]);
}

fn main() {
    random_greeting();
}
