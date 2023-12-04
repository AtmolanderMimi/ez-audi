use super::{Samples, IntermediateSampleType, Sample};

/// A trait to implement on your sample modifiers (aka effects). 
/// Note that the modifiers are made to act upon cpal samples, go see the Sample trait cpal provides.
pub trait ModifierTrait {
    /// Modifies the samples it is used upon.
    /// # NOTES:
    /// * Take into consideration that audio with two channels will be arranged like so: 
    /// Left1, Right1, Left2, Right2...
    /// ## When using inside SamplesPlayer
    /// * The samples you will return will be converted back into the sample type of the original samples
    /// * Do not try to modify the metadata as it is not taken into consideration, changes things such as
    /// the sample rate or channel number will do nothing on how the samples are played
    /// * The order of modifiers is important for SamplesPlayer, the modifiers are applied to the result of the previous one
    fn modify(&self, samples: Samples<IntermediateSampleType>) -> Samples<IntermediateSampleType>;
}

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

/// Adds channels or flattens existing ones into the desired amount, also changes the metadata to match
pub fn into_n_channels(samples: Samples<IntermediateSampleType>, nb_channels: u16) -> Samples<IntermediateSampleType> {
    const OLD_CHANNEL_LENGHT: f64 = 1.0;
    /// The offset applied to the start and end of the new_channel ranges in order to compense
    /// for f64's imprecision
    const OFFSET: f64 = 0.05;

    let metadata = samples.metadata.clone();

    let mut new_metadata = metadata.clone();
    new_metadata.channels = nb_channels;

    // Creates ranges representing each channel from the old and new
    let total_lenght = OLD_CHANNEL_LENGHT * metadata.channels as f64;
    let new_channel_lenght = total_lenght / nb_channels as f64;

    let mut old_channels = Vec::new();
    for i in 0..metadata.channels {
        let start = i as f64 * OLD_CHANNEL_LENGHT;
        let end = OLD_CHANNEL_LENGHT + start;
        old_channels.push(start..end)
    }

    let mut new_channels = Vec::new();
    for i in 0..metadata.channels {
        let start = (i as f64 * new_channel_lenght) + OFFSET;
        let end = (new_channel_lenght + start) - OFFSET;
        new_channels.push(start..end)
    }

    #[derive(Debug)]
    struct Channel {
        factor_by_channel: f64,
        origin_channels_index: Vec<usize>,
    }

    // Checks which old channels the new channels contain
    let mut new_channels_structs = Vec::new();
    for channel in new_channels {
        let truth_map = old_channels.iter()
            .enumerate()
            .map(|(index, oc)| (oc.contains(&channel.start) // If begining is in old
                || oc.contains(&channel.end) // If end is in old
                // If old is completely contained
                || (channel.contains(&oc.start) && channel.contains(&oc.end)), index))
                .collect::<Vec<(bool, usize)>>();

        let origin_channels_index = truth_map.into_iter()
            .filter(|(c, i)| *c == true)
            .map(|(c, i)| i)
            .collect::<Vec<usize>>();

        let origin_channels = origin_channels_index.iter().count();

        new_channels_structs.push(Channel { factor_by_channel: 1.0 / origin_channels as f64, origin_channels_index });
    }

    for c in &new_channels_structs {
        println!("{:?}", c)
    }

    // Joins the new channels
    let seperated_channels = seperate_channels(samples);
    let mut new_channels_samples = Vec::new();
    (0..new_channels_structs.len()).for_each(|_| new_channels_samples.push(Vec::new()));

    for (c_i,c) in new_channels_structs.iter().enumerate() {
        for s_i in 0..seperated_channels[0].len() {
            let new_sample = c.origin_channels_index.iter()
                .fold(0.0, |o, n| o + (seperated_channels[*n][s_i] * c.factor_by_channel));

            new_channels_samples[c_i].push(new_sample)
        }
    }

    let new_samples = join_channels(new_channels_samples);

    Samples::new(new_samples, new_metadata)
}

/// Seperates the channels into vectors
pub fn seperate_channels<T: Sample>(samples: Samples<T>) -> Vec<Vec<T>> {
    let metadata = samples.metadata;

    let mut channels = Vec::new();
    (0..metadata.channels).for_each(|_| channels.push(Vec::new()));

    for (i, sample) in samples.samples.into_iter().enumerate() {
        channels[i % metadata.channels as usize].push(sample)
    }

    channels
}

/// Joins multiple vec cotaining vecs into one by interweaving the values
pub fn join_channels<T: Sample>(seperated_channels: Vec<Vec<T>>) -> Vec<T> {
    let lenght = seperated_channels[0].len();

    let mut coalescence = Vec::new();
    for i in 0..lenght {
        for channel in &seperated_channels {
            coalescence.push(channel[i])
        }
    }

    coalescence
}