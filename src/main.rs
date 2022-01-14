use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("Adivina el Numero!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
    println!("Ingresa un numero entre 1 y 100");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer la linea");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Tu numero es: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}","Tu numero es menor que el secreto".red()),
        Ordering::Greater => println!("{}","Tu numero es mayor que el secreto".red()),
        Ordering::Equal => {
            println!("{}","Ganaste!".green());
            break;
        },
    }};
    println!("El numero era: {}", secret_number);

}
