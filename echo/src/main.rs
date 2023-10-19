fn main() {
    let output = std::env::args().skip(1).collect::<Vec<_>>();
    println!("{}", output.join(" "));
}
