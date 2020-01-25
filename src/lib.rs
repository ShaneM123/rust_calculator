use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

pub fn pause() -> String {
    let mut x = String::new();
    let mut stdout = stdout();
     stdout.write(b"Welcome User, please enter a number").unwrap();
     stdout.flush().unwrap();
     stdin().read_line(&mut x).expect("error, something went wrong reading input");
     x.to_string()
}

pub fn string_iterator(s: String)-> i32 {
    let mut numbers:Vec<Result<u32,ParseIntError>> = Vec::new();
    let mut signs = Vec::new();
    let mut x = 0;
    let mut l=0;
    let mut m :u32=0;
    let mut acc = String::new();

    for t in s.chars(){
        if  t.is_numeric() {
            println!("T={:?}", t);
            m = t.to_digit(10).unwrap();
            println!("M={:?}", m);
        }
        if !t.is_numeric(){
            numbers.push(Ok(m));
            l +=1;
            signs.push(t);
            if l ==2{
                if signs.get(0).unwrap()==&'+'{
                   for o in &numbers{
                       match o{
                           Ok(a) => x +=*a,
                           _=> {}
                       }
                   }

                }
            }

        }
    }
    return x as i32
}


/*pub fn int_iterator(s: String)-> i32 {
    let mut x:i32=0;
    let  parts = s.split_whitespace().map(|s| s.parse::<i32>());
    for b in parts {match b {
        Ok(a)=> {
            x += a;
        }
        _ => {}
    }}
    *//*
    match parts.next() {
        Some(Ok(a))=> { x+=a;
// a and b are i32
        }
// handle other problems: not enough numbers, numbers are invalid, etc
        _ => {}  // ignore invalid input

    }*//*
    return x
}*/

#[cfg(test)]
mod tests {
use super::string_iterator;
    # [test]
    fn addition_test(){
        assert_eq!(4, string_iterator("2 2".to_string()));
    }
}