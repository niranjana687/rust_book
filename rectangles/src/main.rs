#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area_method (&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) ->bool {
         self.width > other.width && self.height > other.height

    }
}

fn main() {
    let h = 33;
    let w = 55;

    println!("Area is: {}", area(h,w));

//tuple version
    let rect1 = (44,78);

    println!("Area is: {}", area_rec(rect1));

//struct version
    let rect2 = Rectangle{
        height: 44,
        width: 56,
    };

    println!("The rectangle is {:#?}", rect2);

    println!("Area is: {}",area_struct_rec(&rect2));

    let rect3 = Rectangle{
        ..rect2
    };

    let rect4 = Rectangle{
        ..rect3
    };

    println!("{}", rect2.can_hold(&rect3));
    println!("{}", rect2.can_hold(&rect4));

//impl version

    println!("The area is:{}", rect2.area_method());
}

fn area (h:u32, w:u32) ->u32 {
    h*w
}

fn area_rec (dimensions: (u32,u32)) -> u32 {
    dimensions.1 * dimensions.0
}

fn area_struct_rec (rectangle: &Rectangle) ->u32 {
    rectangle.height * rectangle.width
}