use std::fs::File;
use std::io::prelude::*;

use machine::Machine;
use parser::parse_program;

use clap::Parser;

mod machine;

mod parser;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    // Name of file to intepret
    filename: String,

    // Turn on verbose printing
    #[arg(short, long)]
    verbose: bool,

    // Print each instruction before executing
    #[arg(short, long)]
    print_instruction: bool,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut file = File::open(cli.filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let program = parse_program(&content);
    let mut machine = Machine::new();
    machine.init_program(program);
    let mut r = Ok(());
    while r == Ok(()) {
        if cli.print_instruction {
            machine.print_current_instruction();
        }
        r = machine.step();
        if cli.verbose {
            machine.print_registers();
            println!("------");
        }
    }
    if !cli.verbose {
        machine.print_registers();
    }
    Ok(())
}
