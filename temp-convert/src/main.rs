use std::io;

// C to F: F = C*(9/5) + 32
// F to C: C = (F-32)*(5/9)
// https://elliotekj.com

fn main() {
    println!("Please insert a temperature in Fahrenheit to convert");
    
   let mut fah = String::new(); 

    io::stdin()
        .read_line(&mut fah)
        .expect("Failed to read line");

    let fah = fah.trim().parse::<f64>().unwrap();

    let cel: f64 = (fah - 32.0) * 5.0/9.0;

    println!("{} degrees in fahrenheit is {} degrees celsius", fah, cel);
}
