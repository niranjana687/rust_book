#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    #[should_panic(expected = "Guess value should be in between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(5);
        assert_eq!(3, value);
    }
    #[test]
    fn greetings_contain_name() {
        let result = greetings("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, it was {}",
        result    
    );
    }
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    #[test]
    fn if_it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("Not equal to 4 "))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 44,
            height: 56,
        };
        let smaller = Rectangle {
            width: 33,
            height: 35,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));

    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn this_fails() {
    //     panic!("This test failed");
    // }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other :&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greetings(name: &str) -> String {
    format!("Hello {}!", name)
}

struct Guess {
    value : i32,
}

impl Guess {
    pub fn new(value :i32) -> Guess {
        if value < 1  {
            panic!("The guessed value should be greater than 1, got {}", value);
        } 
        else if value >100 {
            panic!("The guessed value should be less than 100, got {}", value);
        }
        Guess{ value }

    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("The value given is: {}", a);
    10
}