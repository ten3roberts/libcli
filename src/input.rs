//! Utilities for reading user input
use std::io;
use std::io::Read;
use std::io::Write;

/// Prints a message and prompt to the console
/// Returns one line entered from stdin
/// Includes the newline character, use .trim() to remove
/// msg and prompt are separated because you usually want to provide msg from a list and prompt is usually the same, this alleviates the need for a read_line(format!(())
/// Panics if stdin can't be read
pub fn read_line(msg: &str, prompt: &str) -> String {
    self::prompt(msg, prompt);
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read from stdin");
    string
}

/// Prints a message and prompt to the console
/// Returns all characters from stdin
/// Panics if stdin can't be read
pub fn read_all(msg: &str, prompt: &str) -> String {
    self::prompt(msg, prompt);
    let mut string = String::new();
    io::stdin()
        .read_to_string(&mut string)
        .expect("Failed to read from stdin");
    string
}

/// Prints a message and prompt to the console
/// Reads num_bytes of stdin to a string
/// Returns Err if read buffer couldn't be converted to valid utf8
/// Note: num_bytes may not correspond with resulting string length due to multibyte characters
/// Panics if stdin can't be read
pub fn read_num(num_bytes: usize, msg: &str, prompt: &str) -> Result<String, std::str::Utf8Error> {
    self::prompt(msg, prompt);
    let mut buf = Vec::with_capacity(num_bytes);
    io::stdin()
        .read_exact(&mut buf)
        .expect("Failed to read from stdin");
    match std::str::from_utf8(&buf) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(e),
    }
}

fn prompt(msg: &str, prompt: &str) {
    print!("{}{}", msg, prompt);
    std::io::stdout().flush().expect("Failed to flush stdout");
}
