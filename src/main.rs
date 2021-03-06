struct Const<A> {
    x: A
}

/// Implements the function b -> a for `Const`
impl<A, B> Fn<(B,), A> for Const<A> where A: Send + Clone {
    extern "rust-call" fn call(&self, (_,): (B,)) -> A {
        self.x.clone()
    }
}

/// The constant function a -> b -> a
/// Due to limitations in the type system, the type of b is fixed at the time `const_` is called
/// rather than at the time the created function is called
fn const_<A, B>(a: A) -> Box<Fn<(B,), A> + Send> where A: Send + Clone {
    box Const { x: a }
}

struct Id;

/// Implements the id function a -> a for `Id`
impl<A> Fn<(A,), A> for Id {
    extern "rust-call" fn call(&self, (a,): (A,)) -> A {
        a
    }
}

/// Implements the FizzBuzz algorithm.
/// If n is:
///     * divisible by 3, returns "fizz"
///     * divisible by 5, returns "buzz"
///     * divisible by 3 and 5, returns "fizzbuzz"
///     * anything else, returns n as a String
fn fizzbuzz(n: int) -> String {
    let test = |d, s: &str, x: Box<Fn<(String,), String> + Send>|
        if n % d == 0 {
            const_(s.to_string() + x.call(("".to_string(),)))
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
