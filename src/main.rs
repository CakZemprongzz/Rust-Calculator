use std::io;

fn main() {
    println!("Please input the number and operation such as 1 + 2.");

    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<&str> = input.trim().split_whitespace().collect();

    if numbers.len() != 3 {
        print!("NANI")
    }else{
        let number_1: i32 = numbers[0].parse().unwrap();
        let operation = numbers[1];
        let number_2: i32 = numbers[2].parse().unwrap();
        let result = match operation {
            "+" => number_1 + number_2,
            "-" => number_1 - number_2,
            "*" => number_1 * number_2,
            "/" => {
                if number_2 == 0 {
                    println!("Cannot divide by zero!");
                    return;
                }
                number_1 / number_2
            }
            _ => {
                println!("Unknown operator: {}", operation);
                return;
            }
        };
        
        println!("Result: {} {} {} = {}", number_1, operation, number_2, result);
    }

}