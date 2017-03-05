use ::mutable::Mutable;
use ::error::GeneticError;
use ::GeneticResult;
use rand::StdRng;
use rand::Rng;
use std::fmt;

pub enum GeneticByte{
    genetic_value(u8),
    genetic_operator(u8)
}

impl Mutable for GeneticByte {
    fn mutate(&mut self, rng: &mut StdRng) -> GeneticResult<()> {
        match self {
            &mut GeneticByte::genetic_value(value) => {
                let bit_to_flip = rng.gen_range(0, 8);
                self.mutate_value(bit_to_flip as u32)?;
                Ok(())
            }
            &mut GeneticByte::genetic_operator(mut operator) => {
                let value = rng.gen_range(1, 5);
                operator = match value {
                    1 => '+',
                    2 => '-',
                    3 => '*',
                    4 => '/',
                    _ => Err(GeneticError::MutationError)?
                } as u8;
                Ok(())
            },
        }
    }
}

impl GeneticByte {

    fn new(&mut self, location_in_collection: usize, rng: &mut StdRng) -> GeneticResult<Self>{
        match location_in_collection % 2 {
            0 => Ok(GeneticByte::genetic_value(rng.gen_range(0,256))),
            1 => match rng.gen_range(0,4) {
                0 => Ok(GeneticByte::genetic_operator('+' as u8)),
                1 => Ok(GeneticByte::genetic_operator('-' as u8)),
                2 => Ok(GeneticByte::genetic_operator('*' as u8)),
                3 => Ok(GeneticByte::genetic_operator('/' as u8)),
                _ => Err(GeneticError::GenByteCreationError)
            },
            _ => Err(GeneticError::GenByteCreationError)
        }
    }
    /// # Purpose
    /// This function will take a bit index and flip that bit
    /// (e.g. 1 -> 0, 0 -> 1) in the GeneticByte.
    /// # Parameters
    /// bit_to_flip: u32 - the bit index
    ///
    /// # Return
    /// none
    fn mutate_value(&mut self, bit_to_flip: u32) -> GeneticResult<()>
    {
        match self {
            &mut GeneticByte::genetic_value(mut value) => {
            let mut u8_bit_representation = [false; 8];
                let mut u8_value = value;
                for bool in &mut u8_bit_representation[..] {
                    *bool = match u8_value % 2 {
                        0 => false,
                        1 => true,
                        _ => panic!("This should be impossible!!!!")
                    };
                    u8_value /= 2
                }
                if u8_bit_representation[bit_to_flip as usize] == true {
                    value -= 2u8.pow(bit_to_flip);
                    Ok(())
                } else {
                    value += 2u8.pow(bit_to_flip);
                    Ok(())
                }
            }
            _ => Err(GeneticError::MutationError)
        }
    }
}
impl fmt::Display for GeneticByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GeneticByte::genetic_operator(value) => {
                write!(f, "{} ", value)
            },
            &GeneticByte::genetic_operator(operator) => {
                write!(f, "{} ", operator as char)
            }
        }
    }
}
