use super::{
    config::{EncodeParams, ExtraOptions},
    device::{DeviceImplTrait, IntoDevice},
    encoder_input::EncoderInput,
    encoder_output::EncoderOutput,
    library::Library,
    raw_encoder::RawEncoder,
    shared::encoder_channel,
    texture::TextureBufferImplTrait,
};
use crate::{Codec, CodecProfile, EncodePreset, MultiPassSetting, NvEncError, Result, TuningInfo};
use std::mem::MaybeUninit;

/// Size of the ring buffer that is shared between the input and output
pub const BUFFER_SIZE: usize = 8;

/// Checks if the user's NvEncAPI version is supported.
fn is_version_supported(version: u32) -> bool {
    // TODO: Change this logic once older versions (9.0 to 10.0) are supported
    let this_version = crate::sys::NVENCAPI_MAJOR_VERSION << 4 | crate::sys::NVENCAPI_MINOR_VERSION;
    if version >= this_version {
        true
    } else {
        false
    }
}

/// Builder for an encoder.
pub struct EncoderBuilder<D>
where
    D: DeviceImplTrait,
{
    device: D,
    raw_encoder: RawEncoder,
    #[allow(dead_code)]
    max_supported_version: u32,
    codec: Option<Codec>,
    profile: CodecProfile,
    preset: Option<EncodePreset>,
    tuning_info: TuningInfo,
    extra_options: ExtraOptions,
}

