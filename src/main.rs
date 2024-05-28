use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Привет, это угадайка!");

    let guessed_number = rand::thread_rng().gen_range(1..=100);

    println!("Загаданное число: {guessed_number}");
    println!("Введите число: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка при чтении строки");

    let guess: i32 = guess.trim().parse().expect("Вы ввели не число!");

    match guess.cmp(&guessed_number){
        Ordering::Less => println!("Слишком маленькое число!"),
        Ordering::Equal => println!("Вы угадали!!!"),
        Ordering::Greater => println!("Слишком большое число!"),
    }
}
