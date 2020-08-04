use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let magic_number = rand::thread_rng().gen_range(0, 10);

    loop {
        let mut type_number = String::new();

        println!("Type your number:");
        io::stdin()
            .read_line(&mut type_number)
            .expect("Failed the typed");
        let type_number: u32 = match type_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("wrong number! Again...");
                continue;
            }
        };

        match type_number.cmp(&magic_number) {
            Ordering::Less => println!("too small!!"),
            Ordering::Equal => {
                println!("done!! The magic number is {}", magic_number);
                break;
            }
            Ordering::Greater => println!("too big!!"),
        }
    }
}
