use std::fs;

pub fn solve_a(filename: &str) {
    let foo: String = fs::read_to_string(filename).expect("Error opening file");
    let numbers: Vec<i32> = foo
        .split_whitespace()
        .map(|n| n.parse().expect("Error parsing number"))
        .collect();
    for num1 in &numbers {
        for num2 in &numbers {
            if (num1 + num2 == 2020) && (num1 != num2) {
                println!("Found numbers adding to 2020. {} and {}", num1, num2);
                println!("{}", num1 * num2);
                return;
            }
        }
    }
}
pub fn solve_b(filename: &str) {
    let foo: String = fs::read_to_string(filename).expect("Error opening file");
    let numbers: Vec<i32> = foo
        .split_whitespace()
        .map(|n| n.parse().expect("Error parsing number"))
        .collect();
    for num1 in &numbers {
        for num2 in &numbers {
            for num3 in &numbers {
                if (num1 + num2 + num3 == 2020)
                    && (num1 != num2)
                    && (num2 != num3)
                    && (num1 != num3)
                {
                    println!(
                        "Found numbers adding to 2020. {}, {}, and {}",
                        num1, num2, num3
                    );
                    println!("{}", num1 * num2);
                    return;
                }
            }
        }
    }
}
