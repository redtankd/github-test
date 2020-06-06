pub mod faculty;
pub mod stud;

pub fn add(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn sub(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn branch1(a: i32) -> isize {
    if a < 10 {
        0
    } else if a < 20 {
        1
    } else {
        2
    }
}

pub fn branch2(a: i32) -> isize {
    if a < 10 {
        0
    } else if a < 20 {
        1
    } else {
        2
    }
}

pub fn branch3(a: i32) -> isize {
    if a < 10 {
        0
    } else if a < 20 {
        1
    } else {
        2
    }
}

pub fn match1(a: i32) -> isize {
    match a {
        x if x < 10 => 0,
        x if x < 20 => 1,
        x if x < 30 => 2,
        x if x < 40 => 3,
        x if x < 50 => 4,
        _ => -1
    }
}

#[test]
fn test() {
    assert_eq!(10isize, add(4, 6));

    assert_eq!(0isize, branch1(4));
    assert_eq!(1isize, branch1(15));

    assert_eq!(0isize, branch2(5));
    assert_eq!(2isize, branch2(30));

    assert_eq!(1isize, branch3(15));
    assert_eq!(2isize, branch3(30));

    assert_eq!(0isize, match1(4));
    assert_eq!(1isize, match1(15));
    assert_eq!(3isize, match1(35));
}
