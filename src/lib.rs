use std::io::{stdin, stdout, Write};

pub fn pause()->String{
let mut x = String::new();
let mut stdout = stdout();
stdout.write(b"Welcome User, please enter a number").unwrap();
stdout.flush().unwrap();
stdin().read_line(&mut x).expect("error, something went wrong reading input");
x.trim().to_string()
}

pub fn string_iterator(x: String) {
   let mut  y = false;
   let chr_vec = x.chars().collect();
    for a in chr_vec{
        if a == '+' {
            y = true;
        }

    }
}


#[cfg(test)]
mod tests {
    # [test]
    fn addition_test(){
        assert_eq!(2 + 2, 4);
    }
}