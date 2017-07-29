//standard library uses
use std::fmt::{Display, Formatter};

//local package uses
use ::error::GeneticError;
use ::interfaces::genetic_collection_component::GeneticCollectionComponent;
use ::interfaces::genetic_collection::GeneticCollection;
use ::mutable::Mutable;
use ::GeneticResult;
//Constants
const LEVELS_OF_PRECEDENCE: u8 = 2;


//
use rand::{StdRng, Rng};
use probability::distribution::Binomial;
use std::fmt;
use std;

/// This struct will be used
#[derive(Clone, Debug)]
pub struct GeneticString<T> where T: GeneticCollectionComponent{
    vector: Vec<T>
}

impl<T> GeneticCollection for GeneticString<T> where T: GeneticCollectionComponent {
    type Child = GeneticString<T>;

    /// Gets the fitness of an expression result by subtracting it from the target value.
    ///
    /// # Parameters
    /// <ul>
    /// <li>target - the target value</li>
    /// <li>current_value - the result of the expression</li>
    /// </ul
    ///
    /// # Returns
    /// The fitness of the expression result.
    fn get_fitness(&self, fitness: f64) -> GeneticResult<f64>
    {
        Ok((1f64/(self.evaluate_expression()? as f64 - fitness)).abs())
    }

    /// Takes two genetic strings and, through the genetic breeding process,
    /// combines them to produce a child.
    ///
    /// # Parameters
    /// <ul>
    ///     <li>male - a parent genetic string</li>
    ///     <li>female - a parent genetic string</li>
    /// </ul>
    ///
    /// # Returns
    /// <ul>
    ///     <li>The child genetic string if successful.</li>
    ///     <li>MutationError If the process fails.</li>
    /// </ul>
    fn breed(&self, mate: &Self, rng: &mut StdRng) -> GeneticResult<Self::Child> {
        let mut child: Self;
        if self.get_size() <= mate.get_size() {
            child = GeneticString::mix_elements(&self, &mate, rng)?;
        } else {
            child = GeneticString::mix_elements(&self, &mate, rng)?;
        }
        child.mutate(rng)?;
        Ok(child)
    }
}


impl<T> Mutable for GeneticString<T> where T: GeneticCollectionComponent{
    fn mutate(&mut self, random_number_generator: &mut StdRng) -> GeneticResult<()> {
        let mutation_rate = 0.01f64;
        let binomial_dist = Binomial::new(self.get_size(), mutation_rate);
        let number_of_mutations = binomial_dist.n();
        for _ in 0..number_of_mutations {
            let element_to_mutate_index = (self.get_size() as f64 * random_number_generator.next_f64()) as usize;
            self.get_vector_mutable()[element_to_mutate_index].mutate(random_number_generator)?;
        }
        if random_number_generator.gen_range(0,100) == 99 {
            let element_index = random_number_generator.gen_range(0,self.get_size());
            if random_number_generator.gen_range(0,2) == 1 {
                self.insert_element_at_index(element_index, random_number_generator)?;
            } else {
                self.remove_element_at_index(element_index)?
            }
        }
        Ok(())
    }
}

impl<T> Display for GeneticString<T> where T: GeneticCollectionComponent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut gene_string = String::new();
        for g_collect_component in &self.vector {
            gene_string.push_str(&g_collect_component.to_string())
        }
        write!(f, "{}", gene_string)
    }
}
impl<T> GeneticString<T> where T: GeneticCollectionComponent{
    /// Evaluate the expression the genetic string represents and return the
    /// result
    ///
    /// # Parmeters
    /// self
    ///
    /// # Returns
    /// <ul>
    /// <li> The result of the expression if the computation was successful</li>
    /// <li> Arithmetic error if an error occurred</li>
    /// </ul>
    pub fn evaluate_expression(&self) -> GeneticResult<i32> {
        GeneticString::calculate_value(&self.get_vector_immutable())
    }

    /// # Purpose
    ///
    /// This function will create a new genetic string
    ///
    /// # Parameters
    ///
    /// initial_number_of_values, how many values should the genetic string start with.
    ///
    /// # return
    ///
    /// a new genetic string
    ///
    pub fn new(initial_number_of_values: usize, rng: &mut StdRng) -> GeneticResult<GeneticString<T>> {
        let mut genetic_string_vec: Vec<T> = vec!();
        for x in 0..initial_number_of_values {
            genetic_string_vec.push(T::new(x, rng)?);
        }
        Ok(GeneticString { vector: genetic_string_vec })
    }

    /// Returns the vector of genetic bytes contained in the genetic string. Designed to be less
    /// restrictive then get vector immutable.
    pub fn get_vector_immutable(&self) -> &Vec<T> {
        &self.vector
    }

