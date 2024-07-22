struct Point<T, U> {
    x: T,
    y: U,
}

// Can swap generics with other generics without issue
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let number_list = vec![34, 58, 78, 67, 14];

    let mut largest = number_list[0];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let number_list = vec!['s', 'f', 'u', 'q', 'b'];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);
}

// Type(can be any variable) is a generic type that allows a function to take in different and/or return data types 
fn get_largest<Type: PartialOrd + Copy>(number_list: Vec<Type>) -> Type {
    let mut largest = number_list[0];    
    for number in number_list {
        if number > largest{
            largest = number;
        }
    }
    largest
}