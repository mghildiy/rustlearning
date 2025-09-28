#![allow(warnings)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use rand::prelude::*;
use crate::collections::hashmapling::play_with_hashmap;
use crate::collections::stringling::play_with_string;
use crate::collections::vectorling::play_with_vectors;
use crate::errors::panicky::panicking;
use crate::generics::generics::play_with_generics;

pub mod collections;
pub mod parsing;
mod errors;
mod generics;

fn main() {
    // guessingGame();
    //experiments();
    //println!("{}", someRandomNumber(4));
    //controlFlow();
    //structs();

    //use crate::collections::examples::*;
    //vectorDemo();

    //use crate::parsing::Parser::Message;
    //let message = Message::from(String::from(""));
    //message.parse();

    //loopArray();

    //ownershipAndBorrowing();
    //lifetimes();
    //sliceDemo();
    //stringDemo();
    //function1();
    //references();

    println!("**************play with vector**************");
    play_with_vectors();
    println!("**************play with string**************");
    play_with_string();
    println!("**************play with hashmaps**************");
    play_with_hashmap();
    println!("**************play with panics**************");
    panicking();
    println!("**************play with generics**************");
    play_with_generics();
}

type Table = HashMap<String, Vec<String>>;
fn references() {
    let mut worksByArtists = Table::new();
    worksByArtists.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    worksByArtists.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    worksByArtists.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);
    display(&mut worksByArtists);
    assert_eq!(worksByArtists["Caravaggio"][0], "The Musicians!");

    let x = 4;
    let y = &x;
    assert!(*y == 4);

    let mut x = 4;
    let y = &mut x;
    *y += 3;
    //assert!(x == 7);
    //assert!(*y == 7);
    println!("{}", y);

    let r = &factorial(6);
    assert_eq!(r + 1009, 1729);

    {
        let mut r= &100;;
        {
            let x = 1;
            r = &x;
        }
        //assert_eq!(*r, 1); // bad: reads memory `x` used to occupy
    }

    let myStrings = vec!["Hello".to_string(), "World!".to_string()];
    for s in &myStrings {
        println!("String {:?} is at address {:p} with length {}", *s, s, s.len())
    }

    let myStrs = vec!["Hello", "World!"];
    for s in myStrs {
        println!("String {:?} is at address {:p} with length {}", s, s, s.len())
    }

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let numVec = vec![1,2,3,4,5];
    let first3 = &numVec[0..3];
    for num in first3 {
        println!("{} {:p}", num, num);
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

fn factorial(n: usize) -> usize {
    (1..n+1).product()
}

fn display(table: &mut Table) {
    for (artist, works) in  table {
        println!("Works by artist {}:", artist);
        for mut work in works {
            work.push('!');
            println!("               {work}");
        }
    }
}

fn function1() {
    let mut padovan = vec![1,1,1]; // allocated here
    println!("{}", padovan.capacity());
    println!("{}", padovan.len());
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
        println!("Capacity: {}", padovan.capacity());
    }
    println!("P(1..10) = {:?}", padovan);
    println!("{}", padovan.capacity());
    println!("{}", padovan.len());

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    }

    let v = vec!["liberté".to_string(), "égalité".to_string(), "fraternité".to_string()];
    for mut s in v { // v moves out
        s.push('!');
        println!("{}", s);
    }

    //println!("{}", v[0]); illegal as v already moved out

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string();

    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
}

fn stringDemo() {
    println!("It was a bright, cold day in \nApril, and \
    there were four of us—\
    more or less.");

    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("{default_win_install_path}");

    let method = b"GET"; // byte string
    for elem in method {
        println!("{elem}")
    }

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    /*for elem in oodles {
        println!("{}", elem)
    }*/
    let poodles = "􀀀_􀀀";

    println!("{}", noodles.chars().count());
    println!("{}", noodles.len());
    println!("{}", oodles.len());
    println!("{}", poodles.chars().count());
    println!("{}", poodles.len());

    let mut hw = "Hello World".to_string();
    hw.push('!');
    println!("{}", hw);

    let hwref = &hw[0..];
    assert!(hw == "Hello World!");

    assert!("peanut".contains("nut"));
}

fn sliceDemo() {
    let myArr = [1,2,3,4];
    let myVec = vec![-1,-2,-3,-4];

    printSlice(&myArr);
    printSlice(&myVec);
}

fn printSlice(mySlice: &[i32]) {
    for elem in mySlice {
        println!("{}", elem);
    }
}

fn lifetimes() {
    let mut a1 = Box::new(2);
    let mut b1 = &a1;
    //a1 = Box::new(22); //illegal assignment as value is already borrowed
    println!("{b1}");
    a1 = Box::new(22); // this would make b1 invalid, and hence end its lifecycle
    b1 = &a1; // new lifecycle starts for b1
    println!("{b1}");

    let mut x = Box::new(42);
    let mut z = &x; // 'a
    for i in 0..100 {
        println!("{}", z); // 'a
        x = Box::new(i);
        z = &x; // 'a
    }
    println!("{}", z); // 'a
}

