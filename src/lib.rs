#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// add this codes per the book
pub fn hello_from_lib(message: &str) {
	println!("Printing Hello {} from library", message);
}
