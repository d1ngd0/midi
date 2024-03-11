mod errors;

use std::time::Duration;

pub use errors::Error;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use tokio::time;

const TIMING_CLOCK: u8 = 0b11111000;
const KEEP_ALIVE: u8 = 0b11111110;

// MidiResult is a shortcut for any Midi Errors
type MidiResult<T> = Result<T, Error>;

pub async fn test(output_name: &str) -> MidiResult<()> {
    println!("\nOpening connection");
    let out = MidiOutput::new(output_name)?;
    let mut conn_out = connect_output(out, output_name)?;
    println!("Connection open. Listen!");
    let _ = time::sleep(Duration::from_millis(4 * 150)).await;

    play_note(&mut conn_out, 66, 4).await?;
    play_note(&mut conn_out, 65, 3).await?;
    play_note(&mut conn_out, 63, 1).await?;
    play_note(&mut conn_out, 61, 6).await?;
    play_note(&mut conn_out, 59, 2).await?;
    play_note(&mut conn_out, 58, 4).await?;
    play_note(&mut conn_out, 56, 4).await?;
    play_note(&mut conn_out, 54, 4).await?;

    let _ = time::sleep(Duration::from_millis(150)).await;
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    Ok(())
}

async fn play_note(conn_out: &mut MidiOutputConnection, note: u8, duration: u64) -> MidiResult<()> {
    const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;
    const VELOCITY: u8 = 0x50;
    // We're ignoring errors in here
    conn_out.send(&[NOTE_ON_MSG, note, VELOCITY])?;
    time::sleep(Duration::from_millis(duration * 150)).await;
    conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY])?;
    Ok(())
}

pub fn read_device(input: &str) -> MidiResult<MidiInputConnection<()>> {
    let midi_input = MidiInput::new(input)?;
    connect_input(
        midi_input,
        input,
        move |stamp, message, _| match *message {
            [TIMING_CLOCK] => (),
            [KEEP_ALIVE] => (),
            _ => println!("{}: {:?} (len = {})", stamp, message, message.len()),
        },
        (),
    )
}

pub fn connect_input<F, T: Send>(
    input: MidiInput,
    name: &str,
    callback: F,
    data: T,
) -> MidiResult<MidiInputConnection<T>>
where
    F: FnMut(u64, &[u8], &mut T) + Send + 'static,
{
    let ports = input.ports();
    for p in ports.iter() {
        if input.port_name(p)? == name {
            return Ok(input.connect(p, name, callback, data)?);
        }
    }

    Err(Error::InvalidInput(format!(
        "could not find input with name {}",
        name
    )))
}

pub fn connect_output(out: MidiOutput, name: &str) -> MidiResult<MidiOutputConnection> {
    let ports = out.ports();
    for p in ports.iter() {
        if out.port_name(p)? == name {
            return Ok(out.connect(p, name)?);
        }
    }

    Err(Error::InvalidInput(format!(
        "could not find output with name {}",
        name
    )))
}

// list will return a list of outputs available to the user
pub fn list_outputs(output: &MidiOutput) -> MidiResult<Vec<String>> {
    let ports = output.ports();
    let mut list = vec![];

    for p in ports.iter() {
        list.push(output.port_name(p)?);
    }

    Ok(list)
}

pub fn list_inputs(input: &MidiInput) -> MidiResult<Vec<String>> {
    let ports = input.ports();
    let mut list = vec![];

    for p in ports.iter() {
        list.push(input.port_name(p)?);
    }

    Ok(list)
}

pub fn relay(input: &str, output: &str) -> MidiResult<MidiInputConnection<()>> {
    let midi_out = MidiOutput::new("output")?;
    let mut output = connect_output(midi_out, output)?;

    let midi_input = MidiInput::new("input")?;

    Ok(connect_input(
        midi_input,
        input,
        move |_, message, _| {
            let _ = output.send(message);
        },
        (),
    )?)
}
