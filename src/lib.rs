pub mod wav;
mod errors;
pub use errors::PlayError;
pub mod traits;
// TODO: Rethink Interface design
pub mod cpal_abstraction;
use cpal_abstraction::SampleMetadata;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
