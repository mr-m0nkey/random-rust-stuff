extern crate dotenv;

use dotenv::dotenv;
use std::cell::RefCell;
use std::env;
use std::io::{self, Write};
use structopt::StructOpt;


//NOTE Some code that uses internal mutability and dependency injection
trait Calculator {
    fn calculate(&self) -> i32;
}

struct MockCalculator {}

impl Calculator for MockCalculator {
    fn calculate(&self) -> i32 {
        300
    }
}

struct ConcCalculator {}

impl Calculator for ConcCalculator {
    fn calculate(&self) -> i32 {
        100
    }
}

//REVIEW can this be optimized to use generics
pub struct CalculatorProxy {
    calculator: Box<dyn Calculator>,
    value: RefCell<Option<i32>>,
}

impl CalculatorProxy {
    pub fn calculate(&self) -> i32 {
        let mut stored_value = self.value.borrow_mut();
        if let Some(value) = *stored_value {
            println!("Using cache");
            return value;
        }

        println!("Calculating");
        let value: i32;
        value = self.calculator.calculate();

        *stored_value = Some(value);

        value
    }
}

impl CalculatorProxy {
    pub fn new(use_mock: bool) -> Self {
        let calculator: Box<dyn Calculator>;
        if use_mock {
            calculator = Box::new(MockCalculator {});
        } else {
            calculator = Box::new(ConcCalculator {})
        }

        CalculatorProxy {
            calculator: calculator,
            value: RefCell::new(None),
        }
    }

    pub fn default_new() -> Self {
        dotenv().ok();

        let default_calculator =
            env::var("DEFAULT_CALCULATOR").expect("Default calculator must be configured.");

        let calculator: Box<dyn Calculator>;
        if default_calculator.eq(&String::from("MOCK")) {
            calculator = Box::new(MockCalculator {});
        } else {
            calculator = Box::new(ConcCalculator {})
        }

        CalculatorProxy {
            calculator: calculator,
            value: RefCell::new(None),
        }
    }
}


// NOTE simple command line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Pathfinder",
    about = "A program to find a directory and run a command on it."
)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
   
    pub command: String,
}
