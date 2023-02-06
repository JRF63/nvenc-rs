mod encoder;
mod error;
mod settings;
mod sys;
mod util;

pub type Result<T> = std::result::Result<T, NvEncError>;

pub use self::{
    encoder::{device::*, EncoderBuilder, EncoderInput, EncoderOutput},
    error::NvEncError,
    settings::{Codec, CodecProfile, EncodePreset, MultiPassSetting, TuningInfo},
};
