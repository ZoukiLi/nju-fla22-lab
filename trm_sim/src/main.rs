use trm_sim::trm::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the model from a file
    let model_str = std::fs::read_to_string("../turing-programs/trivial_trm.toml").unwrap();
    let mut machine = Machine::new(&model_str, "toml").unwrap();
    machine.input("bsd");
    machine.run()?;

    let id = machine.identifier();
    println!("Current state: {}", id.current_state);
    println!("Tape: {}", id.tape[0].tape);
    Ok(())
}
