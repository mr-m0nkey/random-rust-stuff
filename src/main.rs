use std::io::{self, Write};
use std::process::Command;
use structopt::StructOpt;
use cache::Cli;


fn main() {
    let args = Cli::from_args();
   
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &args.command])
            .current_dir(args.path)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(&args.command)
                .current_dir(args.path)
                .output()
                .expect("failed to execute process")
    };

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}