use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author = "Crispr", about = "A simple cli calculator")]

struct Args{
    /// The temperature in celcius which should be converted to fahrenheit or kelvin
    celsius: f64,
    /// The unit to convert to. Can be either fahrenheit (f) or kelvin (k)
    unit: char,
}

fn main() {
    let args = Args::parse();
    
    match args.unit {
        'k' => {
            let kelvin = args.celsius - 273.15;
            println!("The temperature in kelvin is {}°", kelvin);
        },

        'f' => {
            let fahrenheit = (args.celsius * 9.0/5.0) + 32.0;
            println!("The temperature in fahrenheit is {}°", fahrenheit);
        },

        _ => eprintln!("Pleae input the unit. 'k' for kelvin, 'f' for fahrenheit"),
    };


}