pub mod faculty;
pub mod stud;

pub fn add(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn sub(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn branch(a: i32) -> isize {
    if a < 10 {
        0
    } else {
        1
    }
}

#[test]
fn test() {
    assert_eq!(10isize, add(4, 6));
    assert_eq!(0isize, branch(4));
}
