// Library
use crate::helpers;

// Modules
mod echo;
use echo::Echo;
mod exit;
use exit::Exit;
mod unknown;
use unknown::Unknown;
mod r#type;
use r#type::Type;

// --------
// COMMANDS
// --------

/// A trait that defines a command that can be executed.
/// These commands accept a [`vector`](Vec<&str>) of arguments and return a [`Result`](std::io::Result).
/// ```rs
/// fn execute(&self, args: Vec<&str>) -> std::io::Result<()>;
/// ```
pub trait ExecutableCommand {
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()>;
}

/// The kind of command.
pub enum Command {
    /// A built-in command in the shell
    Builtin(Builtin),
    /// An external program on the system (usually derived from the PATH environment variable)
    Program(String),
    /// An unknown command. The command is not recognized by the shell
    Unknown,
}

// Provide an unified interface for executing commands.
impl Command {
    /// Execute the command. This function will delegate the execution to the appropriate command.
    pub fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        match self {
            Command::Builtin(builtin) => builtin.execute(args),
            Command::Program(_path) => todo!("Implement Program Execution"),
            Command::Unknown => Unknown.execute(args),
        }
    }
}

// Implement the FromStr trait for the Command enum to parse a string into a Command
impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(builtin) = Builtin::from_str(s) {
            Ok(Command::Builtin(builtin))
        } else if helpers::path::find_executable(s).is_some() {
            Ok(Command::Program(s.to_string()))
        } else {
            Ok(Command::Unknown)
        }
    }
}

// ----------------
// BUILTIN COMMANDS
// ----------------

/// The built-in commands in the shell
pub enum Builtin {
    Echo(Echo),
    Exit(Exit),
    Type(Type),
}

// Implement the Command trait for the Builtin commands
impl ExecutableCommand for Builtin {
    /// Execute the built-in command
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        match self {
            Builtin::Echo(cmd) => cmd.execute(args),
            Builtin::Exit(cmd) => cmd.execute(args),
            Builtin::Type(cmd) => cmd.execute(args),
        }
    }
}

// Implement the FromStr trait for the Builtin enum to parse a string into a Builtin command
impl std::str::FromStr for Builtin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo(Echo)),
            "exit" => Ok(Builtin::Exit(Exit)),
            "type" => Ok(Builtin::Type(Type)),
            _ => Err(()),
        }
    }
}