impl<D> EncoderBuilder<D>
where
    D: DeviceImplTrait,
{
    /// Create a new `EncoderBuilder` using the given device.
    pub fn new<I: IntoDevice<Device = D>>(device: I) -> Result<Self> {
        let library = Library::load()?;

        let max_supported_version = library.get_max_supported_version()?;

        if !is_version_supported(max_supported_version) {
            return Err(NvEncError::UnsupportedVersion);
        }

        let device = device.into_device();
        let raw_encoder = RawEncoder::new(&device, library)?;

        Ok(EncoderBuilder {
            device,
            raw_encoder,
            max_supported_version,
            codec: None,
            profile: CodecProfile::Autoselect,
            preset: None,
            tuning_info: TuningInfo::Undefined,
            extra_options: ExtraOptions::default(),
        })
    }

    /// Set the encoder to use the given `Codec`.
    pub fn with_codec(&mut self, codec: Codec) -> Result<&mut Self> {
        if self.supported_codecs()?.contains(&codec) {
            self.codec = Some(codec);
            Ok(self)
        } else {
            Err(NvEncError::UnsupportedCodec)
        }
    }

    /// Set the encoder to use the given `CodecProfile`.
    pub fn with_codec_profile(&mut self, profile: CodecProfile) -> Result<&mut Self> {
        if self
            .supported_codec_profiles(self.codec.ok_or(NvEncError::CodecNotSet)?)?
            .contains(&profile)
        {
            self.profile = profile;
            Ok(self)
        } else {
            Err(NvEncError::CodecProfileNotSupported)
        }
    }

    /// Set the encoder to use the given `EncodePreset`.
    pub fn with_encode_preset(&mut self, preset: EncodePreset) -> Result<&mut Self> {
        if self
            .supported_encode_presets(self.codec.ok_or(NvEncError::CodecNotSet)?)?
            .contains(&preset)
        {
            self.preset = Some(preset);
            Ok(self)
        } else {
            Err(NvEncError::CodecProfileNotSupported)
        }
    }

    /// Set the encoder to use the given `TuningInfo`.
    pub fn with_tuning_info(&mut self, tuning_info: TuningInfo) -> Result<&mut Self> {
        self.tuning_info = tuning_info;
        Ok(self)
    }

    /// Disable writing SPS/PPS (H.264) or VPS/SPS/PPS (HEVC) in the bitstream. Default is enabled.
    pub fn inband_csd(&mut self, enable: bool) -> Result<&mut Self> {
        self.extra_options.inband_csd(enable);
        Ok(self)
    }

    /// Enable writing SPS/PPS (H.264) or VPS/SPS/PPS (HEVC) every IDR frame. Default is disabled.
    pub fn repeat_csd(&mut self, enable: bool) -> Result<&mut Self> {
        self.extra_options.repeat_csd(enable);
        Ok(self)
    }

    /// Enable spatial adaptive quantization. Default is disabled.
    pub fn spatial_aq(&mut self, enable: bool) -> Result<&mut Self> {
        self.extra_options.spatial_aq(enable);
        Ok(self)
    }

    /// Enable zero-reorder delay. Default is disabled.
    pub fn zero_reorder_delay(&mut self, enable: bool) -> Result<&mut Self> {
        self.extra_options.zero_reorder_delay(enable);
        Ok(self)
    }

    /// Configure with the given `MultiPassSetting`.
    pub fn set_multi_pass(&mut self, multi_pass: MultiPassSetting) -> Result<&mut Self> {
        self.extra_options.set_multi_pass(multi_pass);
        Ok(self)
    }

    /// Enable filler data insertion. Frame rate needs to be supplied to determine the rate at
    /// which the filler is generated. Pass `None` to disable this option. Default is disabled.
    pub fn filler_data_insertion(&mut self, frame_rate: Option<(u32, u32)>) -> Result<&mut Self> {
        self.extra_options.filler_data_insertion(frame_rate);
        Ok(self)
    }

    /// Sets the display aspect ratio. Only needs to be set for non-square pixel ratios.
    pub fn display_aspect_ratio(
        &mut self,
        display_aspect_ratio: Option<(u32, u32)>,
    ) -> Result<&mut Self> {
        self.extra_options
            .display_aspect_ratio(display_aspect_ratio);
        Ok(self)
    }

    /// Build the encoder.
    pub fn build(
        self,
        width: u32,
        height: u32,
        texture_format: <D::Buffer as TextureBufferImplTrait>::TextureFormat,
    ) -> Result<(EncoderInput<D>, EncoderOutput)> {
        let codec = self.codec.ok_or(NvEncError::CodecNotSet)?;
        let profile = self.profile;
        let preset = self.preset.ok_or(NvEncError::EncodePresetNotSet)?;
        let tuning_info = self.tuning_info;

        let mut encode_params = EncodeParams::new(
            &self.raw_encoder,
            width,
            height,
            &texture_format,
            codec,
            profile,
            preset,
            tuning_info,
            &self.extra_options,
            D::params_require_buffer_format(),
        )?;

        encode_params.initialize_encoder(&self.raw_encoder)?;

        let texture_buffer =
            self.device
                .create_texture_buffer(width, height, texture_format, BUFFER_SIZE as u32)?;

        let (writer, reader) = encoder_channel(self.raw_encoder, &texture_buffer)?;

        let encoder_input = EncoderInput::new(self.device, writer, texture_buffer, encode_params)?;
        let encoder_output = EncoderOutput::new(reader);
        Ok((encoder_input, encoder_output))
    }

    /// List all supported codecs (H.264, HEVC, etc.).
    pub fn supported_codecs(&self) -> Result<Vec<Codec>> {
        let codec_guid_count = unsafe {
            let mut tmp = MaybeUninit::uninit();
            self.raw_encoder.get_encode_guid_count(tmp.as_mut_ptr())?;
            tmp.assume_init()
        };

        let mut codec_guids = Vec::with_capacity(codec_guid_count as usize);
        let mut num_entries = MaybeUninit::uninit();
        unsafe {
            self.raw_encoder.get_encode_guids(
                codec_guids.as_mut_ptr(),
                codec_guid_count,
                num_entries.as_mut_ptr(),
            )?;
            codec_guids.set_len(num_entries.assume_init() as usize);
        }

        let codecs = codec_guids.iter().map(|guid| (*guid).into()).collect();
        Ok(codecs)
    }

    /// Lists the profiles available for a codec.
    pub fn supported_codec_profiles(&self, codec: Codec) -> Result<Vec<CodecProfile>> {
        let codec = codec.into();
        let profile_guid_count = unsafe {
            let mut tmp = MaybeUninit::uninit();
            self.raw_encoder
                .get_encode_profile_guid_count(codec, tmp.as_mut_ptr())?;
            tmp.assume_init()
        };

        let mut profile_guids = Vec::with_capacity(profile_guid_count as usize);
        let mut num_entries = MaybeUninit::uninit();
        unsafe {
            self.raw_encoder.get_encode_profile_guids(
                codec,
                profile_guids.as_mut_ptr(),
                profile_guid_count,
                num_entries.as_mut_ptr(),
            )?;
            profile_guids.set_len(num_entries.assume_init() as usize);
        }

        let codec_profiles = profile_guids.iter().map(|guid| (*guid).into()).collect();
        Ok(codec_profiles)
    }

    /// Lists the encode presets available for a codec.
    pub fn supported_encode_presets(&self, codec: Codec) -> Result<Vec<EncodePreset>> {
        let codec = codec.into();
        let preset_guid_count = unsafe {
            let mut tmp = MaybeUninit::uninit();
            self.raw_encoder
                .get_encode_preset_count(codec, tmp.as_mut_ptr())?;
            tmp.assume_init()
        };

        let mut preset_guids = Vec::with_capacity(preset_guid_count as usize);
        let mut num_entries = MaybeUninit::uninit();
        unsafe {
            self.raw_encoder.get_encode_preset_guids(
                codec,
                preset_guids.as_mut_ptr(),
                preset_guid_count,
                num_entries.as_mut_ptr(),
            )?;
            preset_guids.set_len(num_entries.assume_init() as usize);
        }

        let presets = preset_guids.iter().map(|guid| (*guid).into()).collect();
        Ok(presets)
    }

    /// Lists the supported input formats for a given codec.
    pub fn supported_input_formats(
        &self,
        codec: Codec,
    ) -> Result<Vec<crate::sys::NV_ENC_BUFFER_FORMAT>> {
        let codec = codec.into();
        let mut tmp = MaybeUninit::uninit();
        let input_format_count = unsafe {
            self.raw_encoder
                .get_input_format_count(codec, tmp.as_mut_ptr())?;
            tmp.assume_init()
        };

        let mut input_formats = Vec::with_capacity(input_format_count as usize);
        let mut num_entries = MaybeUninit::uninit();
        unsafe {
            self.raw_encoder.get_input_formats(
                codec,
                input_formats.as_mut_ptr(),
                input_format_count,
                num_entries.as_mut_ptr(),
            )?;
            input_formats.set_len(num_entries.assume_init() as usize);
        }
        Ok(input_formats)
    }
}
