use std::fmt;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum GeneticError {
    ComputationYieldedIllegalResult,
    RNGCreationError,
    GenByteCreationError,
    CollectionEmptyError,
    InvalidGeneticByteType,
    MutationError
}

impl Error for GeneticError{
    fn description(&self) -> &str {
        match self {
            &GeneticError::MutationError => "An error occurred in the mutation process",
            &GeneticError::ComputationYieldedIllegalResult => "Result of the computation was invalid.\n\
            The most likely cause was a division by zero.",
            &GeneticError::RNGCreationError => "The mutation failed because RNG couldn't be created.",
            &GeneticError::GenByteCreationError=> "The mutation failed because a genetic byte \
            couldn't be created.",
            &GeneticError::CollectionEmptyError => "The mutation failed because the method tried to \n\
            remove an element from an empty Collection.",
            &GeneticError::InvalidGeneticByteType => "The type of the Genetic Byte was invalid for \n\
            it's position in the genetic string."

       }
    }
}

impl Display for GeneticError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GeneticError::MutationError => self.description().fmt(f),
            &GeneticError::ComputationYieldedIllegalResult => self.description().fmt(f),
            &GeneticError::RNGCreationError => self.description().fmt(f),
            &GeneticError::GenByteCreationError => self.description().fmt(f),
            &GeneticError::CollectionEmptyError => self.description().fmt(f),
            &GeneticError::InvalidGeneticByteType => self.description().fmt(f),
        }
    }
}