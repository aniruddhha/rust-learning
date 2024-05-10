pub fn closure_demo() {

    let cl1: fn(x: u32) -> u32 = |x: u32| {
        12
    };

    cl1(10);
}

pub fn closure_demo_1() {
    let u = vec![1, 2, 3];
    let v = u.iter().map(|&x| x +1).collect::<Vec<_>>();
    println!("{:?}",v); 
    println!("{:?}",u);
}