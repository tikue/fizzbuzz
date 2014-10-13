// - Functions ---------------
struct MyFn {
    x: Box<Fn<(String,), String> + Send>,
    s: String
}

impl Fn<(String,), String> for MyFn {
    extern "rust-call" fn call(&self, (_,): (String,)) -> String {
        self.s + self.x.call(("".to_string(),))
    }
}

struct Id;

impl Fn<(String,), String> for Id {
    extern "rust-call" fn call(&self, (s,): (String,)) -> String {
        s
    }
}

// - Main ------------------------
fn fizzbuzz(n: int) -> String {
    let test = |d, s: &str, x| if n % d == 0 {
        box MyFn { x: x, s: s.to_string() } as Box<Fn<(String,), String> + Send>
    } else {
        x
    };

    let x = test(5, "buzz", box Id);
    let x2 = test(3, "fizz", x);
    x2.call((n.to_string(),))
}

fn main() {
    assert!(fizzbuzz(3).as_slice() == "fizz");
    assert!(fizzbuzz(5).as_slice() == "buzz");
    assert!(fizzbuzz(15).as_slice() == "fizzbuzz");
    assert!(fizzbuzz(7).as_slice() == "7");
}
