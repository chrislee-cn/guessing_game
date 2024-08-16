use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Please input your guess.");
        //通过mut来使变量可变
        let mut guess = String::new();
        io::stdin()
            //&引用
            .read_line(&mut guess)
            //错误处理
            .expect("Failed to read line");
        //{}是占位符，比java规范
        //println!("You guessed :{}",guess)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("The secret number is: {secret_number}");
    }
}
