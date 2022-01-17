use koopa::back::LlvmGenerator;
use koopa::front::Driver;
use std::env::args;
use std::fmt;
use std::io::{stdin, stdout, Error as IoError};
use std::process::exit;

fn main() {
  if let Err(error) = try_main() {
    eprintln!("{}", error);
    exit(-1);
  }
}

fn try_main() -> Result<(), Error> {
  let mut input = None;
  let mut output = None;
  // parse command line arguments
  let mut args = args();
  args.next();
  loop {
    match (args.next(), args.next()) {
      (Some(file), None) => input = Some(file),
      (Some(file), Some(o)) if o == "-o" => {
        input = Some(file);
        match args.next() {
          Some(file) => output = Some(file),
          _ => return Err(Error::InvalidArguments),
        }
      }
      (Some(o), Some(file)) if o == "-o" => output = Some(file),
      (None, None) => break,
      _ => return Err(Error::InvalidArguments),
    }
  }
  // read Koopa IR program from input
  let program = match input {
    Some(file) => Driver::from_path(file)
      .map_err(Error::InvalidInputFile)?
      .generate_program(),
    _ => Driver::from(stdin()).generate_program(),
  }
  .map_err(|_| Error::ParseError)?;
  // generate LLVM IR and write to output
  match output {
    Some(file) => LlvmGenerator::from_path(file)
      .map_err(Error::InvalidOutputFile)?
      .generate_on(&program),
    _ => LlvmGenerator::new(stdout()).generate_on(&program),
  }
  .map_err(Error::GenerateError)
}

enum Error {
  InvalidArguments,
  InvalidInputFile(IoError),
  ParseError,
  InvalidOutputFile(IoError),
  GenerateError(IoError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::InvalidArguments => write!(
        f,
        r#"usage: koopac [INPUT] -o [OUTPUT]

Options:
  INPUT:  input file, default to stdin
  OUTPUT: output file, default to stdout"#
      ),
      Error::InvalidInputFile(e) => write!(f, "invalid input: {}", e),
      Error::ParseError => write!(f, "invalid Koopa IR program"),
      Error::InvalidOutputFile(e) => write!(f, "invalid output: {}", e),
      Error::GenerateError(e) => write!(f, "I/O error: {}", e),
    }
  }
}
