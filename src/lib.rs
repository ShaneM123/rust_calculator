use std::io::{stdin, stdout, Write};

pub fn pause() -> String {
    let mut x = String::new();
    let mut stdout = stdout();
     stdout.write(b"Welcome User, please enter a number").unwrap();
     stdout.flush().unwrap();
     stdin().read_line(&mut x).expect("error, something went wrong reading input");
     x.to_string()
}

pub fn string_iterator(s: String)-> i32 {
    let  x =0;
    let mut i = 0;
    let mut val2 = 0;

    let calculation=to_array(s);

    for y in &calculation{
        println!("Y={:?}", y);
        if y.as_str() == "x"{
            println!("matched on multiply= {:?}",  calculation.get(i));
            //let mut val1  = calculation.get(i-1)?.parse::<i32>();
             val2 = calculation.get(i-1).unwrap().parse::<i32>().unwrap() * calculation.get(i+1).unwrap().parse::<i32>().unwrap();
        }
        i = i+1;
    }
    println!("VAL2 ={:?}", val2);
    return x as i32
}
     pub fn stripper(x: String) -> String {
        x.replace(" ", "")
}

pub fn to_array(s: String) -> Vec<String> {
    let mut calculation = Vec::with_capacity(s.len());
    let mut acc = String::new();

    for t in s.chars() {
        if t.is_numeric() {
        acc.push(t);
        println!("ACC num={:?}", acc);
        println!("T={:?}", t);
}
        if !t.is_numeric() {
         calculation.push(acc.clone());
         acc.clear();
         calculation.push(t.to_string());
}
}
    calculation.push(acc.clone());
    return calculation;
}


#[cfg(test)]
mod tests {
use super::string_iterator;
    # [test]
    fn addition_test(){
        assert_eq!(4, string_iterator("2 2".to_string()));
    }
}