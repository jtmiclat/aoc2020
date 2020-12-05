use std::collections::HashSet;
use std::fs;
use std::i32;

pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut max_id: i32 = 0;
    for line in file.lines() {
        let colbinary = &line[0..7];
        let rowbinary = &line[7..];
        let col = i32::from_str_radix(
            &colbinary.to_string().replace("F", "0").replace("B", "1"),
            2,
        )
        .unwrap();
        let row = i32::from_str_radix(
            &rowbinary.to_string().replace("L", "0").replace("R", "1"),
            2,
        )
        .unwrap();
        let seat_id = col * 8 + row;
        if seat_id >= max_id {
            max_id = seat_id
        }
    }
    println!("Max id {}", max_id)
}
pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut id_list: Vec<i32> = vec![];
    for line in file.lines() {
        let colbinary = &line[0..7];
        let rowbinary = &line[7..];
        let col = i32::from_str_radix(
            &colbinary.to_string().replace("F", "0").replace("B", "1"),
            2,
        )
        .unwrap();
        let row = i32::from_str_radix(
            &rowbinary.to_string().replace("L", "0").replace("R", "1"),
            2,
        )
        .unwrap();
        let seat_id = col * 8 + row;
        id_list.push(seat_id)
    }

    id_list.sort();
    let min = id_list.iter().min().unwrap();
    let max = id_list.iter().max().unwrap();
    let mut a: HashSet<i32> = HashSet::new();
    for v in *min..*max {
        a.insert(v);
    }
    let b: HashSet<_> = id_list.iter().cloned().collect();
    for x in a.difference(&b) {
        println!("{}", x)
    }
}
