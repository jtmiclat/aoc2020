use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut total: i32 = 0;
    for group in file.split("\n\n") {
        let mut answers: HashSet<char> = HashSet::new();
        for person in group.split("\n") {
            for a in person.to_string().chars() {
                answers.insert(a);
            }
        }
        total = total + answers.len() as i32;
    }
    println!("Total {}", total)
}
pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut total: i32 = 0;
    for group in file.split("\n\n") {
        let group_size = group.split("\n").count() as i32;
        let mut answers: HashMap<char, i32> = HashMap::new();
        for person in group.split("\n") {
            for a in person.to_string().chars() {
                if answers.contains_key(&a) {
                    *answers.get_mut(&a).unwrap() += 1;
                } else {
                    answers.insert(a, 1);
                }
            }
        }
        total = total
            + answers
                .iter()
                .filter(|(_, value)| value.clone() == &group_size)
                .count() as i32;
    }
    println!("Total {}", total)
}
