
fn main() {
    // Task 1 : Basic Closure
    let operation = |a: i32, b: i32| {
        a * b
    };
    println!("Task 1: Basic Closure");
    println!("Result: {}", operation(10, 5));

    // Task 2: Environment Capture
    fn track_changes() {
        let mut tracker = 0;
        let mut update = || {
           tracker += 5;
           println!("Tracker: {}", tracker);

        };
    
        update();
        update();
    }
    println!("\nTask 2: Environment Closure");
    track_changes();


    // Task 3: Vector Transformation
    fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32> where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()

}

    // using For loop
    fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32> where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); 
    }
    result

}
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers.clone(), |x| {
        if x > 2 {
            0
        } else {
            x
    }
});


    let doubledd = process_vector_with_for_loop(numbers.clone(), |x| { x * 2 });
    let replacedd = process_vector_with_for_loop(numbers, |x| { if x > 2 { 0 } else { x }});

    println!("\nTask 3: Vector Transformation");
    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
    println!("Doubled with for loop: {:?}", doubledd);
    println!("Replaced with for loop: {:?}", replacedd);


    // Task 4: Lazy Computation
    use std::{thread, time::Duration};

    struct ComputeCache<T>
    where
        T: Fn() -> String,
    {
        computation: T,
        result: Option<String>,
    }

    impl<T> ComputeCache<T>
    where
        T: Fn() -> String,
    {
        fn new(computation: T) -> Self {
            ComputeCache {
                computation,
                result: None,
            }
        }

        fn get_result(&mut self) -> String {
            match &self.result {
                Some(r) => {
                    println!("Retrieved from cache instantly!");
                    r.clone()
                }
                None => {
                    println!("Computing (this will take 2 seconds)... ");
                    thread::sleep(Duration::from_secs(2));
                    let r = (self.computation)();
                    self.result = Some(r.clone());
                    r
                }
            }
        }
    }

    let mut cache = ComputeCache::new(|| {
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });
    println!("\nTask 4: Lazy Computation");
    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());


}