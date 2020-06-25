use libcli::input;

fn main() {
    let line = input::read_line("Enter line\n", "> ");
    println!("Read '{}'", line);
}
