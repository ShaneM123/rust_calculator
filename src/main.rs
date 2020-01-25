mod lib;
use lib::pause;
use crate::lib::string_iterator;

fn main() {
   let x = pause().trim().to_string();
    println!("Thank you for your participation");
    let x = x.replace(" ", "") + ";";

    println!("{:?}", x );
    let y = string_iterator(x);
    println!("{:?}", y);

}
