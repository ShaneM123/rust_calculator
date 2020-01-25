mod lib;
use lib::pause;
use lib::stripper;
use crate::lib::string_iterator;

fn main() {
   let x = stripper(pause().trim().to_string());
    println!("Thank you for your participation");
    println!("{:?}", x );
    let y = string_iterator(x);
    println!("{:?}", y);

}
