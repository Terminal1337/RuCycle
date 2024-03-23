use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::{Mutex, Arc};
use std::time::Duration;

struct Cycle {
    mutex: Mutex<()>,
    locked: Vec<String>,
    list: Vec<String>,
    i: usize,
    wait_time: Duration,
}

impl Cycle {
    fn new(list: Vec<String>) -> Self {
        Cycle {
            mutex: Mutex::new(()),
            locked: Vec::new(),
            list,
            i: 0,
            wait_time: Duration::from_millis(50),
        }
    }

    fn new_from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(Cycle::new(lines))
    }

    fn is_in_list(&self, element: &str) -> bool {
        self.list.iter().any(|v| v == element)
    }

    fn is_locked(&self, element: &str) -> bool {
        self.locked.iter().any(|v| v == element)
    }

    fn next(&mut self) -> &str {
        let _guard = self.mutex.lock().unwrap();

        loop {
            self.i += 1;
            if self.i >= self.list.len() {
                self.i = 0;
            }

            if !self.is_locked(&self.list[self.i]) {
                return &self.list[self.i];
            }

            std::thread::sleep(self.wait_time);
        }
    }

    fn lock(&mut self, element: String) {
        let _guard = self.mutex.lock().unwrap();

        if self.is_in_list(&element) {
            self.locked.push(element);
        }
    }

    fn unlock(&mut self, element: &str) {
        let _guard = self.mutex.lock().unwrap();

        if let Some(index) = self.locked.iter().position(|e| e == element) {
            self.locked.remove(index);
        }
    }

    fn clear_duplicates(&mut self) -> usize {
        let _guard = self.mutex.lock().unwrap();
        let mut removed = 0;
        let mut list = Vec::new();

        for v in &self.list {
            if !list.contains(v) {
                list.push(v.clone());
            } else {
                removed += 1;
            }
        }

        self.list = list;
        removed
    }

    fn remove(&mut self, element: &str) {
        let _guard = self.mutex.lock().unwrap();

        self.list.retain(|e| e != element);
        self.locked.retain(|e| e != element);
    }

    fn lock_by_timeout(&mut self, element: String, timeout: Duration) {
        self.lock(element.clone());
        std::thread::sleep(timeout);
        self.unlock(&element);
    }

    fn list_length(&self) -> usize {
        self.list.len()
    }
}

fn combine_two_cycles(c1: &Cycle, c2: &Cycle) -> Cycle {
    let mut combined_list = Vec::new();
    combined_list.extend_from_slice(&c1.list);
    combined_list.extend_from_slice(&c2.list);

    let mut combined_locked = Vec::new();
    combined_locked.extend_from_slice(&c1.locked);
    combined_locked.extend_from_slice(&c2.locked);

    Cycle {
        mutex: Mutex::new(()),
        locked: combined_locked,
        list: combined_list,
        i: 0,
        wait_time: c1.wait_time,
    }
}

fn main() {
    // Example usage
    let mut c1 = Cycle::new_from_file("proxies.txt").unwrap();
    for x in 1..c1.list_length(){
        println!("{}",c1.next());

    }



}
