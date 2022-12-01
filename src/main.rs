use color_eyre::eyre::Result;
use dotenv::dotenv;
use structopt::StructOpt;

mod day;
mod run;

use run::{Run, RunAll};

#[derive(StructOpt)]
#[structopt(name = "Advent Of Code")]
enum Args {
    /// Run code of the given day
    Run(Run),
    /// Run code of all days
    RunAll(RunAll),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv()?;
    let args = Args::from_args();
    match args {
        Args::RunAll(run_all) => {
            let (output, duration) = run_all.run_all()?;
            println!("{}", output);
            if let Some(duration) = duration {
                println!(
                    "Time: {}s",
                    (duration.subsec_microseconds() as f64) / 1000000.0
                );
            }
        }
        Args::Run(run) => {
            let (output, duration) = run.run()?;
            println!("{}", output);
            if let Some(duration) = duration {
                println!("Time: {}μs", duration.whole_microseconds());
            }
        }
    }
    Ok(())
}
