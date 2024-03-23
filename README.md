# Rucycle - Rust Implementation

Rucycle is a Rust implementation of a cycling mechanism that allows iterating over a list of elements, ensuring thread safety with locks and mutexes. This project provides functionalities similar to those found in the original GoCycle package.

## Features

- **Thread Safety**: Utilizes mutexes to ensure safe concurrent access to shared data.
- **Cycle Iteration**: Iterates over a list of elements in a cycling manner, skipping locked elements.
- **File Input**: Allows initialization of cycles from file input.

## Installation

To use this library in your Rust project, add the following line to your `Cargo.toml` file:

## Usage
Then, in your Rust code, you can import the library as follows:
```use rucycle::Cycle;```

## Example

```
use rucycle::Cycle;
use std::time::Duration;

fn main() {
    let cycle = Cycle::new(vec!["A".to_string(), "B".to_string()]);
    println!("Cycle list length: {}", cycle.list_length());

    let locked_element = "A".to_string();
    cycle.lock(locked_element.clone());
    cycle.lock_by_timeout(locked_element.clone(), Duration::from_secs(2));
    cycle.unlock(&locked_element);
}

```
```toml
[dependencies]
rucycle = { git = "https://github.com/yourusername/rucycle.git" }


