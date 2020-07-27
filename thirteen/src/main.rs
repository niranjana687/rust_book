use std::thread;
use std::time::Duration;

// fn main() {
//     let test = 4 ;
//     if test > 3 {
//   }
  
//   }

fn generate_workout(intensity: u32, random_number: u32) {
    let sec = |num: u32| -> u32 {
        println!("Calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups!", 
            sec(intensity)
        );
        println!(
            "Next, do {} situps", 
            sec(intensity)    
        );

    }else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Run for {} minutes!", sec(intensity));
        }
    }
}

fn main() {
    let intensity  = 8;
    let random_number = 5;
   generate_workout(intensity, random_number);
   fn add_v1(a : i32) -> i32 {
       a + 1 
   }
   //closure type inference and annotation
   let add_v2 = |a| {a + 1 };
   let add_v3 = |a: i32| -> i32 { a + 1 };
   let add_v4 =|a| a + 1;
   let v1 = add_v1(1);
   let v2 = add_v2(1);
   let v3 = add_v3(1);
   let v4 = add_v4(1);
   println!("{} {} {} {} ", v1, v2, v3 , v4);
}



 