/*#[derive(Debug)]
enum KlState {
    Calicut,
    Kochi,
    Kannur,
}

enum Coin {
   Ruppee,
   Paisa(KlState),
}

def value_in_paisa(coin: Coin) -> u32 {
    match coin {

        Coin::Ruppee => 400,

    }
        Coin::Paisa(state) =>{
            println!{"State quarter from {:?}", state}

        }
    }
}

fn main() {
    let value1 = Coin
} */
enum Option<T> {
    Some(T),
    None,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    
}
let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
fn main() {
    



}
