fn main() {
    //loop
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("Result: {}", result);
//while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1; 
    }
    println!("LIFTOFF!");

//while 2
   let array = ["amma", "acha", "nimmu", "ammu"];
   let mut index = 0;
   while index < 4{
       println!("Array[{}] = {}", index, array[index]);
       index += 1;
   }
//for 
   for element in array.iter() {
    println!(" {}", element);
   }
}
