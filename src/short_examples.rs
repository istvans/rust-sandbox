
#[cfg(example01)]
fn example() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

#[cfg(example02)]
fn example() {
    const ALMA: u32 = 5;
    ALMA = 6;
}

fn example() {
    
}

fn main() {
    example();
}
