## Beginner level projects (Focus on Fundamentals)
* **Hello World:** The classic!
    *  Learn how to compile and run your first Rust code.  
	```rust
fn main() {
	println!("Hello world");
}	
// To compile and run: $ cargo run
```
* **Simple Calculator:** Basic arithmetic operations (`+`, `-`, `*`, `/`)
	* Apply structs for data types and functions for actions.
	 ```rust 
use std::io;

// Struct to hold two numbers
struct Numbers {
    a: f64,
    b: f64,
}

impl Numbers {
    fn add(&self) -> f64 {
        self.a + self.b
    }

    fn subtract(&self) -> f64 {
        self.a - self.b
    }

    fn multiply(&self) -> f64 {
        self.a * self.b
    }

    fn divide(&self) -> Option<f64> {
        if self.b != 0.0 {
            Some(self.a / self.b)
        } else {
            None
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut input).unwrap();
    let op: char = input.trim().chars().next().unwrap();
    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    let nums = Numbers { a, b };

    let result = match op {
        '+' => Some(nums.add()),
        '-' => Some(nums.subtract()),
        '*' => Some(nums.multiply()),
        '/' => nums.divide(),
        _ => {
            println!("Invalid operator!");
            None
        }
    };

    if let Some(r) = result {
        println!("Result: {}", r);
    }
}

	```

* **Number Guessing Game:**  A fun way to practice conditional statements, loops, random number generation.
```rust
use rand::Rng;
use std::io;

fn main() {
    // create random number generator
    let mut rng = rand::rng();

    // generate one secret number
    let secret_number: i32 = rng.random_range(1..=100);
    println!("(debug) Secret number: {}", secret_number);
    
    // read input from user
    let mut input = String::new();
    println!("Guess a number between 1 and 100:");
    io::stdin().read_line(&mut input).unwrap();
  
    // parse input
    let usr_guess: i32 = input.trim().parse().unwrap_or(-1);
    println!("You entered: {}", usr_guess);

    // check guess
    if usr_guess == secret_number {
        println!("You guessed the number!");
    } else {
        println!("Wrong guess, try again!");
        main();
    }
}
```


* **Text Analyzer (Basic):** Read a file and count word occurrences.
    * Explore using the `std::io` module for file input/output, `std::collections::HashMap` to store words.
```rust
	
```
* **Temperature Converter:**  Convert Celsius to Fahrenheit or vice versa.
    * Learn about type coercion and unit conversions.
```rust

```

