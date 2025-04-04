fn main() {
    let x = 5;
    const Y: i32 = 10;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }
    println!("{}", x * 2 + Y);
    another_function(10);
}

fn another_function(x : i32) {
    println!("x is {} holy shit", x);
}