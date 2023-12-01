fn main() {

    let s = String::from("abcdefg");
    let c = s.get(3);

    match c {
        Some => println!("{}", c),
        None => return,
    }
}
