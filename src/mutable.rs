use ::error::GeneticError;
use rand::StdRng;
use ::GeneticResult;
pub trait Mutable {
    fn mutate(&mut self, &mut StdRng)-> GeneticResult<()>;
}