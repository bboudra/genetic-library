enum GeneticError {
    MutationError
}

impl Error for GeneticError{
    fn description(&self) -> &str {
        match self {
            &GeneticError::MutationError => "A mutation operation failed in the computation."
       }
    }
}

impl Display for GeneticError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GeneticError::MutationError => self.description().fmt(f),
        }
    }
}