fn ownershipAndBorrowing() {
    let mut x = 7;
    if rand::random() {
        x = 4;
    } else {
        let y = &x; // value borrowed
        //x = 4; //not allowed as value already borrowed
        let z = 3 + *y;
    }
    //println!("{z}");

    let mut a = 6;
    let b = & a;
    //let c = &mut a;
    //a = 8;
    //*c = 77;
    println!("{b}");

    let mut a1 = 6;
    let b1 = &a1;
    let b2 = &a1;
    //let c1 = &mut a1; // not allowed as immutable b1 is still in scope
    //*c1 = 66;
    println!("{b1}")

}
fn loopArray() {
    let mut numbers = Vec::new();
    let x = 1;
    numbers.push(x);
    let y = 2;
    numbers.push(y);
    let z = 3;
    numbers.push(3);
    numbers.push(4);

    for n in numbers {
        println!("Number is: {n}");
    }
    /*println!("********");
    for d in &numbers[0..] {
        println!("Number is: {d}");  
    }*/

    let x1 = 42;
    let y1 = Box::new(84);
    println!("{x1}");
    println!("{y1}");

    { 
        let z = (x1, y1);
    }
    
}

fn guessingGame() {
    println!("Let's start the game");

    let secretNumber = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new(); // a string variable whose value can be mutated
        println!("Please guess the number");
        io::stdin()
            .read_line(&mut guess) // passing reference, that is mutable
            .expect("Failed to read line");

        // rust shadows variable defined earlier with same name
        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("You can enter a number only");
                continue;
            }
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("You guessed lower number"),
            Ordering::Greater => println!("You guessed higher number"),
            Ordering::Equal => {
                println!("You are a champion");
                break;
            }
        }
    }
}

fn experiments() {
    // default integer type is i32
    let x = 5;
    // shadowing
    let x = "hello";
    // a variable is immutable by default
    let m = "immutable";
    // mutable
    let mut y = "mutable";
    // constant
    const SECONDS_IN_A_HOUR: u32 = 60 * 60 * 60;
    let z: u32 = "7".parse().expect("Not a number!");
    // default float type is f64
    let a = 2.0;
    let b: f32 = 2.0;
    let c = true;
    // tuple can contain multipke values of diff types
    let d = (500, "hello", true);
    // access tuple values with . or destructuring
    let e = d.0;
    let (_, f, g) = d;
    println!("{e}, {f}, {g}");
    // arrays contain multiple values of same type, they reside on stack
    let days = ["Sunday", "Monday", "Tuesaday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let firstDay = days[0];
    println!("{firstDay}");
    
}

fn someRandomNumber(factor: u32) -> u32 {
    rand::thread_rng().gen_range(1..=100) * factor
}

fn controlFlow() {
    let x = 7;
    let str = if x <= 7 {
        "Less than or equal to 7"
    } else {
        "Greater than 7"
    };
    println!("{str}");

    let x = 8;
    let str = if x <= 7 {
        println!("Less than or equal to 7")
    } else {
        println!("Greater than 7");
    };

    let mut counter = 0;
    let y = loop {
        counter += 1;
        if counter == 10 {
            break counter
        }
    };
    println!("Loop iterated {y} times");

}

struct User {
    active: bool,
    userName: String,
    email: String,
    signInAmount: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    // &self is short for self: &Self, where Self is alias for the type impl block is for
    // so &self is basically of type reference to Rectangle in this case
    // so we are borrowing from Self, no ownership is moved
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn canHold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    // doesnt have self as first parameter, so not associated to any instance, like static functions in java.
    fn square(dim: u32) -> Self {
        Rectangle { width: dim, length: dim }
    }
}

fn structs() {
    let mut user = User{
        active: true,
        userName: String::from("John Doe"),
        email: String::from("unknown@gmil.com"),
        signInAmount: 1
    };
    println!("{}", user.email);
     
    user.email = String::from("another@asds.com"); 

    let user1 = createFromAnotherUser(createUser(String::from("tom"), String::from("tom@sdc.com")));
    println!("{}", user1.email);

    let rect = Rectangle{width:10, length:20};
    println!("rect is {:#?}", rect);
    println!("Area of rectangle is {}", rect.area());

    let rect1 = Rectangle{width: 7, length: 10};
    println!("Can rect hold rect1: {}", if rect.canHold(&rect1) {"Yes"} else {"No"});

    let sq = Rectangle::square(20);
}

fn createUser(userName: String, email: String) -> User {
    User {
        active: true,
        userName,
        email,
        signInAmount: 1,
    }
}

fn createFromAnotherUser(user: User) -> User {
    User {
        email: String::from("anotherUser@dad.com"),
        ..user // struct update syntax. All fields, except  email, would be taken from user
    }
}


struct Point(i32, i32, i32); // tuple structs. no need for field names
fn createPoint(x: i32, y: i32, z: i32) -> Point {
    Point(x, y, z)
}

// unit like structs with no fields, for purpose of implementing traits on type having no data
struct MyUnitLikeStruct;
