fn main() {
   let s = String::from("Hello");

   takes_ownership(s);

   let x = 5;
   
   makes_copy(x);
   let s1 = String::from("Helloiiii");
   let (st, len) = get_length(s1);
   println!("{} {}", st, len);
   let mut s2 = String::from("I am going to the US");

   let ln = calculate_length(&mut s2);
   println!("length of string {} is {}", s2, ln);

   change(&mut s2);
   
   let word = first_word(&s2);
   println!("First word length is: {}", word);

   //slices
   let s3 = String::from("I love me!")

}
//user defined functions
fn takes_ownership(s :String){
    println!("{}", s);
}

fn makes_copy(x :u32) {
    println!("{}", x);
}

fn get_length(s: String) -> (String,usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(". Yesss I am goingggg");
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
   s.len()
}
struct  {}