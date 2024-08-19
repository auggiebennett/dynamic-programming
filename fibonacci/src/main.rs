use text_io::read;
use std::collections::HashMap;

fn main() {
    loop {
        let mut memo = HashMap::new();
        print!("Enter a number (0 to exit): ");
        let n: i128 = read!();
        match n {
            0 => break,
            1 => println!("1"),
            2 => println!("1"),
            // _ => println!("{}", fib(n)),
            _ => println!("{}", fib(n, &mut memo)),
        }
    }   
}

fn fib(n: i128, memo: &mut HashMap<i128, i128>) -> i128 {
// fn fib(n: i128) -> i128 {
    match n {
        1 => 1,
        2 => 1,
        _ => {
            if let Some(&result) = memo.get(&n) {
                return result;
            }
            let result = fib(n-1, memo) + fib(n-2, memo);
            // let result = fib(n-1) + fib(n-2);
            memo.insert(n, result);
            result
        }

    }
}