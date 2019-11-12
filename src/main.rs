pub use pig_latin::convert;

fn main() {
    let apple = String::from("apple");

    let converted_apple = convert::to_pig_latin(&apple);
    println!("pig latin first: {}", converted_apple);
}