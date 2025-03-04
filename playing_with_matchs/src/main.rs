fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // All patterns need to be placed when using match unless you use _ which is a catch all for all patterns not listed
    match x{
        Some(i) => Some(i + 1),
        //None => None
        _ => None
    }
}