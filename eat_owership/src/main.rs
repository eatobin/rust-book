#![allow(unused)]

fn val_and_borrowing(s: &String) -> () {
    println!("In the val_and_borrowing function: {}", s);
    println!("{}\n", s.len());
}

fn var_and_borrowing_(ms: &mut String) -> () {
    println!("In the var_and_borrowing function: {}", ms);
    println!("{}\n", ms.len());
    ms.push_str(" ...tacked on");
}

// fn val_and_ownership_not_returned(s: String) -> () {
//     println!("In the val_and_ownership_not_returned function: {}", s);
//     println!("{}\n", s.len());
// }

// fn val_and_ownership_returned(s: String) -> String {
//     println!("In the val_and_ownership_returned function: {}", s);
//     println!("{}\n", s.len());
//     s
// }

// fn val_and_ownership_returned(mut v: Vec<u8>) -> Vec<u8> {
//     v.push(99);
//     v
// }

// fn var_and_ownership_not_returned(mut ms: String) -> () {
//     ms.push_str(" -> Pushed onto the parameter (it's mutable) -> In the var_and_ownership_not_returned function");
//     println!("{}\n", ms);
// }

// fn var_and_ownership_returned(mut ms: String) -> String {
//     ms.push_str(" -> Pushed onto the parameter (it's mutable) -> In the var_and_ownership_returned function");
//     println!("{}", ms);
//     println!("{}\n", ms.len());
//     ms
// }

// fn var_and_borrowing(mut ms: &mut String) -> () {
//     ms.push_str("In the var_and_borrowing function");
//     println!("{}", ms);
//     println!("{}\n", ms.len());
// }

// // look... no need to mark the parameter mutable if the param type is mutable
// fn var_and_borrowing_no_mut_param(ms: &mut String) -> () {
//     ms.push_str("In the var_and_borrowing_no_mut_param function");
//     println!("{}", ms);
//     println!("{}\n", ms.len());
// }

