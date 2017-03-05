/// This library will hold a series of highly generic trait definitions and utility methods
/// implemented on those traits to allow the rapid development of genetic utilities and libraries.


//modules
mod genetic_collection;
mod mutable;
mod error;

// Use statements
use error::GeneticError;

pub type Result<T> = Result<T, GeneticError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

