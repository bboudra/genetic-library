//! genetic collections are like their name

use ::mutable::Mutable;
use ::GeneticResult;
use std::iter::Iterator;
pub trait GeneticCollection: Mutable + Iterator{
    fn get_fitness(&self, target: f64) -> GeneticResult<f64>;

    fn breed<T>(&self, T) -> GeneticResult<T>
        where T: Self;
}
