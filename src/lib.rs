#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// add this codes per the book
// this will be called by bin/mymain.rs
pub fn hello_from_lib(message: &str) {
	println!("Printing Hello {} from library", message);
}
