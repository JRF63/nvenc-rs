use super::{raw_encoder::RawEncoder, texture::IntoNvEncBufferFormat};
use crate::{Codec, CodecProfile, EncodePreset, MultiPassSetting, Result, TuningInfo};
use std::{mem::MaybeUninit, ptr::addr_of_mut};

#[repr(transparent)]
pub struct EncodeParams(crate::sys::NV_ENC_RECONFIGURE_PARAMS);

impl Drop for EncodeParams {
    fn drop(&mut self) {
        let ptr = self.0.reInitEncodeParams.encodeConfig;
        debug_assert!(
            !ptr.is_null(),
            "reInitEncodeParams.encodeConfig should not be null"
        );

        // SAFETY: The pointer was allocated by `Box::new` inside `get_codec_config_for_preset`
        let boxed = unsafe { Box::from_raw(ptr) };
        std::mem::drop(boxed);
    }
}

impl EncodeParams {
    pub fn new<T: IntoNvEncBufferFormat>(
        raw_encoder: &RawEncoder,
        width: u32,
        height: u32,
        texture_format: &T,
        codec: Codec,
        profile: CodecProfile,
        preset: EncodePreset,
        tuning_info: TuningInfo,
        extra_options: &ExtraOptions,
        params_require_buffer_format: bool,
    ) -> Result<Self> {
        let mut reconfig_params: crate::sys::NV_ENC_RECONFIGURE_PARAMS =
            unsafe { MaybeUninit::zeroed().assume_init() };
        reconfig_params.version = crate::sys::NV_ENC_RECONFIGURE_PARAMS_VER;

        let init_params = &mut reconfig_params.reInitEncodeParams;
        init_params.version = crate::sys::NV_ENC_INITIALIZE_PARAMS_VER;
        init_params.encodeGUID = codec.into();
        init_params.presetGUID = preset.into();
        init_params.encodeWidth = width;
        init_params.encodeHeight = height;
        init_params.enablePTD = 1;
        init_params.tuningInfo = tuning_info.into();

        #[cfg(windows)]
        {
            // The latency is orders of magnitude higher if synchronous encoding mode is used on
            // Windows based both on testing and according to the docs:
            // https://docs.nvidia.com/video-technologies/video-codec-sdk/nvenc-video-encoder-api-prog-guide/
            init_params.enableEncodeAsync = 1;
            init_params.set_enableOutputInVidmem(0);

            // If DirectX12, bufferFormat must be set
            if params_require_buffer_format {
                init_params.bufferFormat = texture_format.into_nvenc_buffer_format();
            }
        }

        // Needs to be called after `encodeWidth` and `encodeHeight` has been initialized
        extra_options.modify_init_params(init_params);

        let encoder_config = build_encode_config(
            raw_encoder,
            texture_format,
            codec,
            profile,
            preset,
            tuning_info,
            extra_options,
        )?;

        init_params.encodeConfig = Box::into_raw(encoder_config);

        Ok(EncodeParams(reconfig_params))
    }

    pub fn initialize_encoder(&mut self, raw_encoder: &RawEncoder) -> Result<()> {
        unsafe { raw_encoder.initialize_encoder(&mut self.0.reInitEncodeParams) }
    }

    pub fn set_average_bitrate(
        &mut self,
        raw_encoder: &RawEncoder,
        bitrate: u32,
        vbv_buffer_size: Option<u32>,
    ) -> Result<()> {
        let ptr = self.0.reInitEncodeParams.encodeConfig;
        debug_assert!(
            !ptr.is_null(),
            "reInitEncodeParams.encodeConfig should not be null"
        );

        let encoder_config = unsafe { &mut *ptr };
        encoder_config.rcParams.averageBitRate = bitrate;
        encoder_config.rcParams.maxBitRate = bitrate;

        if let Some(vbv_buffer_size) = vbv_buffer_size {
            encoder_config.rcParams.vbvBufferSize = vbv_buffer_size;
            encoder_config.rcParams.vbvInitialDelay = vbv_buffer_size;
        }

        unsafe { raw_encoder.reconfigure_encoder(&mut self.0) }
    }

    pub fn encode_width(&self) -> u32 {
        self.0.reInitEncodeParams.encodeWidth
    }

    pub fn encode_height(&self) -> u32 {
        self.0.reInitEncodeParams.encodeHeight
    }
}

