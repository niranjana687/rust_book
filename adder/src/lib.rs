#[cfg(test)]
mod tests {
    use super::*;
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