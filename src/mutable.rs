use ::error::GeneticError;
use rand::StdRng;
trait Mutable {
    fn mutate(&self, StdRng)-> GeneticError<()>;
}