
pub trait Format {
    fn format(self) -> String;
}

impl<I> Format for I 
where I: ToString {
    fn format(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod test {

	use mytraits::Format;

    #[test]
    fn mytrait() {
    	assert_eq!("32123", 32123.format());
    }

}