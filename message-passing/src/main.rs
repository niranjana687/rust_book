use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;


fn main() {
   let (tx, rx) = mpsc::channel();

   thread::spawn(move || {
       let val = vec![String::from("hi"),
                    String::from("from"),
                    String::from("Niranjana"),
       ];
       for i in val {
        
       tx.send(i).unwrap();
       thread::sleep(Duration::from_secs(1));
       }
       
       let mut line = String::new();
       io::stdin().read_line(&mut line).unwrap();
       let line = line.trim().parse::<i32>().expect("invalid input");
       
       
   });

   for received in rx {
       println!("Got :{}", received);
   }
}
