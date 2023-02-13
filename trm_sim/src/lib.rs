//! A multi-tape turing machine simulator
//!
//! Input Model:
//!
//! - JSON
//! - TOML
//! - YAML
//!
//! Output:
//!
//! - History of the machine
//! - Final state of the machine
//!
//! Input Format:
//!
//! ```json
//! {
//!    "states": [
//!       {
//!         "name": str,
//!         "start": boolean?,
//!         "final": boolean?,
//!         "transitions": [
//!          {
//!            "cons": str,
//!            "prod": str,
//!            "move": str,
//!            "next": str,
//!         },
//!         ...
//!        ]
//!      },
//!     ...
//!   ]
//! }
//! ```
//!
//!
//! # Example
//!
//! A simple example of a turing machine that accepts the language of all strings,
//! and outputs the same string with all 0s replaced with 1s.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use trm_sim::trm::Machine;
//! let model = r#"
//! [[state]]
//! name = "q0"
//! start = true
//!
//! [[state.trans]]
//! cons = "*"
//! prod = "1"
//! move = "R"
//! next = "q0"
//! [[state.trans]]
//! cons = "_"
//! prod = " "
//! move = "L"
//! next = "q1"
//!
//! [[state]]
//! name = "q1"
//! final = true
//!
//! "#;
//! let mut machine = Machine::new(model, "toml")?;
//! machine.input("001100");
//! machine.run()?;
//! let id = machine.identifier();
//! assert_eq!(id.tape[0].tape, "111111");
//! assert_eq!(id.current_state, "q1");
//! # Ok(())
//! # }
//! ```
//!

pub mod trm;
