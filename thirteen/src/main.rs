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
}

 