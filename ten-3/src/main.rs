fn main() {
    let r;
    
        let z = 5;
        r = &z;
    
    println!("{}", r);

    let s1 = String::from("abcde");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }
    else {
        y
    }
}
