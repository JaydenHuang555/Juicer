use std::io::Stdin;

pub fn input(stdin: Stdin, buff: &mut String) -> Result<usize, std::io::Error> {
    let r = stdin.read_line(buff);
    if r.is_ok() {
        buff.pop();
    }
    r
}

pub fn loop_bool_input_str(msg: &str, yes: &str, no: &str) -> Result<bool, std::io::Error> {
    let mut input = String::new();
    loop {
        println!("{}", msg);
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Unable to get input due to {}", e);
            return Err(e);
        }
        input.pop();
        if input.to_lowercase() == yes.to_lowercase() {
            return Ok(true);
        } else if input.to_lowercase() == no.to_lowercase() {
            return Ok(false);
        }
        input.clear();
    }
}
