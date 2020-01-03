pub struct Faculty {
    id: u8,
}

impl Faculty {
    pub fn new(id: u8) -> Faculty {
        Faculty { id: id }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
}

#[test]
fn test_faculty() {
    let f = Faculty::new(10);
    assert_eq!(10u8, f.id);
}