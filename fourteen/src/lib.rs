//!  # Fourteen
//!  'fourteen' is a collection of utitliteis to make few calculations convenient
///Returns the factorial of a given number
///# Example
///```
///let f = 5;
/// let answer = fourteen::factorial(f);
/// assert_eq!(120, answer);
///```
///

pub fn factorial(x: i32) -> i32 {
    let mut fact = 1;
    for i in 1..x+1 {
        fact = i * fact; 
    } fact
}