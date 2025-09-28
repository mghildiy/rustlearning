pub fn play_with_generics() {
    let nums = vec![10, 20, 100, 40, 77];
    println!("Largest number is {}", largest(&nums));
    let chars = vec!['a', 'b', 'c'];
    println!("Largest char is {}", largest(&chars));

    let int_point = Point {x: 1, y: 10};
    let float_point = Point {x: 1.1, y: 10.9};
    let mix_point = Point {x: 1.0, y: 1};

    println!("x: {}, y: {}", int_point.x, int_point.y);
    println!("x: {}, y: {}", float_point.x, float_point.y);
    println!("x: {}, y: {}", mix_point.x, mix_point.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T: PartialEq, U: PartialEq> {
    x: T,
    y: U
}

impl<T: PartialEq, U:PartialEq> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}