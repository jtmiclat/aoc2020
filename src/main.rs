use clap::clap_app;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jt Miclat <jtmiclat@gmail.com>")
        (about: "Advent of Code 2021")
        (@setting SubcommandRequiredElseHelp)
        (@setting ColoredHelp)
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand day1a =>
            (about: "Solve for the 1st problem of day 1")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day1b =>
            (about: "Solve for the 2nd problem of day 1")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day2a =>
            (about: "Solve for the 1st problem of day 2")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day2b =>
            (about: "Solve for the 2nd problem of day 2")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day3a =>
            (about: "Solve for the 1st problem of day 3")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day3b =>
            (about: "Solve for the 2nd problem of day 3")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day4a =>
            (about: "Solve for the 1st problem of day 4")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day4b =>
            (about: "Solve for the 2nd problem of day 4")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day5a =>
            (about: "Solve for the 1st problem of day 5")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
        (@subcommand day5b =>
            (about: "Solve for the 2nd problem of day 5")
            (version: "1.0")
            (@arg INPUT: +required "Sets the input file to use")
        )
    )
    .get_matches();
    match matches.subcommand() {
        Some(("day1a", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day1::solve_a(input);
            }
        }
        Some(("day1b", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day1::solve_b(input);
            }
        }
        Some(("day2a", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day2::solve_a(input);
            }
        }
        Some(("day2b", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day2::solve_b(input);
            }
        }
        Some(("day3a", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day3::solve_a(input);
            }
        }
        Some(("day3b", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day3::solve_b(input);
            }
        }
        Some(("day4a", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day4::solve_a(input);
            }
        }
        Some(("day4b", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day4::solve_b(input);
            }
        }
        Some(("day5a", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day5::solve_a(input);
            }
        }
        Some(("day5b", sub_m)) => {
            if let Some(input) = sub_m.value_of("INPUT") {
                day5::solve_b(input);
            }
        }
        _ => {}
    }
}
