pub mod lexer;
pub mod parser;
pub mod token;
pub mod value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
