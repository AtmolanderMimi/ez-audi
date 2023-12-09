use crate::samples::IntermediateSampleType;
use crate::samples::Samples;

use super::ModifierTrait;

#[derive(Debug, Clone)]
/// Loops the samples by the u32 value specified
pub struct Loop(pub u32);

impl ModifierTrait for Loop {
    fn modify(&self, mut samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let inner_samples = samples.samples.clone();
        
        for _ in 0..self.0 {
            // Append consumes so I will copy before
            let mut cloned_inner_samples = inner_samples.clone();
            samples.samples.append(&mut cloned_inner_samples);
        }

        samples
    }
}