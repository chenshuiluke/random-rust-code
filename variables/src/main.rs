const MAX_POINTS:u32 = 100_000;
fn main() {
    let mut x = 5;

    println!("The value of x is {}", x);
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);

    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("The number of spaces is: {}", spaces);
}
