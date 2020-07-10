fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v2: Vec<String> = Vec::new();
    let v = vec![2, 3, 4, 5, 6];
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(56);
    let third: &i32 = &v[2];
    println!("The value of third :{}", third);

    match v.get(2) {
        Some(third) => println!("The value is {}", third),
        None => println!("The vaule does not exist"),
    }

    let mut v4 = vec![3, 4, 5, 6, 6, 7];
    //let _does_not_exist = &v4[99];
    // let _does_not_exist = v4.get(99);
    //let first = &v4[0];
    v4.push(6);
    // println!("The first value is {}", first);
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v4 {
        *i *= 10;
        println!("-- {}", i);
    }

    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        Spreadsheet::Int(454),
        Spreadsheet::Float(56.345678),
        Spreadsheet::Text(String::from("Niranjana")),
    ];

    let data = "i love me";
    let mut _s = String::new();
    let mut s = data.to_string();
    println!("{}", s);
    let s2 = ", I love me.";
    s.push_str(s2);
    println!("{}", s);
    let mut me = String::from("Awe");
    me.push('e');
    println!("{}", me);

    let i = String::from("Niranjana");
    let j = String::from("is amazing");
    let p = i + &j;
    println!("{}", p);

    let m = String::from("I");
    let n = String::from("Love");
    let o = String::from("me");
    let r = m + "-" + &n + "-" + &o;
    println!("{}", r);
    let q = String::from("i");
    let t = q + &n + &o;
    println!("{}", t);
    let w = format!("{} {}", o, t);
    println!("{}", w);
    let slice = &w[3..7];
    println!("{}", slice);

    for c in slice.chars() {
        print!("-{}", c);
    }

    for b in "I am so happpy!".bytes() {
        println!("{}", b);
    }
    //hashmaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Niranjana"), 100);
    scores.insert(String::from("Ammu"), 100);

    let names = vec![String::from("Niranjana"), String::from("Ammu")];
    let age = vec![6, 1];

    let mut age: HashMap<_, _> = names.into_iter().zip(age.into_iter()).collect();

    //ownership
    for (k, v) in &scores {
        println!("{} : {}", k, v);
    }

    scores.entry(String::from("Ammu")).or_insert(99);
    scores.entry(String::from("Niranjana")).or_insert(99);

    println!("{:?}", &scores);

    let text = "hello hi you are going to be the best in the world";
    let mut fut = HashMap::new();
    for word in text.split_whitespace() {
        let count = fut.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", &fut);
}
