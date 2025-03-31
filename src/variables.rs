fn main(){
    // difference between constants and variables
    // You aren't allowed to use mut with constants
    // use let for variables and const for constants
    // the type of the value the consttant will recieve must be annotated
    // constants may be set only to a constant expression and not to a result of a value that can computed only on run time
    let mut  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The new value of x is: {x}")
}