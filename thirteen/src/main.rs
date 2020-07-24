use std::thread;
use std::time::Duration;



fn generate_workout(intensity: u32, random_number: u32) {
    let sec = |num| {
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

 