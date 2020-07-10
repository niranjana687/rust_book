#![allow(unused_variables)]

/*use std::io::{self, Read};
use std::fs::File;

fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

*/
//}



//fn main() {
  
/*enum Result<T, E> {
       Ok(T),
       Err(E),
   }*/

 //let f= File::open("hello.txt");

 /*let f = match f {
     Ok(file) => file,
     Err(error) => panic!("File error {:?}",error),
 };*/

 /*let f = match f {
     Ok(file) =>  file,
     Err(error) => match error.kind() {
         ErrorKind::NotFound => match File::create("hello.txt") {
             Ok(fc) => fc,
             Err(e) => panic!("Problem creating file {:?}",e),
         },
         other_error => {panic!("Problem opening the file: {:?}", other_error)}
     },
 };*/
//let f1 = File::open("hello.txt").expect("Failed to open hello.txt");
//let f2 = File::open("bye.txt").unwrap();





//}

use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;

fn main() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
    
        File::open("hello.txt")?.read_to_string(&mut s)?;
    
        Ok(s)


        
        
    }
    let home: IpAddr = "127.0.0.1".parse().unwrap()

    loop {
        let guess :i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if (guess<1 || guess> 100) {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
    
    }
}