fn build_encode_config<T: IntoNvEncBufferFormat>(
    raw_encoder: &RawEncoder,
    texture_format: &T,
    codec: Codec,
    profile: CodecProfile,
    preset: EncodePreset,
    tuning_info: TuningInfo,
    extra_options: &ExtraOptions,
) -> Result<Box<crate::sys::NV_ENC_CONFIG>> {
    let mut encode_config = unsafe {
        let mut tmp: MaybeUninit<crate::sys::NV_ENC_PRESET_CONFIG> = MaybeUninit::zeroed();

        let ptr = tmp.as_mut_ptr();

        addr_of_mut!((*ptr).version).write(crate::sys::NV_ENC_PRESET_CONFIG_VER);
        addr_of_mut!((*ptr).presetCfg.version).write(crate::sys::NV_ENC_CONFIG_VER);
        raw_encoder.get_encode_preset_config_ex(
            codec.into(),
            preset.into(),
            tuning_info.into(),
            ptr,
        )?;
        tmp.assume_init().presetCfg
    };

    // Need to set the profile after `NvEncGetEncodePresetConfigEx` because it will get wiped
    // otherwise. A zeroed GUID is a valid value for the profileGUID in which case the encoder
    // autoselects a profile.
    encode_config.profileGUID = profile.into();

    extra_options.modify_encode_config(&mut encode_config);

    let codec_config = &mut encode_config.encodeCodecConfig;

    match codec {
        Codec::H264 => {
            let h264_config = unsafe { &mut codec_config.h264Config.as_mut() };

            extra_options.modify_h264_encode_config(h264_config);

            let nvenc_format = texture_format.into_nvenc_buffer_format();
            h264_config.chromaFormatIDC = chroma_format_idc(&nvenc_format);

            // https://docs.nvidia.com/video-technologies/video-codec-sdk/nvenc-video-encoder-api-prog-guide/
            // Settings for optimal performance when using
            // `IDXGIOutputDuplication::AcquireNextFrame`
            #[cfg(windows)]
            {
                h264_config.set_enableFillerDataInsertion(0);
                h264_config.set_outputBufferingPeriodSEI(0);
                h264_config.set_outputPictureTimingSEI(0);
                h264_config.set_outputAUD(0);
                h264_config.set_outputFramePackingSEI(0);
                h264_config.set_outputRecoveryPointSEI(0);
                h264_config.set_enableScalabilityInfoSEI(0);
                h264_config.set_disableSVCPrefixNalu(1);
            }
        }
        Codec::Hevc => {
            let hevc_config = unsafe { &mut codec_config.hevcConfig.as_mut() };

            extra_options.modify_hevc_encode_config(hevc_config);

            let nvenc_format = texture_format.into_nvenc_buffer_format();
            hevc_config.set_chromaFormatIDC(chroma_format_idc(&nvenc_format));
            hevc_config.set_pixelBitDepthMinus8(pixel_bit_depth_minus_8(&nvenc_format));

            // Same settings needed for `AcquireNextFrame`
            #[cfg(windows)]
            {
                hevc_config.set_enableFillerDataInsertion(0);
                hevc_config.set_outputBufferingPeriodSEI(0);
                hevc_config.set_outputPictureTimingSEI(0);
                hevc_config.set_outputAUD(0);
                hevc_config.set_enableAlphaLayerEncoding(0);
            }
        }
    }

    Ok(Box::new(encode_config))
}

pub struct ExtraOptions {
    inband_csd_disabled: u32,
    csd_should_repeat: u32,
    spatial_aq_enabled: u32,
    zero_reorder_delay_enabled: u32,
    multi_pass: MultiPassSetting,
    filler_data_frame_rate: Option<(u32, u32)>,
    filler_data_enabled: u32,
    display_aspect_ratio: Option<(u32, u32)>,
}

impl Default for ExtraOptions {
    fn default() -> Self {
        Self {
            inband_csd_disabled: 0,
            csd_should_repeat: 0,
            spatial_aq_enabled: 0,
            zero_reorder_delay_enabled: 0,
            multi_pass: MultiPassSetting::Disabled,
            filler_data_frame_rate: None,
            filler_data_enabled: 0,
            display_aspect_ratio: None,
        }
    }
}

impl ExtraOptions {
    pub(crate) fn inband_csd(&mut self, enable: bool) {
        // Reverse 0/1 here
        self.inband_csd_disabled = if enable { 0 } else { 1 };
    }

    pub(crate) fn repeat_csd(&mut self, enable: bool) {
        self.csd_should_repeat = if enable { 1 } else { 0 };
    }

