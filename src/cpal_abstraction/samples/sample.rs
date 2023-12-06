use std::fmt::Debug;

use cpal;

use super::Samples;

/// Type that the samples with be converted to in order to do stuff such as apply modifiers
pub type IntermediateSampleType = f64;
/// Trait implemented on all supported samples types
pub trait Sample: cpal::SizedSample + cpal::FromSample<IntermediateSampleType> + std::marker::Send + 'static + Into<SampleType> + Debug {}

impl<T: cpal::SizedSample + cpal::FromSample<IntermediateSampleType> + std::marker::Send + 'static + Into<SampleType> + Debug> Sample for T
where IntermediateSampleType: cpal::FromSample<IntermediateSampleType> {}

#[derive(Debug, Clone, PartialEq)]
pub enum SampleType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl From<SampleType> for cpal::SampleFormat {
    fn from(value: SampleType) -> Self {
        match value {
            SampleType::U8 => cpal::SampleFormat::U8,
            SampleType::U16 => cpal::SampleFormat::U16,
            SampleType::U32 => cpal::SampleFormat::U32,
            SampleType::U64 => cpal::SampleFormat::U64,

            SampleType::I8 => cpal::SampleFormat::I8,
            SampleType::I16 => cpal::SampleFormat::I16,
            SampleType::I32 => cpal::SampleFormat::I32,
            SampleType::I64 => cpal::SampleFormat::I64,

            SampleType::F32 => cpal::SampleFormat::F32,
            SampleType::F64 => cpal::SampleFormat::F64,
        }
    }
}

// TODO: This is stupid bruteforcing of the problem
// There is probably a way to make a macro for this
impl From<u8> for SampleType {
    fn from(_value: u8) -> Self {
        SampleType::U8
    }
}

impl From<u16> for SampleType {
    fn from(_value: u16) -> Self {
        SampleType::U16
    }
}

impl From<u32> for SampleType {
    fn from(_value: u32) -> Self {
        SampleType::U32
    }
}

impl From<u64> for SampleType {
    fn from(_value: u64) -> Self {
        SampleType::U64
    }
}

impl From<i8> for SampleType {
    fn from(_value: i8) -> Self {
        SampleType::I8
    }
}

impl From<i16> for SampleType {
    fn from(_value: i16) -> Self {
        SampleType::I16
    }
}

impl From<i32> for SampleType {
    fn from(_value: i32) -> Self {
        SampleType::I32
    }
}

impl From<i64> for SampleType {
    fn from(_value: i64) -> Self {
        SampleType::I64
    }
}

impl From<f32> for SampleType {
    fn from(_value: f32) -> Self {
        SampleType::F32
    }
}

impl From<f64> for SampleType {
    fn from(_value: f64) -> Self {
        SampleType::F64
    }
}

// Split

impl From<&Samples<u8>> for SampleType {
    fn from(_value: &Samples<u8>) -> Self {
        SampleType::U8
    }
}

impl From<&Samples<u16>> for SampleType {
    fn from(_value: &Samples<u16>) -> Self {
        SampleType::U16
    }
}

impl From<&Samples<u32>> for SampleType {
    fn from(_value: &Samples<u32>) -> Self {
        SampleType::U32
    }
}

impl From<&Samples<u64>> for SampleType {
    fn from(_value: &Samples<u64>) -> Self {
        SampleType::U64
    }
}

impl From<&Samples<i8>> for SampleType {
    fn from(_value: &Samples<i8>) -> Self {
        SampleType::I8
    }
}

impl From<&Samples<i16>> for SampleType {
    fn from(_value: &Samples<i16>) -> Self {
        SampleType::I16
    }
}

impl From<&Samples<i32>> for SampleType {
    fn from(_value: &Samples<i32>) -> Self {
        SampleType::I32
    }
}

impl From<&Samples<i64>> for SampleType {
    fn from(_value: &Samples<i64>) -> Self {
        SampleType::I64
    }
}

impl From<&Samples<f32>> for SampleType {
    fn from(_value: &Samples<f32>) -> Self {
        SampleType::F32
    }
}

impl From<&Samples<f64>> for SampleType {
    fn from(_value: &Samples<f64>) -> Self {
        SampleType::F64
    }
}