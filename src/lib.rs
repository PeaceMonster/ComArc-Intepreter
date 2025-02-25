use wasm_bindgen::prelude::*;

mod machine;

use machine::{Machine, ProgramError};

#[wasm_bindgen]
pub fn create_machine() -> Machine {
    Machine::new()
}

#[wasm_bindgen]
pub fn get_current_instruction(machine: &Machine) -> String {
    machine.get_current_instruction()
}

#[wasm_bindgen]
pub fn get_string_registers(machine: &Machine) -> String {
    machine.get_string_registers()
}

#[wasm_bindgen]
pub fn step_machine(machine: &mut Machine) -> Result<(), ProgramError> {
    machine.step()
}
