// - Functions ---------------
struct MyFn {
    x: Box<FnMut<(String,), String> + Send>,
    s: String
}

impl FnMut<(String,), String> for MyFn {
    extern "rust-call" fn call_mut(&mut self, (_,): (String,)) -> String {
        self.s + self.x.call_mut(("".to_string(),))
    }
}

struct Id;

impl FnMut<(String,), String> for Id {
    extern "rust-call" fn call_mut(&mut self, (s,): (String,)) -> String {
        s
    }
}

// - Main ------------------------

fn test(n: int, d: int, s: String, x: Box<FnMut<(String,), String> + Send>) 
        -> Box<FnMut<(String,), String> + Send> {
    if n % d == 0 {
        box MyFn { x: x, s: s }
    } else {
        x
    }
}

fn fizzbuzz(n: int) -> String {
    test(n, 3, "fizz".to_string(), test(n, 5, "buzz".to_string(), box Id)).call_mut((n.to_string(),))
}

fn main() {
    assert!(fizzbuzz(3).as_slice() == "fizz");
    assert!(fizzbuzz(5).as_slice() == "buzz");
    assert!(fizzbuzz(15).as_slice() == "fizzbuzz");
    assert!(fizzbuzz(7).as_slice() == "7");
}
