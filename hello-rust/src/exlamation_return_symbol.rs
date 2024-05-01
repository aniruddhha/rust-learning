/**
 * In Rust, the ! symbol represents the "never" type, also known as the "bottom" type. 
 * This type is used to indicate that a function never returns normally, 
 * meaning it either loops indefinitely, panics, or exits the program.
 * 
 */
pub fn print_okay() ->! {
    println!("🤔 What is ! mark in return type ?");
    println!("🚀 In Rust, the ! symbol represents the 'never' type, also known as the 'bottom' type. This type is used to indicate that a function never returns normally, meaning it either loops indefinitely, panics, or exits the program. ");
    loop { }
    //panic!("expr")
    // std::process::exit(0);
} 