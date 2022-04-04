use std::io;

fn main() {

    let mut tempreture = String::new();
    
    println!("please input the celsius number which should be converted to Fahrenheit: ");

    io::stdin().read_line(&mut tempreture).expect("failed to read line");

    let mut tempreture : u32 = tempreture.trim().parse().expect("please type a number!");

    tempreture = (tempreture * 9/5) + 32;

    println!("The temperature in Fahrenheit is: {}°", tempreture);
}