fn main() {
    println!();
    let immut_string: String = String::from("Immut string!");
    let mut mut_string: String = String::from("Mut string!");

    println!(
        "Print the immutable string - first time: {:?}",
        immut_string
    );
    println!(
        "Print the immutable string - second time: {:?}\n",
        immut_string
    );

    val_and_borrowing(&immut_string);

    println!("Print the borrowed immutable string: {:?}\n", immut_string);

    var_and_borrowing_(&mut mut_string);

    println!("Print the borrowed mutable string: {:?}", mut_string);
    println!(
        "Print the borrowed mutable string again: {:?}\n",
        mut_string
    );

    // val_and_ownership_not_returned(hello);
    // val_and_ownership_returned(hello); Won't work cuz hello has been moved

    // let dogs = String::from("my dogs");
    // let dogs2 = val_and_ownership_returned(dogs);
    // let mut dogs: Vec<u8> = vec![1, 2];
    // let dogs2 = val_and_ownership_returned(dogs);
    // println!("Just the vec parameter - which was returned: {:?}\n", dogs2);
    // println!("Now the original vec: {:?}\n", dogs2);

    // let horses = String::from("my horses");
    // val_and_borrowing(&horses);
    // println!(
    //     "Just the string - which was borrowed and never left: {}\n",
    //     horses
    // );

    // let fish = String::from("my fish");
    // var_and_ownership_not_returned(fish);
    // // var_and_ownership_returned(fish); This won't work cuz fish is moved

    // let goats = String::from("my goats");
    // let goats2 = var_and_ownership_returned(goats);
    // println!(
    //     "The mutated string parameter - which was returned: {}\n",
    //     goats2
    // );
    //
    // let mut worms = String::from("my worms");
    // var_and_borrowing(&mut worms);
    // println!("{}\n", worms);
    //
    // var_and_borrowing_no_mut(&mut worms);
    // println!("{}\n", worms);
    //
    // // these are all still valid:
    // println!("{}\n", dogs2);
    // println!("{}\n", horses);
    // println!("{}\n", goats2);
    // println!("{}\n", worms);
    //
    // // while these are gone (moved):
    // // println!("{}\n", hello);
    // // println!("{}\n", fish);
    //
    // let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3
    // let mut b = [11, 22, 33]; // a[0] = 11, a[1] = 22, a[2] = 33
    //
    // b[0] = 99;
    // println!("{:?}\n", a); //[1, 2, 3]
    // println!("{:?}\n", b); //[99, 22, 33
    //
    // // a[0] = 99; not possible till a made mutable
    //
    // println!("{:?}\n", a); //[1, 2, 3]
    //
    // let eric: String = String::from("eric");
    // let karen: String = "karen".to_string();
    // let spot: &str = "spot";
    // println!("{}\n", eric);
    // println!("{}\n", karen);
    // println!("{}\n", spot);
    // let spot_twin: &str = &spot;
    // println!("{}\n", spot_twin);
    //
    // let karen_twin: &str = &karen;
    // println!("{}\n", karen);
    // println!("{}\n", karen_twin);
    //
    // let mut karen_mut: String = "karen the mutt".to_string();
    // println!("{}\n", karen_mut);
    //
    // karen_mut.push_str(&karen);
    // println!("{}\n", karen_mut);
    // println!("{}\n", karen_mut + &eric);
    // println!("{}\n", eric);
    //
    // let zero: u8 = 77;
    // let one: u8 = zero;
    // println!("{}\n", zero);
    //
    // let mut a: [u8; 3] = [1, 2, 3];
    // let mut array: [i32; 3] = [0; 3]; // [0, 0, 0]
    //                                   // let b = &a;
    // println!("{:?}\n", a);
    // let mut c = a; // no move - elements are Copy
    // println!("{:?}\n", a);
    // println!("{:?}\n", array);
    // println!("{:?}\n", c);
    // // b[0] = 11;  // won't work unless b is a mutable reference - see c, below
    // c[0] = 11; // a is mutable and so is the reference: c
    // println!("{:?}\n", c);
    //
    // let stack: &str = "stack";
    // let heap: String = String::from("heap");
    // println!("{}\n", stack);
    // println!("{}\n", &stack);
    // println!("{}\n", heap);
    // println!("{}\n", &heap);
    //
    // let times = vec![
    //     "one-1".to_owned(),
    //     String::from("two-2"),
    //     String::from("three-3"),
    // ];
    // vectorcize_ref(&times);
    // println!("{:?}\n", times);
    // //    vectorcize(times);
    //
    // let mut times_mut = vec![
    //     "one-11".to_owned(),
    //     String::from("two-22"),
    //     String::from("three-33"),
    // ];
    // println!("{:?}\n", &times_mut);
    // vectorcize_ref_mut(&mut times_mut);
    // println!("{:?}\n", times_mut);
    //
    // let v1 = vec![1, 2, 3];
    //
    // let v1_iter = v1.iter();
    //
    // for val in v1_iter {
    //     println!("Got: {}\n", val);
    // }
    //
    // let v2_iter = v1.iter();
    //
    // for val in v2_iter {
    //     println!("Got again: {}\n", val);
    // }
    //
    // let t1_iter = times.iter();
    //
    // for val in t1_iter {
    //     println!("Got: {}\n", val);
    // }
    //
    // let t2_iter = times.iter();
    //
    // for val in t2_iter {
    //     println!("Got again: {}\n", val);
    // }
    //
    // let times2 = vec![
    //     "one-11111".to_owned(),
    //     String::from("two-22222"),
    //     String::from("three-33333"),
    // ];
    //
    // let mut times2_iter = times2.iter();
    // println!("{:?}\n", times2_iter.next());
    // println!("{:?}\n", times2_iter.next());
    // println!("{:?}\n", times2_iter.next());
    // println!("{:?}\n", times2_iter.next());
    //
    // let times3 = vec![
    //     "one-one".to_owned(),
    //     String::from("two-two"),
    //     String::from("three-three"),
    // ];
    //
    // let mut times3_into_iter = times3.into_iter();
    // println!("{:?}\n", times3_into_iter.next());
    // println!("{:?}\n", times3_into_iter.next());
    // println!("{:?}\n", times3_into_iter.next());
    // println!("{:?}\n", times3_into_iter.next());
    //
    // let names = vec!["Jane", "Jill", "Jack", "John"];
    //
    // let total_bytes = names
    //     .iter()
    //     .map(|name: &&str| name.len())
    //     .fold(0, |acc, len| acc + len);
    //
    // assert_eq!(total_bytes, 16);
    // use_names_for_something_else(names);
    //
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];
    //
    // // `iter()` for vecs yields `&i32`.
    // let mut iter = vec1.iter();
    // // `into_iter()` for vecs yields `i32`.
    // let mut into_iter = vec2.into_iter();
    //
    // // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    // println!("Find 2 in vec1: {:?}\n", iter.find(|&&x| x == 2));
    // // A reference to what is yielded is `&i32`. Destructure to `i32`.
    // println!("Find 2 in vec2: {:?}\n", into_iter.find(|&x| x == 2));
    //
    // let vec1 = vec!["one", "two", "three"];
    // let vec2 = vec!["four", "five", "six"];
    //
    // let mut iter = vec1.iter();
    // let mut into_iter = vec2.into_iter();
    //
    // // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    // println!("Find 2 in vec1: {:?}\n", iter.find(|&&x| x == "two"));
    // // A reference to what is yielded is `&i32`. Destructure to `i32`.
    // println!("Find 2 in vec2: {:?}\n", into_iter.find(|&x| x == "two"));
    //
    // let vec1 = vec![&1, &2, &3];
    // let vec2 = vec![&4, &5, &6];
    //
    // let mut iter = vec1.iter();
    // let mut into_iter = vec2.into_iter();
    //
    // // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    // println!("Find 2 in vec1: {:?}\n", iter.find(|&&x| x == &1));
    // // A reference to what is yielded is `&i32`. Destructure to `i32`.
    // println!("Find 2 in vec2: {:?}\n", into_iter.find(|&x| x == &2));
    //
    // let vec1 = vec!["one".to_owned(), "two".to_owned(), "three".to_owned()];
    // let vec2 = vec!["four".to_owned(), "five".to_owned(), "six".to_owned()];
    //
    // let mut iter = vec1.iter();
    // let mut into_iter = vec2.into_iter();
    //
    // // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    // println!(
    //     "Find 2 in vec1: {:?}\n",
    //     iter.find(|x| x == &&"three".to_owned())
    // );
    // // A reference to what is yielded is `&i32`. Destructure to `i32`.
    // println!(
    //     "Find 2 in vec2: {:?}\n",
    //     into_iter.find(|x| x == &"six".to_owned())
    // );
}

fn use_names_for_something_else(_names: Vec<&str>) {}

fn vectorized(v: Vec<String>) {
    for s in &v {
        println!("{}\n", s);
    }
    println!("{:?}\n", v);

    let lengths: Vec<usize> = v.iter().map(|time| time.len()).collect();
    println!("{:?}\n", lengths);
    println!("{:?}\n", v);

    let lengths: Vec<usize> = v.into_iter().map(|time| time.len()).collect();
    println!("{:?}\n", lengths);
    //    println!("{:?}\n", v);
}

fn vectorized_ref(rv: &Vec<String>) {
    for s in rv {
        println!("{}\n", s);
    }
    println!("{:?}\n", rv);

    let lengths: Vec<usize> = rv.iter().map(|time| time.len()).collect();
    println!("{:?}\n", lengths);
    println!("{:?}\n", rv);

    let lengths: Vec<usize> = rv.into_iter().map(|time| time.len()).collect();
    println!("{:?}\n", lengths);
    println!("{:?}\n", rv);
}

fn vectorized_ref_mute(rvm: &mut Vec<String>) {
    rvm.iter_mut()
        .map(|time| time.push_str(", ericky!"))
        .collect::<Vec<()>>();
}
