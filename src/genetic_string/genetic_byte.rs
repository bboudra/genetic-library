use ::mutable::Mutable;
use ::error::GeneticError;
use ::GeneticResult;
use rand::StdRng;
use rand::Rng;
use std::fmt;

pub enum GeneticByte{
    GeneticValue(u8),
    GeneticOperator(u8)
}

impl Mutable for GeneticByte {
    fn mutate(&mut self, rng: &mut StdRng) -> GeneticResult<()> {
        match self {
            &mut GeneticByte::GeneticValue(value) => {
                let bit_to_flip = rng.gen_range(0, 8);
                self.mutate_value(bit_to_flip as u32)?;
                Ok(())
            }
            &mut GeneticByte::GeneticOperator(mut operator) => {
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

    fn new(location_in_collection: usize, rng: &mut StdRng) -> GeneticResult<Self>{
        match location_in_collection % 2 {
            0 => { let random_value = rng.gen_range(0,255);
                Ok(GeneticByte::GeneticValue(random_value))
            },
            1 => match rng.gen_range(0,4) {
                0 => Ok(GeneticByte::GeneticOperator('+' as u8)),
                1 => Ok(GeneticByte::GeneticOperator('-' as u8)),
                2 => Ok(GeneticByte::GeneticOperator('*' as u8)),
                3 => Ok(GeneticByte::GeneticOperator('/' as u8)),
                _ => Err(GeneticError::GenByteCreationError)
            },
            _ => Err(GeneticError::GenByteCreationError)
        }
    }

    fn get_value(&self) -> u8{
        match self {
            &GeneticByte::GeneticValue(value) => value,
            &GeneticByte::GeneticOperator(operator) => operator
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
            &mut GeneticByte::GeneticValue(ref mut value) => {
            let mut u8_bit_representation = [false; 8];
                let mut u8_value = *value;
                for bool in &mut u8_bit_representation[..] {
                    *bool = match u8_value % 2 {
                        0 => false,
                        1 => true,
                        _ => panic!("This should be impossible!!!!")
                    };
                    u8_value /= 2
                }
                if u8_bit_representation[bit_to_flip as usize] == true {
                    *value -= 2u8.pow(bit_to_flip);
                    Ok(())
                } else {
                    *value += 2u8.pow(bit_to_flip);
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
            &GeneticByte::GeneticValue(value) => {
                write!(f, "{} ", value)
            },
            &GeneticByte::GeneticOperator(operator) => {
                write!(f, "{} ", operator as char)
            }
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    use rand::StdRng;
    #[test]
    fn mutate_changes_1_byte() {
        let mut random_number_generator = StdRng::new().unwrap();
        let mut g_byte =create_g_byte(&mut random_number_generator);
        let start_value= g_byte.get_value() as i16;
        g_byte.mutate(&mut random_number_generator);
        let new_value = g_byte.get_value() as i16;
        let one_bit_was_changed= one_bit_changed(new_value, start_value);
        assert_eq!(start_value, new_value);
    }

    fn one_bit_changed(original_value: i16, new_value: i16) -> bool {
        let mut change_in_value = original_value - new_value;
        let mut number_bits_changed = 0;
        while change_in_value != 0 {
            number_bits_changed += change_in_value % 2;
            change_in_value /= 2;
        }
        number_bits_changed == 1
    }

    fn create_g_byte(rng: &mut StdRng) -> GeneticByte {
        GeneticByte::new(0, rng).unwrap()
    }

}