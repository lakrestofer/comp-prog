use std::io::Read;
fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut n: u32 = buffer.trim().parse().unwrap();

    if n <= 3 {
        println!("1");
    } else {
        println!("{}", n - 2);
    }
}
