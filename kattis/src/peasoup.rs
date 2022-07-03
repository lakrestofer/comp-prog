use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    //number of restaurants
    let n = lines.next().unwrap().parse().unwrap();

    let mut restaurant = "".into();
    let mut found_res = false;
    for _ in 0..n {
        let n_dishes = lines.next().unwrap().parse().unwrap();
        let name = lines.next().unwrap();
        let dishes = lines.by_ref().take(n_dishes);
        let mut has_pancakes = false;
        let mut has_peasoup = false;
        for dish in dishes {
            if dish == "pancakes" {
                has_pancakes = true;
            } else if dish == "pea soup" {
                has_peasoup = true;
            }
        }
        if has_pancakes && has_peasoup {
            restaurant = name;
            found_res = true;
            break;
        }
    }

    if found_res {
        println!("{}", restaurant);
    } else {
        println!("Anywhere is fine I guess");
    }
}
