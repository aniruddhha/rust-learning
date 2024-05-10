pub fn ownership_demo1() {
   let mut s = String::from("hello");
   s.push_str(", world");
   println!("{}", s)
}

pub fn ownership_demo2() {
    let s1 = String::from("abc");

    let s2 = &s1;
    println!("{} pqr", s1);
}

pub fn ownership_demo3() {
    
    let s1 = String::from("abc");

    fn pass_value(s : &String) {
        println!("{}", s);
    }

    pass_value(&s1);

    println!("S1 {:?}", s1);
}