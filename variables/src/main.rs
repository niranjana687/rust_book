fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 10;
    println!("x : {}", x);
//constant
    const MAX_NUM: u32  = 10_000;
    println!("{}",MAX_NUM);
//shadowing
    let y = 5;
    let y = y + 1;
    println!("{}",y);
    let y = y * 2;
    println!("{}",y);
//spaces
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}",spaces);

//float
    let z = 2.03;
    let p: f32 = 3.4555;
    println!("{} {}", z, p);
//boolean
    let b: bool = true;
    println!("{}", b);
//char
    let juukins = "☀️";
    println!("{}", juukins);
//tuples
    let tup: (u32, i32, f32, bool) = (3, -45, 56.777, true);
    println!("{}", tup.1);
//arrays
    let array: [i32;2] = [1,2];
    println!("{}",array[1]);
//function call 
     another_function(6, 6.88);
/*hello */
// function scopes {}
    let _q = 5;
    let r = {
        let q = 3;
        q + 1
    };
    println!("{}", r);
    let e = forty();
    println!("{}", e);

    let _o = plus_one(e);
}

fn another_function(x: u64, y: f32) {
    println!("Onnulalo!{} {}",x,y);
}

fn forty() -> i8 {
    40
}

fn plus_one(x: i8) -> i8 {
     x + 1
}

