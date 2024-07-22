enum IpAddressKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message{
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn some_function(){
        println!("Words words words");
    }
}

struct IpAddress{
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let localhost = IpAddressKind::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; 

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // unwrap gives access to the value in y wrapped in Some and the or gives the option of a default if no value exists in y
    let sum = x + y.unwrap_or(0);
}

fn route(ip_kind: IpAddressKind) {

}