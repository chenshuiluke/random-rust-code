fn main() {
    println!("Hello, world!");
    another_function(3,4);
    /*
        Rust has statements and expressions
        Statements return no value. An example would be a let statement: let x = (let y = 3);
        The above will fail because the let statement returns no value, so there is nothing for x to bind to
    */

    /*
        Expressions return a value.
        Calling a function is an expression
        the '7' in let x = 7 is an expression which evaluates to 7
        Calling a macro is an expression
        The block that is used to create new scopes {} is also an expression
    */

}
fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
