use ::mutable::Mutable;
use ::error::GeneticError;
use ::GeneticResult;
use rand::StdRng;

pub enum GeneticByte{
    genetic_value(u8),
    genetic_operator(u8)
}

impl Mutable for GeneticByte {
    fn mutate(&self, rng: StdRng) -> GeneticResult<()> {
        match &self {
            &GeneticByte::genetic_value(value) => {
                let bit_to_flip = rng.gen_range(0, 8);
            }
        }
        if byte_type {
            let bit_to_flip = rand::thread_rng().gen_range(0, 8);
            self.mutate_value(bit_to_flip as u32);
        } else {
            let value = rand::thread_rng().gen_range(1, 5);
            self.value = match value {
                1 => '+',
                2 => '-',
                3 => '*',
                4 => '/',
                _ => panic!("you messed up")
            } as u8;
        }
    }
}

impl GeneticByte {
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
        match &self {
            &GeneticByte::genetic_value() => {
            let mut u8_bit_representation = [false; 8];
                let mut u8_value = ;
                for bool in &mut u8_bit_representation[..] {
                    *bool = match u8_value % 2 {
                        0 => false,
                        1 => true,
                        _ => panic!("This should be impossible!!!!")
                    };
                    u8_value /= 2
                }
                if u8_bit_representation[bit_to_flip as usize] == true {
                    self.value -= 2u8.pow(bit_to_flip)
                } else {
                    self.value += 2u8.pow(bit_to_flip)
                }
            }
            _ => Err(GeneticError::MutationError)
        }
    }
}
