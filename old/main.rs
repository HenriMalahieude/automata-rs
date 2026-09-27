mod grid;

use std::io;
use crate::grid::grid::{Dim1SymKernel, World};

fn dump_grid(world: &World<bool>) {
    let mut out = String::new();
    for i in 0..world.squares.len() {
        out.push_str(if world.squares[i] { "\u{2588}" } else { " " });
    }
    println!("{out}");
}

fn dump_rules(world: &World<bool>) {
    for i in 0..world.rules.len() {
        let mut out = String::new();
        out.push_str(&format!("\tRule {i}: "));
        for j in 0..world.rules[i].kernel.len() {
            out.push_str(if world.rules[i].kernel[j] { "\u{2588}" } else { "_" });
        }
        let result = world.rules[i].result;
        out.push_str(&format!("-> {result}"));
        println!("{out}");
    }
}

fn main() {
    let mut automata: World<bool> = Dim1SymKernel::new(128, 2);
    automata.squares[64] = true; //set middle to true

    println!("Linear Automata, Ruleset {0}", automata.world_id);
    dump_rules(&automata);
    dump_grid(&automata);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        print!("\x1B[1A\x1B[2K"); //Place cursor back up to replace the newline

        if (input.trim().to_lowercase() == "q") { break }
        if (input.trim().to_lowercase() == "r") { println!("Linear Automata, Ruleset {0}", automata.world_id) }

        automata.step();
        dump_grid(&automata);
    };
}
