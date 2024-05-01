pub fn data_type_demo1() {
    let guess:u32 = "abc".parse().expect("Not A Number");
    println!("{guess}");
}

pub fn compound_type_demo2() {
    let tup = (200, 1.9, 1);
    let (x, y, z) = tup;
    println!("X {x}, Y {y}, Z {z}");
    println!(" 0th {}", tup.0)
}

pub fn arr_type() {
    // normal array
    let arr1 = [1, 2, 3, 4, 5];

    // 3 float items
    let arr2: [f32; 3] = [0.1, 0.4, 0.4];

    // value 3 would be 5 times
    let arr3 = [3; 5];

    let first = arr1[0];

    let second = arr2[2];
}