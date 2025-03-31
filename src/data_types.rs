fn main() {
    // Data types in rust have two subsets which are scalar and compound data types
    //Scalar types reps a single value
    //Rust has four primary scalar types which are: integers, floating point numbers, Booleans and characters.
    //Rust has two primituve compound types tuples and arrays
    // It is worthy to note that rust is statically type language just like Typescript I must say!
    // A tuple without values is called a unit
    let guess: u32 = "42".parse().expect("Not a number");
    let z_snake_name: char = 'a';
    let tup: (u32, &str, u16, f32, i8, char, bool) = (300, "I am great", 32, 400.4, -3, 'a', true);
    let (a, b, c, d, e, f, g) = tup;
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    print!("This is as a parsed number {guess}");
    print!("This is as a character {z_snake_name}");
    print!("This is an unsigned 32 bit integer {a}");
    print!("This is a string {b}");
    print!("This is an unsigned 16 bits integer {c}");
    print!("This is a 32 bit floating point number {d}");
    print!("This is a signed integer{e}");
    print!("This is a character {f}");
    print!("This is a boolean {g}");
}
