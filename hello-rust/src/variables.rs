pub fn variables_demo() {
    let a = 10;
    println!("{a}");
    // a = 90; error
    let mut b = 40;
    println!("{b}");
    b = 90;
    println!("{b}")
}

pub fn constants_demo() {
    const ONE_LAKH: u32 = 1_00_000;
    println!("One Lakh {ONE_LAKH}");
}

pub fn shadowing_demo() {

    let a = 5;

    let a = a + 1; 
    {
        let a = a * 2;
        println!("Inner a {a}");
    }
    println!("Outer a {a}");
}