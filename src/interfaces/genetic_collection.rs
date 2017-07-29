//! genetic collections are like their name

use ::mutable::Mutable;
use ::GeneticResult;
use std::fmt::{Display, Debug};
use rand::StdRng;

pub trait GeneticCollection: Mutable + Debug + Display {
    type Child;

    fn get_fitness(&self, f64) -> GeneticResult<f64>;

    fn breed(&self, &Self, &mut StdRng) -> GeneticResult<Self::Child>;

}
