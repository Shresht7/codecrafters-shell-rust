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
mod program;
use program::Program;
mod pwd;
use pwd::PWD;
mod cd;
use cd::CD;

// --------
// COMMANDS
// --------

/// A trait that defines a command that can be executed.
pub trait ExecutableCommand {
    fn execute<T>(
        &self,
        args: Vec<String>,
        out_writer: &mut T,
        err_writer: &mut T,
    ) -> std::io::Result<()>
    where
        T: std::io::Write;
}

/// A trait that defines the information about a command.
/// This includes the name, description, and usage of the command.
pub trait CommandInfo {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn usage(&self) -> String;
}

/// The kind of command.
pub enum Command {
    /// A built-in command in the shell
    Builtin(Builtin),
    /// An external program on the system (usually derived from the PATH environment variable)
    Program(Program),
    /// An unknown command. The command is not recognized by the shell
    Unknown,
}

// Provide an unified interface for executing commands.
impl Command {
    /// Execute the command. This function will delegate the execution to the appropriate command.
    pub fn execute<T>(
        &self,
        args: Vec<String>,
        out_writer: &mut T,
        err_writer: &mut T,
    ) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        match self {
            Command::Builtin(builtin) => builtin.execute(args, out_writer, err_writer),
            Command::Program(path) => path.execute(args, out_writer, err_writer),
            Command::Unknown => Unknown.execute(args, out_writer, err_writer),
        }
    }
}

// Implement the FromStr trait for the Command enum to parse a string into a Command
impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(builtin) = Builtin::from_str(s) {
            Ok(Command::Builtin(builtin))
        } else if let Some(path) = helpers::path::find_executable(s) {
            Ok(Command::Program(Program::new(path)))
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
    PWD(PWD),
    CD(CD),
}

// Implement the Command trait for the Builtin commands
impl ExecutableCommand for Builtin {
    /// Execute the built-in command
    fn execute<T>(
        &self,
        args: Vec<String>,
        out_writer: &mut T,
        err_writer: &mut T,
    ) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        match self {
            Builtin::Echo(cmd) => cmd.execute(args, out_writer, err_writer),
            Builtin::Exit(cmd) => cmd.execute(args, out_writer, err_writer),
            Builtin::Type(cmd) => cmd.execute(args, out_writer, err_writer),
            Builtin::PWD(cmd) => cmd.execute(args, out_writer, err_writer),
            Builtin::CD(cmd) => cmd.execute(args, out_writer, err_writer),
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
            "pwd" => Ok(Builtin::PWD(PWD)),
            "cd" => Ok(Builtin::CD(CD)),
            _ => Err(()),
        }
    }
}
