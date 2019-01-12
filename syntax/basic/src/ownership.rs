// Rust's ownership system

//
//        Moving ownership          |--->  Copy Traits
// (the default behavior in rust)   |
//                                  |
//                                  |--->  Borrowing
//                                  |         |---->  Reference
//                                  |         |---->  &mut Reference
//                                  |
//                                  |--->  Shared Ownership
//                                  |         |---->  Reference Counter: Rc and Arc

// Box, Rc, Arc, Reference, &mut Reference are all pointers in the system level.
//
// But Box have ownership, so the behavior is moving. Rc and Arc are shared ownership.
//
// Reference and &mut Reference borrow ownership, but do not take ownership, which
// is more polite if you don't need the ownership. References allow you to accept a
// wide variety of other pointers, and so are useful so that you don't have to write
// a number of variants per pointer ( see borrow_mut_reference() ).

#[cfg(test)]
mod test {

    // -------------------------------------------------------
    // Moving ownership, the default behavior in rust

    #[test]
    fn move_ownership() {
        let x = Box::new(5i8);
        let x_pos = format!("{:p}", x); // get the address

        // x lost ownership
        let y = x;
        let y_pos = format!("{:p}", y);
        // x, y are pointer to the same address
        assert_eq!(x_pos, y_pos);
        // so y equals to x
        assert_eq!(*y, 5);

        // y lost ownership
        let z = move_add_one(y);
        let z_pos = format!("{:p}", z);
        // x, z are pointer to the same address
        assert_eq!(x_pos, z_pos);
        // so z = x + 1
        assert_eq!(*z, 6);
    }

    // the return value transit the ownership back
    fn move_add_one(mut num: Box<i8>) -> Box<i8> {
        *num += 1;
        num
    }

    // -------------------------------------------------------
    // Copy Trait changes the moving behavior.
    //
    // Note: Copy Trait is implemented for primitive values.

    #[test]
    fn copy_ownership() {
        let x = Some(5); // Option is implemeted with Copy
        let x_pos = format!("{:p}", &x); // get the address

        // x is copied
        let y = x;
        let y_pos = format!("{:p}", &y);
        // x equals to y, but x is not y.
        assert_eq!(x, y);
        assert!(x_pos != y_pos);

        // y is copied
        let (z, a_pos) = copy_one(y);
        let z_pos = format!("{:p}", &z);
        // y equals to z, but y is not z.
        assert_eq!(y, z);
        assert!(z_pos != y_pos);
        assert!(y_pos != a_pos); // y is copied when calling function
        assert!(a_pos != z_pos); // z is copied from function
    }

    // the return value copy the values
    fn copy_one(a: Option<u32>) -> (Option<u32>, String) {
        // a is copied from y. the return value is copied from a.
        (a, format!("{:p}", &a))
    }

    // -------------------------------------------------------
    // Borrowing ownership with reference

    #[test]
    fn borrow_reference() {
        let x = 1;

        // arithmetic operators for references to primitive values in std library
        assert_eq!(reference_add_one1(&x), 2);

        // arithmetic operators for primitive values
        assert_eq!(reference_add_one2(&x), 2);
    }

    fn reference_add_one1(num: &i8) -> i8 {
        // All of the arithmetic operators in Rust are implemented
        // for both primitive values and references to primitives
        // on either side of the operator. But mutable reference is
        // not. See std::ops::{Add, Mul ...}
        num + 1 // the same as '*num + 1'
    }

    fn reference_add_one2(num: &i8) -> i8 {
        *num + 1 // the same as 'num + 1'
    }

    // -------------------------------------------------------
    // Borrowing ownership with mutable reference

    #[test]
    fn borrow_mut_reference() {
        let mut x = 5i8;
        let x_pos_1 = format!("{:p}", &x);

        mut_reference_add_one(&mut x);
        assert_eq!(x, 6);

        let x_pos_2 = format!("{:p}", &x);
        assert_eq!(x_pos_1, x_pos_2);

        let mut y = Box::new(5i8);
        let y_pos_1 = format!("{:p}", &y);

        mut_reference_add_one(&mut *y); // The Box value is not moved
        assert_eq!(*y, 6);

        *y += 1;
        assert_eq!(*y, 7);
        let y_pos_2 = format!("{:p}", &y);
        assert_eq!(y_pos_1, y_pos_2);
    }

    // ownership is returned when funciton quits
    fn mut_reference_add_one(num: &mut i8) {
        *num += 1; // can't be 'num += 1'
    }

    // -------------------------------------------------------
    // Shared ownership with Rc

    use std::rc::Rc;

    #[test]
    fn share_ownership() {
        assert_eq!(3, *rc());
    }

    // ownership is returned when funciton quits
    fn rc() -> Rc<u32> {
        let rc = Rc::new(3);
        rc.clone()
        // rc goes out of scope.
        // you can't return a reference of rc.
    }
}
