use std::io;

fn main() {
    // Cannot change value or data type
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);

    println!("Hello, world!");

    // needs mut in order to change the value
    let x: i32 = 4;
    // let mut x: i32 = 4;
    // x = 5;
    println!("x is: {}", x);

    // Different scope, so the it does not actually change the variable being used if calleld again outside the scope
    // Can use prexisting variables in the scope if desired
    {
        let x: i32 = 2;
        println!("x is: {}", x);
    }

    // Can change the value of a variable without mut if let is used again
    let x: i32 = x + 1;
    println!("x is: {}", x);

    // y will be made the same type as x
    let y = x;
    println!("x is: {} y is: {}", x, y);

    let x: &str = "hello";
    println!("x is: {}", x);

    let no: bool = false;
    println!("{}", no);

    let floater: f64 = 10.85;
    println!("{}", floater);

    let letter: char = 'a';
    println!("{}", letter);

    let mut tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);
    tup.0 = 5;
    println!("{}", tup.0);

    // arrays need to be initialized, cannot be declared or left empty 
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[4]);
    arr[4] = 8;
    println!("{}", arr[4]);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    // Can recast data types to match other data types, though with numbers should always convert to the higher bit size to avoid overflow
    let a: i32 = 250;
    let b: i64 = 10;
    let c = a as i64 / b;
    println!("{}", c);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input);

    // () ! && || is the order of operations with conditions

    test_making_functions();

    let x: i32 = 5;
    let result = add_two_numbers(x, y);
    print!("The sume is {}", result);

}

// Functions can be placed above or below main, order does not matter
fn test_making_functions(){
    println!("test succesful!");
}

// return type is listed after the paramaeters and before the {} and denoted with a ->
fn add_two_numbers(x: i32, y: i32) -> i32 {
    //print!("The sume is {}", x + y);
    x + y 
    // return x + y; also works
}