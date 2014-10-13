// - Functions ---------------
struct Const<A> {
    x: A
}

impl<A, B> Fn<(B,), A> for Const<A> where A: Clone {
    extern "rust-call" fn call(&self, (_,): (B,)) -> A {
        self.x.clone()
    }
}

struct Id;

impl<A> Fn<(A,), A> for Id {
    extern "rust-call" fn call(&self, (a,): (A,)) -> A {
        a
    }
}

// - Main ------------------------
fn fizzbuzz(n: int) -> String {
    let test = |d, s: &str, x: Box<Fn<(String,), String> + Send>|
        if n % d == 0 {
            box Const {
                x: s.to_string() + x.call(("".to_string(),))
            } as Box<Fn<(String,), String> + Send>
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
