use std::thread;
use std::time::Duration;


fn main() {
    thread::spawn(|| {
        for i in 1..10{
            println!("number from spwaned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
