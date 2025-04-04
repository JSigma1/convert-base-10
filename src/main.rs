use std::io;

trait ConvertBase {
    fn convert(&self, input: &str) -> Result<String, String>;
}

struct Base2;

impl ConvertBase for Base2 {
    fn convert(&self, input: &str) -> Result<String, String> {
        match input.parse::<u32>() {
            Ok(num) => Ok(format!("{:b}", num)), 
            Err(_) => Err("Invalid base-10 input.".to_string()),
        }
    }
}

struct Base16;

impl ConvertBase for Base16 {
    fn convert(&self, input: &str) -> Result<String, String> {
        match input.parse::<u32>() {
            Ok(num) => Ok(format!("{:X}", num)),
            Err(_) => Err("Invalid base-10 input.".to_string()),
        }
    }
}

struct ToBase10 {
    base: u32,
}

impl ConvertBase for ToBase10 {
    fn convert(&self, input: &str) -> Result<String, String> {
        match u32::from_str_radix(input, self.base) {
            Ok(num) => Ok(num.to_string()), 
            Err(_) => Err(format!("Invalid input for base {}.", self.base)),
        }
    }
}

fn main() {
    let base2 = Base2;
    let base16 = Base16;
    let base10_from_2 = ToBase10 { base: 2 };
    let base10_from_16 = ToBase10 { base: 16 };

    loop {
        println!("\nChoose an option:");
        println!("1. Convert base-10 to base-2");
        println!("2. Convert base-10 to base-16");
        println!("3. Convert base-2 to base-10");
        println!("4. Convert base-16 to base-10");
        println!("5. Exit");

        let choice = read_input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let input = read_input("Enter a base-10 number:");
                match base2.convert(&input) {
                    Ok(result) => println!("Base-2 result: {}", result),
                    Err(e) => println!("{}", e),
                }
            }
            "2" => {
                let input = read_input("Enter a base-10 number:");
                match base16.convert(&input) {
                    Ok(result) => println!("Base-16 result: {}", result),
                    Err(e) => println!("{}", e),
                }
            }
            "3" => {
                let input = read_input("Enter a base-2 number:");
                match base10_from_2.convert(&input) {
                    Ok(result) => println!("Base-10 result: {}", result),
                    Err(e) => println!("{}", e),
                }
            }
            "4" => {
                let input = read_input("Enter a base-16 number:");
                match base10_from_16.convert(&input) {
                    Ok(result) => println!("Base-10 result: {}", result),
                    Err(e) => println!("{}", e),
                }
            }
            "5" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
