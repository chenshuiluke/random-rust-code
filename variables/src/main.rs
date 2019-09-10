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

    //Rust's integer types go from 8 bits to 64 bits, ie i8 to u64
    //They can also be signed or unsigned, ie i8 or u8


    //Rust has two floating point types: f32 and f64

    //Rust has the bool type to represent true and false

    //Rust has characters which are unicode
}
