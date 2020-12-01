use clap::clap_app;

mod day1;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jt Miclat <jtmiclat@gmail.com>")
        (about: "Advent of Code 2021")
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
    )
    .get_matches();
    if let Some(matches) = matches.subcommand_matches("day1a") {
        if let Some(input) = matches.value_of("INPUT") {
            day1::solve_a(input);
        }
    }
    if let Some(matches) = matches.subcommand_matches("day1b") {
        if let Some(input) = matches.value_of("INPUT") {
            day1::solve_b(input);
        }
    }
}
