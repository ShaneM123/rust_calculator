use std::io::{stdin, stdout, Write};

fn pause()->String{
    let mut x = String::new();
    let mut stdout = stdout();
    stdout.write(b"Welcome User, please enter a number").unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut x).expect("error, something went wrong reading input");
    x.trim().to_string()
}

fn main() {
   let x = pause();
    println!("Thank you for your participation");
    println!("{:?}", x );
}
