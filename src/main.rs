use std::io;

fn main() {

    let mut tempreture = String::new();
    
    println!("please input the celsius number: ");

    io::stdin().read_line(&mut tempreture).expect("failed to read line");

    let mut tempreture : u32 = tempreture.trim().parse().expect("please type a number!");

    tempreture = (tempreture * 9/5) + 32;

    println!("{}", tempreture);
}
