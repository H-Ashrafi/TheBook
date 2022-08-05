use std::io;
fn main() {
    println!("Geussing Numbers!");
    println!("Please input your gusse!");

    let mut guess=String::new();
    io::stdin()
            .read_line(&mut guess)
            .expect("faild to read your guess");
    println!("you guessed {guess}");
}
