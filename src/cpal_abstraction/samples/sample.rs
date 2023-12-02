use std::fmt::Debug;

use cpal;

pub trait Sample: cpal::SizedSample + cpal::FromSample<f32> + std::marker::Send + 'static + Debug {}

impl<T: cpal::SizedSample + cpal::FromSample<f32> + std::marker::Send + 'static + Debug> Sample for T
where f32: cpal::FromSample<T> {}

#[derive(Debug, Clone)]
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