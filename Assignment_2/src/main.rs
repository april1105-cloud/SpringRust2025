fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 7, 15, 22, 9, 30, 5, 18, 11, 20];

    // Step 3: Iterate and check conditions
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // Step 4: Find the sum using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("\nSum of all numbers: {}", sum);

    // Step 5: Find the largest number
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}