pub mod parser;
pub mod token;
pub mod scanner;
pub mod errors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parser::hello_world();
    }
}
