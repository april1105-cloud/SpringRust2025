fn sum(total: &mut i32, low: i32, high: i32) {
    *total = (low..=high).sum();
}

fn main() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total sum from 0 to 100: {}", total);
}