use core::option::Option;

fn main() {
   let fav_color: Option<&str> = None;
   let is_tuesday = true;
   let age: Result<u8, _> = "19".parse();
   
   if let Some(color) = fav_color {
       println!("Your Favourite color is {}", color);
   } else if is_tuesday {
       println!("Today is green day ");
   }else if let Ok(age) = age {
       if age > 30 {
           println!("Using purple as background")
       }
   }

   let mut stack = Vec::new();

   stack.push(1);
   stack.push(2);
   stack.push(3);

   while let Some(top) = stack.pop() {
       println!("{}", top);
   }

   let reg = vec!["niranjana", "global", "yet", "grounded"];

   for (index, value) in reg.iter().enumerate() {
       println!("{} :{}", index, value);
   }
//    let x = 5;
   let some_option: Option<i32> = None;
   if let x = 5 {
       println!("{}", x);
   };

   let y = 2;
   match y {
       1 | 2 => println!("one or two"),
       3 => println!("three"),
       _ => println!("anything"),
   }

   let p = Some(50);
   let q = 10;

   match p {
       Some(5) => println!("Got 5"),
       Some(q) => println!("Got {:?}", q),
       _ => println!("Got def {:?}", p),
   }

   println!("at the end p {:?} q {:?}", p, q);


   for i in 0..=5 {
       print!("{} ", i);
   }
   println!("");

   let a = 3;

   match a {
       1..=6 => println!("1 through 6"),
       _ => println!("not relevant"),
   }

   let b = 'b';

   match b {
       'a'..='j' => println!("first half of alphabets"),
       'k'..='z' => println!("second half of alphabets"),
       _ => println!("something else"),

    }

    let p = Point {
        x: 0,
        y: 19,
    };

    let Point { x:a, y: b  } = p;

    match p {
        Point { x, y: 0} => println!(" on the x axis at {}",x),
        Point {x: 0, y } => println!("one the y axis at {}", y),
        Point{ x, y} => println!("on neither axis, {}, {}",x,y),
    }

    let msg = Message::ChangeColor(Color::hsv(23, 56, 67));

    match msg {
        Message::Quit => println!("The quit varaint has not messsage to destructure"),
        Message::Move{x, y} => println!("Move point to {}, {}", x, y),
        Message::Write(text) => println!("Writes text {}", text),
        Message::ChangeColor(Color::rgb(r, g, b)) => println!("Changes color to r {}, g {}, b{}", r, g, b),
        Message::ChangeColor(Color::hsv(h, s, v)) => println!("Change color to h{}, s{}, v{}", h, s, v),
        _ => (),
    }

    let mut setting = Some(10);
    let new_setting = Some(5);

    match (setting, new_setting) {
        (Some(_), Some(_)) => {println!("Values cannot be replaced");}
        _ => {setting = new_setting;},
    }   

    println!("new val {:?}", setting);

    let number = (1,4,5,6,6);

    match number {
        (f, .., l) => {
            println!("first {} and last {}", f, l);
        }
    }

    let s = Some(String::from("Niranjana"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}",s);

    match p {
        Point{x, ..} => println!("x is {}", x),
    }

    let num = Some(7);

    match num {
        Some(x) if x < 10 => println!("less than ten x : {}", x),
        Some(x) => println!("value of x {}", x),
        None => (),
    }

    let c = 4;
    let d = false;

    match c {
        4 | 5 | 6 if d => println!("yes"),
        _ => println!("no"),
    }

    let greet = Greet::Hello{id: 5};

    match greet {
        Greet::Hello{id: id_v @3..=7,}
                                    =>println!("found in an id {}", id_v),
        Greet::Hello{id: 10..=12,} => {println!("found an another id ")}, 
        Greet::Hello{id,} => println!("anotehr id found {}", id)
    }

}

struct Point  {
    x: i32,
    y: i32,
}

enum Color {
    rgb(i32, i32, i32),
    hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Greet {
    Hello {id: i32},
}