    pub(crate) fn spatial_aq(&mut self, enable: bool) {
        self.spatial_aq_enabled = if enable { 1 } else { 0 };
    }

    pub(crate) fn zero_reorder_delay(&mut self, enable: bool) {
        self.zero_reorder_delay_enabled = if enable { 1 } else { 0 };
    }

    pub(crate) fn set_multi_pass(&mut self, multi_pass: MultiPassSetting) {
        self.multi_pass = multi_pass;
    }

    pub(crate) fn filler_data_insertion(&mut self, frame_rate: Option<(u32, u32)>) {
        self.filler_data_frame_rate = frame_rate;
        self.filler_data_enabled = if frame_rate.is_some() { 1 } else { 0 };
    }

    pub(crate) fn display_aspect_ratio(&mut self, display_aspect_ratio: Option<(u32, u32)>) {
        self.display_aspect_ratio = display_aspect_ratio;
    }

    fn modify_init_params(&self, init_params: &mut crate::sys::NV_ENC_INITIALIZE_PARAMS) {
        if let Some((frame_rate_num, frame_rate_den)) = self.filler_data_frame_rate {
            init_params.frameRateNum = frame_rate_num;
            init_params.frameRateDen = frame_rate_den;
        }

        let (dar_width, dar_height) = match self.display_aspect_ratio {
            Some(pair) => pair,
            None => {
                // Assume square pixels
                let width = init_params.encodeWidth;
                let height = init_params.encodeHeight;
                let gcd = crate::util::gcd(width, height);
                (width / gcd, height / gcd)
            }
        };
        init_params.darWidth = dar_width;
        init_params.darHeight = dar_height;
    }

    fn modify_encode_config(&self, config: &mut crate::sys::NV_ENC_CONFIG) {
        config.rcParams.set_enableAQ(self.spatial_aq_enabled);
        config
            .rcParams
            .set_zeroReorderDelay(self.zero_reorder_delay_enabled);
        config.rcParams.multiPass = self.multi_pass.into();
    }

    fn modify_h264_encode_config(&self, h264_config: &mut crate::sys::NV_ENC_CONFIG_H264) {
        h264_config.set_disableSPSPPS(self.inband_csd_disabled);
        h264_config.set_repeatSPSPPS(self.csd_should_repeat);
        h264_config.set_enableFillerDataInsertion(self.filler_data_enabled);
    }

    fn modify_hevc_encode_config(&self, hevc_config: &mut crate::sys::NV_ENC_CONFIG_HEVC) {
        hevc_config.set_disableSPSPPS(self.inband_csd_disabled);
        hevc_config.set_repeatSPSPPS(self.csd_should_repeat);
        hevc_config.set_enableFillerDataInsertion(self.filler_data_enabled);
    }
}

fn pixel_bit_depth_minus_8(nvenc_format: &crate::sys::NV_ENC_BUFFER_FORMAT) -> u32 {
    // Ignore 10-bit RGB formats:
    //
    // https://github.com/NVIDIA/video-sdk-samples/blob/aa3544dcea2fe63122e4feb83bf805ea40e58dbe/nvEncBroadcastSample/nvEnc/nvCodec/nvEncoder/NvEncoder.cpp#L200
    match nvenc_format {
        crate::sys::NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_YUV420_10BIT
        | crate::sys::NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_YUV444_10BIT => 2,
        _ => 0,
    }
}

fn chroma_format_idc(nvenc_format: &crate::sys::NV_ENC_BUFFER_FORMAT) -> u32 {
    // Contrary to the header that says YUV420 should have chromaFormatIDC = 1, the video SDK
    // sample only changes the chromaFormatIDC for YUV444 and YUV444_10BIT:
    //
    // https://github.com/NVIDIA/video-sdk-samples/blob/aa3544dcea2fe63122e4feb83bf805ea40e58dbe/nvEncBroadcastSample/nvEnc/nvCodec/nvEncoder/NvEncoder.cpp#L189
    //
    // What should be done is to set chromaFormatIDC to 3 for YUV444 and to 1 otherwise (even for
    // non-YUV formats like RGB). Calls to `NvEncGetEncodePresetConfigEx` automatically sets
    // chromaFormatIDC to 1 so *perhaps* zero is not a valid value for chromaFormatIDC.
    match nvenc_format {
        crate::sys::NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_YUV444
        | crate::sys::NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_YUV444_10BIT => 3,
        _ => 1,
    }
}
