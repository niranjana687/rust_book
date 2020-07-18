fn main() {
    let r;
    
        let z = 5;
        r = &z;
    
    println!("{}", r);

    let s1 = String::from("abcde");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("{}", result);


    let string = String::from("I am so happy. I want to fly");
    let first_sen = string.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt  {
        part : first_sen,

    };
    println!("{}", i.part);
    let word = first_word(first_sen);


}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }
    else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s :&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }


    }
    &s[..]
}