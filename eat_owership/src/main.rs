#![allow(unused)]

fn val_and_ownership_not_returned(s: String) -> () {
    println!("{}", s);
}

fn val_and_ownership_returned(s: String) -> String {
    println!("{}", s);
    println!("{}", s.len());
    s
}

fn val_and_borrowing(s: &String) -> () {
    println!("{}", s);
    println!("{}", s.len());
}

fn var_and_ownership_not_returned(mut ms: String) -> () {
    ms.push_str("_I'm_mutable");
    println!("{}", ms);
}

fn var_and_ownership_returned(mut ms: String) -> String {
    ms.push_str("_I'm_mutable  - too!");
    println!("{}", ms);
    println!("{}", ms.len());
    ms
}

fn var_and_borrowing(mut ms: &mut String) -> () {
    ms.push_str("_I'm_mutable_and borrowed...");
    println!("{}", ms);
    println!("{}", ms.len());
}

// look... no need to mark the parameter mutable if the param type is mutable
fn var_and_borrowing_2(ms: &mut String) -> () {
    ms.push_str("_I'm_mutable_and borrowed...");
    println!("{}", ms);
    println!("{}", ms.len());
}

fn main() {
    let hello = String::from("hello there!");
    val_and_ownership_not_returned(hello);

    let dogs = String::from("my dogs");
    let dogs2 = val_and_ownership_returned(dogs);
    println!("{}", dogs2);

    let horses = String::from("my horses");
    val_and_borrowing(&horses);
    println!("{}", horses);

    let fish = String::from("my fish");
    var_and_ownership_not_returned(fish);

    let goats = String::from("my goats");
    let goats2 = var_and_ownership_returned(goats);
    println!("{}", goats2);

    let mut worms = String::from("my worms");
    var_and_borrowing(&mut worms);
    println!("{}", worms);

    // these are all still valid:
    println!("{}", dogs2);
    println!("{}", horses);
    println!("{}", goats2);
    println!("{}", worms);

    // while these are gone (moved):
    // println!("{}", hello);
    // println!("{}", fish);
}
