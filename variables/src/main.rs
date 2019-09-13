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

    //Rust has tuples. Each item in a tuple can have a different type:
    let tup: (i32, f64, u8) = (1, 5.6, 1);

    //You can use pattern matching to access the contents of a tuple:
    let (x, y, z) = tup;

    //You can also access an item in the tuple by its element:
    let x = tup.0;

    //Rust has arrays. Unlike tuples, each element must have the same type
    //Arrays have a fixed size. They cannot grow or shrink

    let a = [1, 2, 3, 4, 5];

    //If you attempt to access an array's contents using an index that it does not contain,
    //a runtime panic will occur
    print_hello_world();
}

fn print_hello_world(){
    println!("Hello, world!");
}