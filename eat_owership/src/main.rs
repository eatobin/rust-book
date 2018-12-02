#![allow(unused)]
fn main() {
    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");

    println!("{}", hello);

    let bye = hello;

    // println!("{}", hello); // won't work - moved

    println!("{}", bye);

    // bye.push_str(" And one more. So bye is mutable too"); bye is not mutable so this wont work!

    let mut maybe = bye;

    // println!("{}", bye); // won't work - moved

    maybe.push_str(" And one more. So maybe is mutable even though bye is not");

    println!("{}", maybe);

    let mut maybe_ref = &mut maybe;

    println!("{}", &mut maybe_ref); // maybe still in scope

    maybe_ref.push_str(" Line 2");

    println!("{}", maybe_ref);
}
