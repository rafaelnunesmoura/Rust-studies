fn main() {
    println!("Hello, world!");

    print_labeled_mesurement(5, 'h');
    another_function(5);
}

fn another_function(x: i32) {
    println!("the value of x is; {}.", x);
}

fn print_labeled_mesurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}