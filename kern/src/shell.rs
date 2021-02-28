use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};

use shim::io::Read;

use core::str;
use core::default::Default;

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
        self.args[0]
    }

    fn execute(&self) {
        let path = self.path();
        match path {
            "echo" => echo_cmd(&self.args.as_slice()),
            _ => kprint!("unknown command: {}", path)
        }
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    let mut cmd_mem: [u8; 512] = [0; 512];
    //let mut parse_mem: [&str; 10] = Default::default();

    let mut user_input: StackVec<u8> = StackVec::new(&mut cmd_mem);

    kprint!("{}", prefix);
    loop {
        let read_char = CONSOLE.lock().read_byte();

        if read_char == 127 || read_char == 8 {
            if !user_input.is_empty() {
                user_input.pop();
                kprint!("\u{8} \u{8}");
            }
        } else if read_char == b'\n' || read_char == b'\r' {
            kprint!("\r\n");

            let mut parsed_cmd = [""; 64]; 
            let string = str::from_utf8(user_input.as_slice()).unwrap();
            let cmd = Command::parse(&string, &mut parsed_cmd);

            match cmd {
                Ok(cmd) => cmd.execute(),
                Err(err) => {
                    match err {
                        Error::TooManyArgs => kprintln!("error: too many arguments"),
                        Error::Empty => (),
                    }
                }
            }

            kprint!("\r\n{}", prefix);
            user_input.truncate(0);
        } else {
            if !read_char.is_ascii_graphic() && !read_char.is_ascii_whitespace() {
                kprint!("\u{7}");
            } else if user_input.is_full() {
            } else {
                kprint!("{}", read_char as char);
                user_input.push(read_char).unwrap();
            }
        }

        //kprintln!("Char {} = Byte {}", read_char as char, read_char as u8);
    }
}

fn echo_cmd(args: &[&str]) {
    for arg in args.iter().skip(1) {
        kprint!("{} ", arg);
    }
}
