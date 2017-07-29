use ::mutable::Mutable;
use ::GeneticResult;
use ::std::marker::Sized;
use std::fmt::Debug;
use std::fmt::Display;
use rand::StdRng;

pub trait GeneticCollectionComponent: Mutable + Clone + Debug + Display{

    fn new(location: usize, &mut StdRng) -> GeneticResult<Self> where Self: Sized;

    fn get_value(&self) -> GeneticResult<u8>;

    fn get_operator(&self) -> GeneticResult<u8>;

    fn get_operator_precedence(&self) -> GeneticResult<u8>;

}