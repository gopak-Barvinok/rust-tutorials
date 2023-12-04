// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number");
// }

// fn main() {
//     let x = 2.0;

//     let y: f32 = 3.0;
// }

// fn main() {
//     let sum = 5 + 10;

//     let difference = 95.5 - 4.3;

//     let product = 4 * 30;

//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3;

//     let remainder = 43 % 5;
// }

// fn main() {
//     let t = true;

//     let f: bool = false;
// }

// fn main() {
//     let c = 'z';
//     let z: char = 'Z';
//     let heart_eyed_cat = '😻'
// }

// fn main() {
//     // let tup: (i32, i64, u8) = (500, 6.4, 1);

//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     /* 
//         Сначала создаётся шаблон let, затем берётся tup
//         и превращается в три отдельные переменные (z, y, z)
    
//     */

//     println!("The value of y is: {y}");
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// fn main() {
//     let a = [1,2,3,4,5];
// }

// fn main() {
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//     "August", "September", "October", "November", "December"];
// }

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     let a = [3; 5];

// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}