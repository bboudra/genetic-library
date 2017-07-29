use rand::StdRng;
use ::GeneticResult;
pub trait Mutable {
    /// Mutates self if mutation conditions are met.
    ///
    /// # Parameters
    /// Self - the Genetic string to be mutated
    ///
    /// # Returns
    /// Mutation error if an error occurs
    fn mutate(&mut self, &mut StdRng)-> GeneticResult<()>;
}