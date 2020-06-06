pub mod faculty;
pub mod stud;

pub fn add(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn sub(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn branch_not_covered(a: i32) -> isize {
    if a < 10 {
        0
    } else if a == 10 {
        1
    } else {
        2
    }
}

pub fn branch_covered(a: i32) -> isize {
    if a < 10 {
        0
    } else if a == 10 {
        1
    } else {
        2
    }
}

#[test]
fn test() {
    assert_eq!(10isize, add(4, 6));

    assert_eq!(0isize, branch_not_covered(4));
    assert_eq!(2isize, branch_not_covered(20));

    assert_eq!(0isize, branch_covered(4));
    assert_eq!(1isize, branch_covered(10));
}
