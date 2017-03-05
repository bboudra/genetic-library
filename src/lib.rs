/// This library will hold a series of highly generic trait definitions and utility methods
/// implemented on those traits to allow the rapid development of genetic utilities and libraries.

extern crate rand;

//modules
pub mod interfaces;
pub mod mutable;
pub mod error;
pub mod genetic_string;

// Use statements
use error::GeneticError;

pub type GeneticResult<T> = Result<T, GeneticError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

