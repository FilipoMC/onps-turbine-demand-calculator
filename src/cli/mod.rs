use console as cs;
use cs::Term;
use std::io;

use crate::constants;
use crate::logic;

pub enum InputResult<T> {
    Valid(T),
    Invalid,
    Quit,
}

pub fn run_cli() -> io::Result<()> {
    let term = Term::stderr();
    let stdout = Term::stdout();

    term.write_line(
        &cs::style("ONPS per-turbine demand calc.")
            .green()
            .bold()
            .to_string(),
    )?;

    term.write_line(&style_instructions("Provide a number between 0 and 1500"))?;

    term.write_line(&style_instructions("Enter q to quit"))?;

    loop {
        let input = match prompt_input()? {
            InputResult::Valid(n) => n,
            InputResult::Invalid => {
                term.write_line(
                    &cs::style("Provide a number between 0 and 1500")
                        .red()
                        .italic()
                        .to_string(),
                )?;
                continue;
            }
            InputResult::Quit => break,
        };

        let demand = logic::calc_turbine_demand(input);

        stdout.write_line(&format!("Turbine: {}", cs::style(demand).green().bold()))?;
    }

    Ok(())
}

fn prompt_input() -> io::Result<InputResult<u32>> {
    let term = Term::stderr();

    term.write_str(&style_instructions("Demand: "))?;

    let buf = term.read_line()?;

    if buf == "q" {
        return Ok(InputResult::Quit);
    }

    let num: u32 = match buf.parse() {
        Ok(n) => n,
        Err(_) => return Ok(InputResult::Invalid),
    };

    if num > constants::MAX_DEMAND {
        return Ok(InputResult::Invalid);
    }

    Ok(InputResult::Valid(num))
}

fn style_instructions(text: &str) -> String {
    cs::style(text)
        .color256(constants::BRIGHT_BLACK)
        .italic()
        .to_string()
}
