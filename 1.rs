use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut sum: usize = 0;
    let str_num = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for line in stdin.lock().lines() {
        let words = line.unwrap();
        let mut first_num = words.find(char::is_numeric);
        let mut last_num = words.rfind(char::is_numeric);
        
        let mut a = 0;
        let mut b = 0;
        
        if !first_num.is_none() { a = words.chars().nth(first_num.unwrap()).unwrap().to_string().parse::<usize>().unwrap(); }
        if !last_num.is_none() { b = words.chars().nth(last_num.unwrap()).unwrap().to_string().parse::<usize>().unwrap(); }
        
        let mut first_str_num;
        let mut last_str_num;
        for (x, num) in str_num.iter().enumerate() {
            first_str_num = words.find(num);
            last_str_num = words.rfind(num);
            
            if !first_str_num.is_none() && (first_num.is_none() || first_str_num < first_num) { first_num = first_str_num; a = x + 1; }
            if !last_str_num.is_none() && (last_num.is_none() || last_str_num > last_num) { last_num = last_str_num; b = x + 1; }
        }
        
        let val = a*10 + b;
        sum += val;
        println!("{} {:?} {:?}", words, val, sum);
    }
}
