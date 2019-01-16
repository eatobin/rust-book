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

    let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    let mut b = [11, 22, 33]; // a[0] = 11, a[1] = 22, a[2] = 33

    b[0] = 99;
    println!("{:?}", a); //[1, 2, 3]
    println!("{:?}", b); //[99, 22, 33

    // a[0] = 99; not possible till a made mutable

    println!("{:?}", a); //[1, 2, 3]

    let eric: String = String::from("eric");
    let karen: String = "karen".to_string();
    let spot: &str = "spot";
    println!("{}", eric);
    println!("{}", karen);
    println!("{}", spot);
    let spot_twin: &str = &spot;
    println!("{}", spot_twin);

    let karen_twin: &str = &karen;
    println!("{}", karen);
    println!("{}", karen_twin);

    let mut karen_mut: String = "karen the mutt".to_string();
    println!("{}", karen_mut);

    karen_mut.push_str(&karen);
    println!("{}", karen_mut);
    println!("{}", karen_mut + &eric);
    println!("{}", eric);

    let zero: u8 = 77;
    let one: u8 = zero;
    println!("{}", zero);

    let mut a: [u8; 3] = [1, 2, 3];
    let mut array: [i32; 3] = [0; 3]; // [0, 0, 0]
    // let b = &a;
    println!("{:?}", a);
    let mut c = a; // no move - elements are Copy
    println!("{:?}", a);
    println!("{:?}", array);
    println!("{:?}", c);
    // b[0] = 11;  // won't work unless b is a mutable reference - see c, below
    c[0] = 11; // a is mutable and so is the reference: c
    println!("{:?}", c);

    let stack: &str = "stack";
    let heap: String = String::from("heap");
    println!("{}", stack);
    println!("{}", &stack);
    println!("{}", heap);
    println!("{}", &heap);

    let times = vec![
        "one-1".to_owned(),
        String::from("two-2"),
        String::from("three-3"),
    ];
    vectorcize_ref(&times);
    println!("{:?}", times);
    vectorcize(times);
}

fn vectorcize(v: Vec<String>) {
    for s in &v {
        println!("{}", s);
    }
    println!("{:?}", v);

    let lengths: Vec<usize> = v.iter().map(|time| time.len()).collect();
    println!("{:?}", lengths);
    println!("{:?}", v);

    let lengths: Vec<usize> = v.into_iter().map(|time| time.len()).collect();
    println!("{:?}", lengths);
//    println!("{:?}", v);
}

fn vectorcize_ref(rv: &Vec<String>) {
    for s in rv {
        println!("{}", s);
    }
    println!("{:?}", rv);

    let lengths: Vec<usize> = rv.iter().map(|time| time.len()).collect();
    println!("{:?}", lengths);
    println!("{:?}", rv);

    let lengths: Vec<usize> = rv.into_iter().map(|time| time.len()).collect();
    println!("{:?}", lengths);
    println!("{:?}", rv);
}
