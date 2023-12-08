use crate::samples::IntermediateSampleType;
use crate::samples::Samples;

use super::ModifierTrait;

#[derive(Debug, Clone)]
/// Multiples the amplitude by the IntermediateSampleType (f64) value
pub struct Volume(pub IntermediateSampleType);

impl ModifierTrait for Volume {
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType> {
        let new_samples = samples.samples.into_iter()
            .map(|s| s * self.0)
            .collect();

        Samples::new(new_samples, samples.metadata)
    }
}