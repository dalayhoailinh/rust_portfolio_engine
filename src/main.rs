fn main() {
    let symbol = String::from("AAPL");
    print_symbol(&symbol);
    print_symbol(&symbol);
    println!("Still own it: {}", symbol);

    let mut quantity: f64 = 10.0;
    add_shares(&mut quantity, 5.0);
    add_shares(&mut quantity, 2.5);
    println!("Final quantity: {}", quantity);

    let mut price = 100.0;
    let r1 = &price;
    let r2 = &price;
    println!("Two readers: {}, {}", r1, r2);

    println!("{}", r1);
    let r3 = &mut price;
    
    *r3 += 5.0;
    println!("After mut borrow: {}", price);
}

fn print_symbol(s: &String) {
    println!("Symbol: {}", s);
}

fn add_shares(qty: &mut f64, delta: f64) {
    *qty += delta;
}