    pub fn get_vector_mutable(&mut self) -> &mut Vec<T> {
        &mut self.vector
    }

    fn get_size(&self) -> usize {
        self.get_vector_immutable().len()
    }

    fn mix_elements(parent_1: &Self, parent_2: &Self, rand_gen:&mut StdRng) -> GeneticResult<Self>{
        if  parent_1.get_size() == 0 {
            let child: Self = GeneticString::new(0, rand_gen)?;
            Ok(child)
        } else {
            let mut child= GeneticString::new(0, rand_gen)?;
            let split_index = rand_gen.gen_range(0,parent_1.get_size());
            for index in 0..parent_2.get_size() {
                if index < split_index {
                    child.get_vector_mutable().push(parent_1.get_vector_immutable()[index].clone())
                } else {
                    child.get_vector_mutable().push(parent_2.get_vector_immutable()[index].clone())
                }
            }
            Ok(child)
        }
    }
    fn insert_element_at_index(&mut self, mut index: usize, rng: &mut StdRng) -> GeneticResult<()> {
        if index % 2 == 1 {
            index = index - 1
        }
        self.get_vector_mutable().insert(index, T::new(index, rng)?);
        index +=1;
        self.get_vector_mutable().insert(index, T::new(index, rng)?);
        Ok(())
    }

    fn remove_element_at_index(&mut self, mut index: usize) -> GeneticResult<()>{
        if self.get_size() == 0 {
            return Err(GeneticError::CollectionEmptyError);
        } else if self.get_size() == 1 {
            self.get_vector_mutable().pop();
        } else {
            if self.get_size() == index {
                index = index - 2;
            } else if index & 2 == 1{
                index = index - 1;
            }
            self.get_vector_mutable().remove(index);
            self.get_vector_mutable().remove(index);
        }
        return Ok(());
    }


    /// Operate will return a closure that can add, subtract, multiply, or divide two i32 GeneticResults.
    ///
    /// # Parameters
    /// gen_str_slice - The genetic string whose values will be operated on
    ///
    /// # Return
    /// GeneticResult<i32> if successful
    /// None if a failure occurs
    fn operate(gen_str_slice: &[T], index: usize) -> Box<Fn(GeneticResult<i32>, GeneticResult<i32>) -> GeneticResult<i32>> {
        match gen_str_slice[index].get_operator().unwrap() as char {                                //TODO: Remove unwrap call eventually
            '+' => Box::new(move |a: GeneticResult<i32>, b: GeneticResult<i32>| {
                a?.checked_add(b?).ok_or(GeneticError::ComputationError)
            }),
             '-' => Box::new(move |a: GeneticResult<i32>, b: GeneticResult<i32>| {
                a?.checked_sub(b?).ok_or(GeneticError::ComputationError)
            }),
             '*' => Box::new(move |a: GeneticResult<i32>, b: GeneticResult<i32>| {
                a?.checked_mul(b?).ok_or(GeneticError::ComputationError)
            }),
             '/' => Box::new(move |a: GeneticResult<i32>, b: GeneticResult<i32>| {
                a?.checked_div(b?).ok_or(GeneticError::ComputationError)
            }),
           _ => Box::new(move |_: GeneticResult<i32>, _: GeneticResult<i32>| Err(GeneticError::InvalidGeneticByteType))
        }
    }

    /// Take a genetic string slice and compute the value of that slice
    ///
    /// # Parameters
    /// gen_str_slice - The genetic string slice
    ///
    /// # returns
    /// The genetic string slices value.
    fn calculate_value(gen_str_slice: &[T]) -> GeneticResult<i32>{
        if gen_str_slice.len() == 0 {
            Ok(0)
        } else if gen_str_slice.len() == 1 {
            Ok(gen_str_slice[0].get_value()? as i32)
        } else {
            for precedence in 0..LEVELS_OF_PRECEDENCE{
                for (index, g_byte) in gen_str_slice.iter().enumerate() {
                    if index % 2 == 1 {
                        let current_precedence = g_byte.get_operator_precedence()?;
                        if current_precedence  == precedence {
                            return GeneticString::operate(gen_str_slice, index)
                             (GeneticString::calculate_value(&gen_str_slice[0..index]),
                              GeneticString::calculate_value(&gen_str_slice[index + 1 ..gen_str_slice.len()]))
                        }
                    }
                }
            }
            Err(GeneticError::ComputationError)
        }
    }

    fn create_random_number_generator() -> GeneticResult<StdRng> {
        StdRng::new().map_err(|_: std::io::Error | GeneticError::RNGCreationError)
    }

}




pub trait GeneticStringTestUtilities <T: GeneticCollectionComponent> {
    /// # Purpose
    /// Create a genetic strings whose value can be precomputed. Used for testing purposes.
    ///
    /// # Parameters
    /// g_byte_vec - a vector of genetic bytes
    ///
    /// # Returns
    /// a genetic string
    fn new(Vec<T>) -> Self;
}

