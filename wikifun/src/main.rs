/*
A demo program that prints a random Wikipedia article title or a random Wikipedia article title and summary.
*/

use rand::Rng;
extern crate wikipedia;

//create a global variable that contains five topics
static TOPICS: [&str; 5] = ["Rust", "Python", "C++", "Java", "JavaScript"];

//create a function returns the the summary of a topic passed into the function
fn get_content(topic: &str) -> String {
    let topic = topic.to_string();
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title(topic);
    page.get_content().unwrap()
}

//build a function that prints the topic and summary randomly
fn random_topic() {
    //randomly select a topic from the TOPICS array
    let random_index = rand::thread_rng().gen_range(0..5);
    let topic = TOPICS[random_index];
    println!("Topic: {}", topic);
    println!("Summary: {}", get_content(topic));
}

fn main() {
    random_topic();
}
