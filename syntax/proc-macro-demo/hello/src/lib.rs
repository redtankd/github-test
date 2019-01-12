pub trait HelloMacro {
    fn hello_macro() -> String;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