impl<T> GeneticStringTestUtilities<T> for GeneticString<T> where T: GeneticCollectionComponent{
    fn new(g_byte_vec: Vec<T>) -> Self {
        GeneticString{vector: g_byte_vec}
    }
}

//
#[cfg(test)]
mod tests {
    use super::*;
    use ::genetic_byte::{GeneticByte, GeneticByteTestUtilities};
    #[test]
    fn single_byte_calculate_value_should_equal_23_when_byte_equal_23() {
        let mut single_byte_genetic_string = build_single_item_genetic_string();
        let value = match single_byte_genetic_string.evaluate_expression() {
            Ok(value) => value,
            Err(error) => panic!("Test should not have received an error.")
        };
        assert!(23 == value)
    }

    #[test]
    fn empty_genetic_string_calculate_value_should_equal_0_when_blank() {
        let mut empty_genetic_string = build_empty_genetic_string();
        let value = match empty_genetic_string.evaluate_expression() {
            Ok(value) => value,
            Err(error) => panic!("Test should not have received an error.")
        };
        assert_eq!(0, value)
    }
    #[test]
    fn error_should_occur_when_invalid_multi_item_gen_string_is_evaluated() {
        let mut bad_mult_item_gen_str = build_bad_multi_item_gen_string();
        let result = match bad_mult_item_gen_str.evaluate_expression() {
            Ok(value) => panic!("value should not be okay"),
            Err(error) => error
        };
        assert!(match result{
            ::error::GeneticError::ComputationError => true,
            _ => false
        })
    }

    #[test]
    fn calculate_value_returns_negative_5_when_given_expression_which_equals_negative_5() {
        let mut mult_item_gen_str = build_multi_item_gen_string();
        let result = match mult_item_gen_str.evaluate_expression() {
            Ok(value) => value,
            Err(error) => panic!("Test should not have received the error: {}.", error)
        };
        assert_eq!(-5, result)
    }

    /// Build a genetic string with no length
    ///
    /// # Parameters
    ///
    /// # Returns
    /// An empty genetic string.
    fn build_empty_genetic_string() -> GeneticString<GeneticByte> {
        let new_g_vec: Vec<GeneticByte> = vec!();
        GeneticStringTestUtilities::new(new_g_vec)
    }

    /// Build a genetic string 1 byte long with a precomputed value
    ///
    /// # Parameters
    ///
    /// # Return
    /// The genetic string
    fn build_single_item_genetic_string() -> GeneticString<GeneticByte> {
        let new_g_vec: Vec<GeneticByte> = vec!();
        let mut g_byte_1 = GeneticByteTestUtilities::new(23, 0).unwrap();
        let mut g_vec = vec![g_byte_1];
        GeneticStringTestUtilities::new(g_vec)
    }

    fn build_multi_item_gen_string() -> GeneticString<GeneticByte> {
        let new_g_vec: Vec<GeneticByte> = vec!();
        let mut g_byte_1 = GeneticByteTestUtilities::new(5, 0).unwrap();
        let mut g_byte_2 = GeneticByteTestUtilities::new('+' as u8, 1).unwrap();
        let mut g_byte_3 = GeneticByteTestUtilities::new(2, 2).unwrap();
        let mut g_byte_4 = GeneticByteTestUtilities::new('-' as u8, 3).unwrap();
        let mut g_byte_5 = GeneticByteTestUtilities::new(4, 4).unwrap();
        let mut g_byte_6 = GeneticByteTestUtilities::new('*' as u8, 5).unwrap();
        let mut g_byte_7 = GeneticByteTestUtilities::new(3, 6).unwrap();
        let vector = vec![g_byte_1, g_byte_2, g_byte_3, g_byte_4, g_byte_5, g_byte_6, g_byte_7];
        GeneticStringTestUtilities::new(vector)
    }

    fn build_bad_multi_item_gen_string() -> GeneticString<GeneticByte> {
        let new_g_vec: Vec<GeneticByte> = vec!();
        let mut g_byte_1 = GeneticByteTestUtilities::new(5, 0).unwrap();
        let mut g_byte_2 = GeneticByteTestUtilities::new('+' as u8, 1).unwrap();
        let mut g_byte_3 = GeneticByteTestUtilities::new(2, 2).unwrap();
        let mut g_byte_4 = GeneticByteTestUtilities::new('/' as u8, 3).unwrap();
        let mut g_byte_5 = GeneticByteTestUtilities::new(0, 4).unwrap();
        let vector = vec![g_byte_1, g_byte_2, g_byte_3, g_byte_4, g_byte_5];
        GeneticStringTestUtilities::new(vector)
    }
}
