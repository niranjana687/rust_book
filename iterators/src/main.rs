#[derive(PartialEq, Debug)]
 use std::iter::Iterator;
struct Shoe {
    size: u8,
    style: String,
}

fn shoes_in_my_size(shoe_size: Vec<Shoe>, my_size: u8) -> Vec<Shoe> {
    shoe_size.into_iter().filter(|s| s.size == my_size).collect()
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn skip(&mut self, c: u8) -> Option<Self::Item>;
}

struct Counter {
    count: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count) 
        } else {
            None
        }
    }

    fn skip(&mut self, val: u8) -> Option<Self::Item> {
        if self.count < 5 {
            if self.count == val {
                Some(self.count + 1)
            }else {
                Some(self.count)
            }
        } else {
            None
        }
        

    }
}

fn main() {
    let v1 = vec![2,3,4,56,6];

    let v1_iter = v1.iter();
    

    for val in v1_iter {
        println!("{}", val);
    }

    for val in v1.iter() {
        println!("or {}", val);
    }

    for val in v1 {
        println!("will {}", val);
    }

    let v2 = vec![1,2,3];
    
    let v2_coll = v2.iter().map(|x| x*2 );
    for i in v2_coll {
        println!("*{}", i);
    }

    let v4 = vec![1,2,3];

    let v4_iter: Vec<_> = v4.iter().map(|x| x + 2).collect();

    assert_eq!(vec![3,4,5], v4_iter);

    let c = Counter::new();

    println!("{}", c.count);

    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn test_iterator() {
        let v = vec![1,2,3];

        let mut vec = v.iter();

        assert_eq!(vec.next(), Some(&1));
        assert_eq! (vec.next(), Some(&2));
        assert_eq! (vec.next(), Some(&3));
        assert_eq! (vec.next(), None);

        let v_iter = v.iter();
        let v_sum: i32  = v_iter.sum();
    
        let v2 = vec![1,2,3,4,5];
        let mut v2_map = v2.iter().map(|x| x+1);
        
        assert_eq!(v2_map.next(), Some(2));
        assert_eq!(v2_map.next(), Some(3));
        assert_eq!(v2_map.next(), Some(4));
        assert_eq!(v2_map.next(), Some(5));
        assert_eq!(v2_map.next(), Some(6));
        assert_eq!(v2_map.next(), None);

    }

    #[test]
    fn iter_sum() {
        let v3 = vec![1,2,3];

        let v3_iter = v3.iter();

        let total: i32 = v3_iter.sum();

        assert_eq!(total, 6);

    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 12,
                style: String::from("sketcher"),
            },
            Shoe{
                size: 14,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 12);

        assert_eq!(vec![
            Shoe{
                size: 12,
                style: String::from("sketcher"),
            },
        ], in_my_size);
    }

    #[test]
    fn calling_next_directly() {
        let mut c = Counter::new();

        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), Some(4));
        assert_eq!(c.next(), Some(5));
        assert_eq!(c.next(), None);
       
    }

    #[test]
    fn using_other_iterator_trait() {
        let sum = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|a,b| a*b)
            .filter(|x| x%3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
