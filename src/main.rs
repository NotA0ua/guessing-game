use std::io;
use rand::Rng;

fn main() {
    println!("Привет, это угадайка!");

    let guessed_number = rand::thread_rng().gen_range(1..=100);

    println!("Загаданное число: {guessed_number}");
    println!("Введите число: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка при чтении строки");

    println!("Вы ввели число: {guess}");
}
