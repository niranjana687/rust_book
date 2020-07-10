struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Student {
    name: String,
    ad_no: u32,
    branch: String,
    present: bool,
}
//tuple structs 
struct Coordinate(i8, i8, i8);
struct Origin(i8, i8, i8);


fn main() {
    println!("Hello, world!");
//instance of user struct is created 
    let mut user1 = User {
        username: String::from("Niranjana"),
        email: String::from("niranjana687@gmail.com"),
        sign_in_count: 55,
        active: true,
    };
    user1.email = String::from("niranjana.mec@gmail.com");
    println!("{}", user1.username);

//creating instances from other instances with struct update syntax
    let st1 = Student{
        name: String::from("Niranjana"),
        ad_no: 8699,
        branch: String::from("CS"),
        present: true,
    };

    let st2 = Student{
        name: String::from("Nirupama Biju"),
        ad_no: 9987,
        ..st1
    };

    println!("{}", st2.branch);
//tuple struct instances
    let niranjana = Coordinate(7,-7,7);
    let o_niran = Origin(-1,1,0);
    let x_dist = niranjana.0 - o_niran.0;
    println!("{}", x_dist);

}

fn build_user(username: String, email :String) -> User{
   User{
       username: username,
       email: email,
       sign_in_count: 1,
       active: true,
   }
}

fn build_student(name: String, ad_no :u32, branch: String) -> Student{
    Student{
        name,
        ad_no,
        branch,
        present: true,
    }
}
