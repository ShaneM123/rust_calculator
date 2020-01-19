mod lib;
use lib::pause;
use crate::lib::string_iterator;

fn main() {
   let x = pause();
    println!("Thank you for your participation");
    println!("{:?}", x );
    let y = string_iterator(x);
    println!("{:?}", y);

}
