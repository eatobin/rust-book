#![allow(unused)]
fn main() {
    let hello = String::new();
    println!("{}", hello); // prints nothing

    let hello = 42;
    println!("{}", hello);

    let hello = String::from("hello");
    println!("{}", &hello);

    let mut hello_2 = String::from("hello_2");
    println!("{}", hello_2);

    hello_2.push('2');
    hello_2.push_str("_skidoo!");
    println!("{}", hello_2);

    let hello_22 = &mut hello_2;
    // println!("{}", hello_2); // cannot borrow `hello_2` as immutable
    // because it is also borrowed as mutable
    println!("{}", hello_22);

    hello_22.push_str("_last_one");
    println!("{}", hello_22);
}
