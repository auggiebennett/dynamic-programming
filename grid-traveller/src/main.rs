use std::collections::HashMap;

fn main() {
    let mut memo = HashMap::new();
    println!("{}", grid_traveler(18,18, &mut memo));
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

            let result = grid_traveler(x-1, y, memo) + grid_traveler(x, y-1, memo);
            memo.insert(search_key, result);
            result
        }
    }
}