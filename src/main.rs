use std::fs::File;
use std::io::prelude::*;

use asm_virtual_machine::machine::Machine;
use asm_virtual_machine::parser::parse_program;

use clap::Parser;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    // Name of file to intepret
    filename: String,

    // Turn on verbose printing
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut file = File::open(cli.filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let program = parse_program(&content)?;
    let mut machine = Machine::new();
    machine.init_program(program);
    let mut r = Ok(());
    while r == Ok(()) {
        if cli.verbose {
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
