fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| {
            if i % 3 == 0 && i % 5 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect()
}

fn main() {
    let n = 15;
    let result = fizz_buzz(n);
    for s in result {
        println!("{}", s);
    }
}
