fn main() {
    let a = String::from("AAPL");
    let b = a;

    // print!("{}", a);
    println!("b owns: {}", b);

    let x = 42;
    let y = x;
    println!("x: {}, y: {}", x, y);

    let symbol = String::from("GOOGL");
    consume_symbol(symbol);
    // println!("symbol: {}", symbol);

    let s1 = String::from("MSFT");
    let s1 = take_and_give_back(s1);
    println!("Got back: {}", s1);
}

fn consume_symbol(s: String) {
    println!("Function received: {}", s);
}

fn take_and_give_back(s: String) -> String {
    println!("Borrow-and-returned: {}", s);
    s
}
