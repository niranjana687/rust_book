enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move {x: i32,y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> String{
        self.Write(std::string::String::from("Niranjana is so cool"));
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Option<T> {
    Some(T),
    None,
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Option(5);
    let some_string = Option("Helloooooo");
    let absent_number: Option<i32> =None;
    
}


