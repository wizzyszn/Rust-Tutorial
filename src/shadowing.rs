fn main() {
    let x = 7;
    let x = x + 1;
    let spaces = "";
    let spaces = spaces.len();
    /*
    Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name */
    {
        let x = x * 2;
        println!("This is the value of x in the inner scope: {x}");
    }
    println!("This is the value of x: {x}")
}
