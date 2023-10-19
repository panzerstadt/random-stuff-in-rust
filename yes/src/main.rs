// from https://endler.dev/2017/yes/

use std::io::{self, BufWriter, Write};

const BUFFER_SIZE: usize = 64 * 1024;

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    let default = String::from("y");
    let option = input.first().unwrap_or(&default).as_str();
    let mut args = Vec::new();
    match option {
        "--help" => return println!("sorry i can't help you"),
        "--version" => return println!("v1"),
        value => args.push(value),
    }

    // 1.99MiB/s
    // loop {
    //     println!("{}", args.join(" "));
    // }

    // 33.7MiB/s
    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, io::stdout());
    loop {
        writeln!(writer, "{}", args.join(" ")).unwrap();
    }

    // 4.89GiB/s
    // let stdout = io::stdout();
    // let mut locked = stdout.lock();
    // let mut buffer = [0u8; BUFFER_SIZE];
    // buffer.fill(b'y');
    // while locked.write_all(&buffer).is_ok() {}
    // std::process::exit(1);
}
// speedrun
// cargo run --release | pv -r > /dev/null
