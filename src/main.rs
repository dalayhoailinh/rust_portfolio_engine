fn main() {
    // move
    let a = String::from("AAPL");
    let b = a;
    // println!("{}", a); // Error
    println!("b owns: {}", b);

    // copy
    let x = 42;
    let y = x; // i32 is 'Copy', valid
    println!("x = {}, y = {}", x, y);

    // move into a function
    let symbol = String::from("GOOGL");
    consume_symbol(symbol); // symbol moved into the functioni
    // println!("{}", symbol); // Error

    // return ownership back
    let s1 = String::from("MSFT");
    let s1 = take_and_gice_back(s1);
    println!("Got back: {}", s1);
}

fn take_and_gice_back(s: String) -> String {
    println!("Borrowed and returned: {}", s);
    s
}

fn consume_symbol(s: String) {
    println!("Function received: {}", s);
}
