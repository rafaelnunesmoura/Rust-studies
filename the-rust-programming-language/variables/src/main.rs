fn main() {
    let x: isize = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }
    println!("The value of x is {}", x);

    // cause an error: The error says we’re not allowed to 
    // mutate a variable’s type:
    //let spaces = "  ";
    //let spaces = spaces.len();
}
