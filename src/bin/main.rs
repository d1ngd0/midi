use core::panic;

use clap::{arg, command, Arg, ArgAction, ArgMatches, Command};
use midi::Error;
use midir::{MidiInput, MidiOutput};
use tokio::{signal, task::JoinSet};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let m = command!()
        .subcommand(
            Command::new("list")
                .about("list all the input and output devices available to you")
                .alias("l")
                .arg(arg!(-i --input <INPUT>).action(ArgAction::SetTrue))
                .arg(arg!(-o --output <OUTPUT>).action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("test")
                .about("Send a test sound to the output specified")
                .arg(Arg::new("outputs").action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("read")
                .about("Read the incoming midi packets")
                .arg(Arg::new("inputs").action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("relay")
                .about("send all the data coming in from one location, out to another")
                .arg(Arg::new("input"))
                .arg(Arg::new("output")),
        )
        .get_matches();

    match m.subcommand() {
        Some(("list", cmd)) => run_list(cmd),
        Some(("test", cmd)) => run_test(cmd).await,
        Some(("read", cmd)) => run_read(cmd).await,
        Some(("relay", cmd)) => run_relay(cmd).await,
        Some((name, _)) => Err(Error::InvalidSubCommand(format!(
            "unknown subcommand {}",
            name
        ))),
        None => Err(Error::InvalidSubCommand(String::from(
            "there is no root command yet",
        ))),
    }
}

async fn run_relay(args: &ArgMatches) -> Result<(), Error> {
    let input = args
        .get_one::<String>("input")
        .ok_or(Error::InvalidInput(String::from(
            "Expected at least one output",
        )))?;

    let output = args
        .get_one::<String>("output")
        .ok_or(Error::InvalidInput(String::from(
            "Expected at least one output",
        )))?;

    let _i = midi::relay(input, output)?;

    signal::ctrl_c().await.expect("failed to listen for event");

    return Ok(());
}

async fn run_test(args: &ArgMatches) -> Result<(), Error> {
    let output = args
        .get_many::<String>("outputs")
        .ok_or(Error::InvalidInput(String::from(
            "Expected at least one output",
        )))?;

    for device in output {
        midi::test(device).await?;
    }

    Ok(())
}

async fn run_read(args: &ArgMatches) -> Result<(), Error> {
    let input = args
        .get_many::<String>("inputs")
        .ok_or(Error::InvalidInput(String::from(
            "Expected at least one input",
        )))?;

    let mut set: JoinSet<Result<(), Error>> = JoinSet::new();
    for device in input {
        let dev = device.clone();

        set.spawn(async move {
            let _conn = midi::read_device(&dev)?;
            signal::ctrl_c().await.expect("failed to listen for event");
            Ok(())
        });
    }

    while let Some(res) = set.join_next().await {
        if let Err(err) = res {
            panic!("{}", err)
        }

        if let Err(err) = res.unwrap() {
            return Err(err);
        }
    }

    Ok(())
}

fn run_list(args: &ArgMatches) -> Result<(), Error> {
    let input = *args.get_one("input").unwrap_or(&false);
    let output = *args.get_one("output").unwrap_or(&false);

    if !input && !output {
        return run_list_both();
    }

    if input {
        let midi_in = MidiInput::new("in")?;
        for i in midi::list_inputs(&midi_in)? {
            print!("{}\n", i);
        }
    }

    if output {
        let midi_out = MidiOutput::new("out")?;
        for i in midi::list_outputs(&midi_out)? {
            print!("{}\n", i);
        }
    }

    Ok(())
}

fn run_list_both() -> Result<(), Error> {
    print!("inputs:\n");
    let midi_in = MidiInput::new("in")?;
    for i in midi::list_inputs(&midi_in)? {
        print!("\t{}\n", i);
    }

    print!("outputs:\n");
    let midi_out = MidiOutput::new("out")?;
    for i in midi::list_outputs(&midi_out)? {
        print!("\t{}\n", i);
    }

    Ok(())
}
