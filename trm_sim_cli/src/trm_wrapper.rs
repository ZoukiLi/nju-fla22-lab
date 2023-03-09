use trm_sim::trm;
use trm_sim::trm::MachineIdentifier;

pub struct MachineWrapper<Formatter: MachineIdentifierFormatter> {
    trm: trm::Machine,
    formatter: Formatter,
}

impl MachineWrapper<DefaultMachineIdentifierFormatter> {
    pub fn from_file(path: &str, ext: Option<&str>) -> Result<Self, String> {
        let ext = ext.or(path.split('.').last()).ok_or("No extension provided")?;
        let model_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let trm = trm::Machine::new(&model_str, ext).map_err(|e| e.to_string())?;
        Ok(Self { trm, formatter: DefaultMachineIdentifierFormatter })
    }

    pub fn run(&mut self, input: &str, verbose: bool) -> Result<String, String> {
        self.trm.reset();
        self.trm.input(input);
        let mut s = String::new();
        if !verbose {
            self.trm.run().map_err(|e| e.to_string())?;
            s.push_str(self.formatter.format(self.trm.identifier()).as_str());
        } else {
            while !self.trm.run_once().map_err(|e| e.to_string())? {
                s.push_str(self.formatter.format(self.trm.identifier()).as_str());
            }
        }

        Ok(s)
    }
}

pub trait MachineIdentifierFormatter {
    fn format(&self, id: MachineIdentifier) -> String;
}

pub struct DefaultMachineIdentifierFormatter;

impl MachineIdentifierFormatter for DefaultMachineIdentifierFormatter {
    fn format(&self, id: MachineIdentifier) -> String {
        let state = id.current_state;
        let tapes = id.tape;
        let mut s = String::new();
        s.push_str(format!("State: {}\n", state).as_str());
        for (i, tape) in tapes.iter().enumerate() {
            s.push_str(format!("Tape {}: {}\n", i, tape.tape).as_str());
            s.push_str(format!("Head {}: {}\n", i, tape.head).as_str());
            s.push_str(format!("Range ({}..{})\n", tape.range.start, tape.range.end).as_str());
        }
        s
    }
}