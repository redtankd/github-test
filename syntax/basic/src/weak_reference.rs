#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use std::sync::Weak;

#[test]
fn weak_reference() {
    let arc = Arc::new(5);
    let weak: Weak<i8> = Arc::downgrade(&arc);

    assert_eq!(1, Arc::strong_count(&arc));
    assert_eq!(1, Arc::weak_count(&arc));

    let mut arc_option = Some(arc);

    {
        let arc = arc_option.take().unwrap();
        assert_eq!(1, Arc::strong_count(&arc));
        assert_eq!(1, Arc::weak_count(&arc));
    }

    assert!(weak.upgrade().is_none());
}
