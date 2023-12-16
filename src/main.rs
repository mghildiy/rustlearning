use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub mod collections;

fn main() {
    // guessingGame();
    //experiments();
    //println!("{}", someRandomNumber(4));
    //controlFlow();
    //structs();

    use crate::collections::examples::*;
    vectorDemo();
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