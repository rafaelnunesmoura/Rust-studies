fn main() {


    // Scalar Types

    // Integer Types
    let x = 2.0; // f64
    let y: f32 = 3.0; //  f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 20;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0

    // remainder
    let remainder = 43 % 5;


    // The Boolean Type
    let t = true;

    let f: bool = false; // with explicit type annotation


    // The Character Type
    let c = 'z';
    let z = 'z';
    let heat_eyed_cat = '😻';


    // Compound Types 

    // The Tuple Type
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of y is: {}", y);

    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;


    // The Array Type

    let a = [1,2,3,4,5];

    let months: ["January","February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December" ];

    let a [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3, 5];

    // accessing Arraey elements
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
}
