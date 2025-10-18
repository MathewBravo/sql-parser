pub mod parser;
pub mod token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parser::hello_world();
    }
}
