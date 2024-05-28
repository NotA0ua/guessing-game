use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Привет, это угадайка!");

    let guessed_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Введите число: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка при чтении строки");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Вам нужно ввести число!");
                continue;
            },
        };

        match guess.cmp(&guessed_number) {
            Ordering::Less => println!("Слишком маленькое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы угадали!!!");
                break;
            }
        }
    }
}
