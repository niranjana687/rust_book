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
        assert!(!smaller.can_hold(&larger));
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
        self.width >= other.width && self.height >= other.height
    }
}