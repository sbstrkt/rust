//CHAPTER 3 CODE
fn main() {
use std::io;
    
//    let mut x = 5;
//    println!("The value of x is : {}", x);
//    x = 6;
//    println!("The value of x is : {}", x);
//}


//    let x = 5;
//
//    let x = x + 1;
//
//    {
//        let x = x * 2;
//        println!("The value of x in the inner scope is: {}", x);
//    }
//
//    println!("The value of x is: {}", x);
//########################################//
//    let tup = (500, 6.4, 1);
//
//    let (x, y, z) = tup;
//
//    println!("The value of y is: {}", y);
//####################################//
//    let x: (i32, f64, u8) = (500, 6.4, 1);
//
//    let five_hundred = x.0;
//
//    let six_point_four = x.1;
//
//    let one = x.2;
//#######################################//
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
    "The value of the element at index {} is: {}",
    index, element
    );
}




