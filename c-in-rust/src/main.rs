
use std::fmt;

// Minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

// Foreign functions must be declared inside an extern block 
// annotated with a #[link] attribute containing the name 
// of the foreign library.
//
// this extern block links to the libm library
#[link(name = "m")]
extern {
    fn ccosf(z: Complex) -> Complex;
}

// safe wrapper for foreign function
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

// the package's native  dependency
extern {
    fn add(lhs: u32, rhs: u32) -> u32;
}

fn main() {
    // z = 0 + 1i
    let z = Complex { re: 0., im: 1. };

    println!("cos({:?}) = {:?}", z, cos(z));

	println!("1 + 2 = {:?}", unsafe { add(1, 2) });    
}

