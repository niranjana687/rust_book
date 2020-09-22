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
       1 => println!("one"),
       2 => println!("two"),
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
}
