use super::{Samples, SampleType};

/// Adds a method that returns the sample type based on the real sample type of the generic parameter
pub trait GetRealSampleType {
    /// Returns the real sample type based on the generic parameter, if it is not found returns none
    fn get_real_sample_type(&self) -> Option<SampleType> {
        None
    }
}

impl GetRealSampleType for Samples<u16> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::U16)
    }
}

impl GetRealSampleType for Samples<u32> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::U32)
    }
}

impl GetRealSampleType for Samples<u64> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::U64)
    }
}

impl GetRealSampleType for Samples<i8> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::I8)
    }
}

impl GetRealSampleType for Samples<i16> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::I16)
    }
}

impl GetRealSampleType for Samples<i32> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::I32)
    }
}

impl GetRealSampleType for Samples<i64> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::I64)
    }
}

impl GetRealSampleType for Samples<f32> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::F32)
    }
}

impl GetRealSampleType for Samples<f64> {
    fn get_real_sample_type(&self) -> Option<SampleType> {
        Some(SampleType::F64)
    }
}