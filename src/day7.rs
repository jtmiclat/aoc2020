use std::collections::HashMap;
use std::fs;

fn get_color(text: &str) -> Option<String> {
    let t = text.replace(".", "");
    if t == "no other bags" {
        return None;
    } else {
        return Some(t.split_whitespace().collect::<Vec<&str>>()[1..3].join(" "));
    }
}

fn get_color_and_number(text: &str) -> Option<(i32, String)> {
    let t = text.replace(".", "");
    if t == "no other bags" {
        return None;
    } else {
        let v = t.split_whitespace().collect::<Vec<&str>>();
        return Some((v[0].parse().unwrap(), v[1..3].join(" ")));
    }
}

fn contains_shiny_gold(map: &HashMap<String, Vec<String>>, key: String) -> bool {
    let containing_bags = map.get(&key).unwrap();
    if containing_bags.contains(&"shiny gold".to_string()) {
        return true;
    } else if containing_bags.len() == 0 {
        return false;
    } else {
        return containing_bags
            .iter()
            .map(|x| contains_shiny_gold(map, x.clone()))
            .any(|x| x);
    }
}
fn total_bags(map: &HashMap<String, Vec<(i32, String)>>, key: String) -> i32 {
    let containing_bags = map.get(&key).unwrap();

    if containing_bags.len() == 0 {
        return 0;
    } else {
        println!("{:?}", containing_bags);
        return containing_bags
            .iter()
            .map(|t| t.0 * (total_bags(map, t.1.clone()) + 1))
            .sum();
    }
}
pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut mappings: HashMap<String, Vec<String>> = HashMap::new();
    for line in file.lines() {
        let s: Vec<&str> = line.split("contain").collect();
        let source_bag = s[0].replace(" bags", "").trim().to_string();
        let bags: Vec<String> = s[1]
            .split(",")
            .map(|n| get_color(n.trim()))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        mappings.insert(source_bag, bags);
    }
    let mut bags_num: i32 = 0;
    for key in mappings.keys() {
        if contains_shiny_gold(&mappings, key.clone()) {
            bags_num += 1;
        }
    }
    println!("{}", bags_num);
}
pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut mappings: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for line in file.lines() {
        let s: Vec<&str> = line.split("contain").collect();
        let source_bag = s[0].replace(" bags", "").trim().to_string();
        let bags: Vec<(i32, String)> = s[1]
            .split(",")
            .map(|n| get_color_and_number(n.trim()))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        mappings.insert(source_bag, bags);
    }
    let inner = total_bags(&mappings, "shiny gold".to_string());
    println!("{}", inner)
}
