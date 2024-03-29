#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

#[derive(Debug, PartialEq)]
struct City {
    name: String,
    lat: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name: &str = "Peter";
    let age: u8 = 27;
    let mut peter: Person = Person { name, age };

    // My attempt
    let tucson: City = City {
        name: String::from("Tucson"),
        lat: 77,
    };

    // Print debug struct
    println!("{:?}", peter);
    println!("{:?}", tucson);

    let new_city: City = City {
        name: String::from("Chicago"),
        ..tucson
    };
    println!("{:?}", new_city);

    let mut newark: City = City {
        name: "Newark".to_string(),
        lat: age,
    };
    println!("{:?}", newark);
    newark = City { lat: 88, ..newark };
    println!("{:?}", newark);
    let boston: City = City {
        name: "Boston".to_string(),
        lat: age,
    };

    // EAT added
    peter.greet();

    peter.set_name("Tony");
    peter.greet();

    println!(
        "{:?}",
        Person {
            name: "Doofy",
            age: 22,
        }
    );

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point: Point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y }: Point = point;

    let _rectangle: Rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!("{:?}", _rectangle);

    // Instantiate a unit struct
    let _nil: Nil = Nil;

    // Instantiate a tuple struct
    let pair: Pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal): Pair = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let mut ptr_v: Vec<&City> = Vec::new();
    ptr_v.push(&newark);
    ptr_v.push(&boston);
    println!("{:?}", ptr_v);

    let target: City = City {
        name: "Newark".to_string(),
        lat: 88,
    };
    assert!(ptr_v.contains(&&target));

    let mut v: Vec<&City> = Vec::new();
    v.push(&newark);
    v.push(&boston);
    println!("{:?}", v);
}
