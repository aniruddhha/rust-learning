pub fn raw_pointer_demo_1() {

    let num = 5;
    let raw_ptr_1 = &num;

    let  raw_ptr_2: *const i32 = &num;

    // println!("{raw_ptr_1}, {raw_ptr_2}");
}

pub fn raw_pointer_demo_2() {

    let num = String::from("abc");
    let raw_ptr_1 = num;
    // let raw_ptr_2 = num;
    // println!("{}", num);

}