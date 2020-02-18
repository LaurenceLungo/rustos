use shim::io;
use shim::path::{Path, PathBuf};

use stack_vec::StackVec;

use pi::atags::Atags;

use fat32::traits::FileSystem;
use fat32::traits::{Dir, Entry};

use crate::console::{kprint, kprintln, CONSOLE};
use shim::io::Read;
use core::str;
use crate::ALLOCATOR;
use crate::FILESYSTEM;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        return self.args[0];
    }
}

<<<<<<< HEAD
/// Starts a shell using `prefix` as the prefix for each line.
=======
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns.
>>>>>>> skeleton/lab3
pub fn shell(prefix: &str) -> ! {
    const char_limit: usize = 512;
    const args_limit: usize = 64;
    loop { // keep repeating line
        let mut bufRead: [u8; char_limit] = [0; char_limit];
        let mut bufLine: [u8; char_limit] = [0; char_limit];
        let mut line = StackVec::new(&mut bufLine);
        kprint!("{}",prefix);

        loop { // keep listening key events
            let buf_count = CONSOLE.lock().read(&mut bufRead).expect("CONSOLE read() failed");
            let input = str::from_utf8(&bufRead[0..buf_count]).expect("CONSOLE buf translate to str failed");
            
            let mut newline_flag = false;
            for c in input.chars() {
                if c == '\n' || c == '\r' { // enter
                    newline_flag = true;
                    let mut bufCommand: [&str; args_limit] = [""; args_limit];
                    let command = Command::parse(str::from_utf8(line.as_slice()).expect(""), &mut bufCommand);
                    match command {
                        Ok(c) => executeCMD(c.path(), &c.args),
                        Err(Error::Empty) => newline_flag = true,
                        Err(Error::TooManyArgs) => {
                            kprintln!();
                            kprint!("error: too many arguments");
                            newline_flag = true;
                        },
                    }
                } else if c == (8u8 as char) || c == (127u8 as char) { // back space / delete
                    let line_len = line.len();
                    line.pop();
                    kprint!("{}",'\r');
                    for i in 0..(line_len + prefix.len()) { 
                        kprint!(" ");
                    }
                    kprint!("{}",'\r');
                    kprint!("{}", prefix);
                    kprint!("{}", str::from_utf8(line.as_slice()).expect(""));
                } else if (c as u8) < 32u8 || (c as u8) > 255u8 { // invisible ASCII
                    kprint!("{}", 7u8 as char);
                } else {
                    match line.push(c as u8) {
                        Ok(()) => kprint!("{}", c),
                        Err(()) => {},
                    }
                }
            }
            if newline_flag {
                kprintln!();
                break;
            }
        }
    }
}

fn executeCMD(path: &str, args: &StackVec<&str>) {
    match path {
        "echo" => echo(args),
        unknown => {
            kprintln!();
            kprint!("unknown command: {}", unknown);
        },
    }
}

fn echo(args: &StackVec<&str>) {
    kprintln!();
    for i in 1..args.len() {
        kprint!("{} ", args[i]);
    }
}