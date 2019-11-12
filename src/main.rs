pub use pig_latin::convert;

fn main() {
    let apple = String::from("apple");

    let converted_apple = convert::to_pig_latin(&apple);
    println!("pig latin apple: {}", converted_apple);

    let first = String::from("first");

    let converted_first = convert::to_pig_latin(&first);
    println!("pig latin first: {}", converted_first);

}