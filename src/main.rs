fn main() {
    // imutable borrow
    let symbol = String::from("APPL");
    print_symbol(&symbol);
    print_symbol(&symbol); // lend many times
    println!("Still own it: {}", symbol);

    // mutable borrow
    let mut quantity = 10.0;
    add_shares(&mut quantity, 5.0);
    add_shares(&mut quantity, 2.5);
    println!("Final quantity: {}", quantity);

    // borrow checker rule
    let mut price = 100.0;
    let r1 = &price;
    let r2 = &price;
    // let r3 = &mut price; // Error
    println!("Two readers: {} {}", r1, r2); // borrows end

    let r3 = &mut price;
    *r3 += 5.0;
    println!("After mut borrow: {}", price);
}

fn add_shares(qty: &mut f64, delta: f64) {
    *qty += delta;
}

fn print_symbol(s: &String) {
    println!("Symbol = {}", s);
}
