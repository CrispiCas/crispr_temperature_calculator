use clap::Parser;

#[derive(Parser ,Debug)]
#[clap(version, author = "Crispr", about = "A simple cli calculator")]

struct Args{
    /// The temperature in celsius which should be converted to Fahrenheit
    #[clap(short, long)]
    temp: u16,
}

fn main() {
    let args = Args::parse();

    let temp = (args.temp * 9/5) + 32;
    println!("The temprature in Fahrenheit is {}°", temp);
}
