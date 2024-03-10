use clap::{arg, command, ArgAction, ArgMatches, Command};
use midi::Error;

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
        .get_matches();

    match m.subcommand() {
        Some(("list", cmd)) => run_list(cmd),
        Some((name, _)) => Err(Error::InvalidSubCommand(format!(
            "unknown subcommand {}",
            name
        ))),
        None => Err(Error::InvalidSubCommand(String::from(
            "there is no root command yet",
        ))),
    }
}

fn run_list(args: &ArgMatches) -> Result<(), Error> {
    let mut input = *args.get_one("input").unwrap_or(&false);
    let mut output = *args.get_one("output").unwrap_or(&false);

    if !input && !output {
        return run_list_both();
    }

    if input {
        let items = midi::list_inputs()?;

        for i in items.iter() {
            print!("{}\n", i);
        }
    }

    if output {
        let items = midi::list_outputs()?;

        for i in items.iter() {
            print!("{}\n", i);
        }
    }

    Ok(())
}

fn run_list_both() -> Result<(), Error> {
    print!("inputs:\n");
    let items = midi::list_inputs()?;

    for i in items.iter() {
        print!("\t{}\n", i);
    }

    print!("outputs:\n");
    let items = midi::list_inputs()?;

    for i in items.iter() {
        print!("\t{}\n", i);
    }

    Ok(())
}
