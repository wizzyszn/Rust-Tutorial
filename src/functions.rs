fn main(){
    another_function();
    let five = five();
    print!("This is a return value from a function call {five}")
}

fn another_function() {
    println!("This is another function")   
}

//A function with a Return value

fn five() -> i32{
    5
}