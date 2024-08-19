use std::collections::HashMap;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let mut memo = HashMap::new();
    println!("{}", grid_traveler(18,18, &mut memo));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.subsec_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}


fn grid_traveler(x: i32, y: i32, memo: &mut HashMap<String, i128>) -> i128 {
    match (x, y) {
        (0,..) => 0,
        (..,0) => 0,
        (1,..) => 1,
        (..,1) => 1,
        _ => {
            let search_key = format!("{},{}", x, y);
            if let Some(&result) = memo.get(&search_key) {
                return result;
            }
            let search_key = format!("{},{}", y, x);
            if let Some(&result) = memo.get(&search_key) {
                return result;
            }
            

            let result = grid_traveler(x-1, y, memo) + grid_traveler(x, y-1, memo);
            memo.insert(search_key, result);
            result
        }
    }
}