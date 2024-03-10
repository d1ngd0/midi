mod errors;

pub use errors::Error;
use midir::{MidiInput, MidiOutput};

// MidiResult is a shortcut for any Midi Errors
type MidiResult<T> = Result<T, Error>;

// list will return a list of outputs available to the user
pub fn list_outputs() -> MidiResult<Vec<String>> {
    let out = MidiOutput::new("out")?;
    let outputs = out.ports();
    let mut list = vec![];

    for p in outputs.iter() {
        list.push(out.port_name(p)?);
    }

    Ok(list)
}

pub fn list_inputs() -> MidiResult<Vec<String>> {
    let ins = MidiInput::new("in")?;
    let inputs = ins.ports();
    let mut list = vec![];

    for p in inputs.iter() {
        list.push(ins.port_name(p)?);
    }

    Ok(list)
}
