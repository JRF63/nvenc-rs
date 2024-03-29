mod guids;

use guids::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Codec {
    H264,
    Hevc,
}

impl Into<crate::sys::GUID> for Codec {
    fn into(self) -> crate::sys::GUID {
        match self {
            Codec::H264 => NV_ENC_CODEC_H264_GUID,
            Codec::Hevc => NV_ENC_CODEC_HEVC_GUID,
        }
    }
}

impl From<crate::sys::GUID> for Codec {
    fn from(guid: crate::sys::GUID) -> Self {
        match guid {
            NV_ENC_CODEC_H264_GUID => Codec::H264,
            NV_ENC_CODEC_HEVC_GUID => Codec::Hevc,
            _ => panic!("Invalid codec guid"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CodecProfile {
    Autoselect,
    H264Baseline,
    H264Main,
    H264High,
    H264High444,
    H264Stereo,
    H264ProgressiveHigh,
    H264ConstrainedHigh,
    HevcMain,
    HevcMain10,
    HevcFrext,
}

impl Into<crate::sys::GUID> for CodecProfile {
    fn into(self) -> crate::sys::GUID {
        match self {
            CodecProfile::Autoselect => NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID,
            CodecProfile::H264Baseline => NV_ENC_H264_PROFILE_BASELINE_GUID,
            CodecProfile::H264Main => NV_ENC_H264_PROFILE_MAIN_GUID,
            CodecProfile::H264High => NV_ENC_H264_PROFILE_HIGH_GUID,
            CodecProfile::H264High444 => NV_ENC_H264_PROFILE_HIGH_444_GUID,
            CodecProfile::H264Stereo => NV_ENC_H264_PROFILE_STEREO_GUID,
            CodecProfile::H264ProgressiveHigh => NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID,
            CodecProfile::H264ConstrainedHigh => NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID,
            CodecProfile::HevcMain => NV_ENC_HEVC_PROFILE_MAIN_GUID,
            CodecProfile::HevcMain10 => NV_ENC_HEVC_PROFILE_MAIN10_GUID,
            CodecProfile::HevcFrext => NV_ENC_HEVC_PROFILE_FREXT_GUID,
        }
    }
}

impl From<crate::sys::GUID> for CodecProfile {
    fn from(guid: crate::sys::GUID) -> Self {
        match guid {
            NV_ENC_CODEC_PROFILE_AUTOSELECT_GUID => CodecProfile::Autoselect,
            NV_ENC_H264_PROFILE_BASELINE_GUID => CodecProfile::H264Baseline,
            NV_ENC_H264_PROFILE_MAIN_GUID => CodecProfile::H264Main,
            NV_ENC_H264_PROFILE_HIGH_GUID => CodecProfile::H264High,
            NV_ENC_H264_PROFILE_HIGH_444_GUID => CodecProfile::H264High444,
            NV_ENC_H264_PROFILE_STEREO_GUID => CodecProfile::H264Stereo,
            NV_ENC_H264_PROFILE_PROGRESSIVE_HIGH_GUID => CodecProfile::H264ProgressiveHigh,
            NV_ENC_H264_PROFILE_CONSTRAINED_HIGH_GUID => CodecProfile::H264ConstrainedHigh,
            NV_ENC_HEVC_PROFILE_MAIN_GUID => CodecProfile::HevcMain,
            NV_ENC_HEVC_PROFILE_MAIN10_GUID => CodecProfile::HevcMain10,
            NV_ENC_HEVC_PROFILE_FREXT_GUID => CodecProfile::HevcFrext,
            _ => panic!("Invalid codec profile guid"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EncodePreset {
    DefaultPreset,
    Hp,
    Hq,
    Bd,
    LowLatencyDefault,
    LowLatencyHq,
    LowLatencyHp,
    LosslessDefault,
    LosslessHp,
    Streaming,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

impl Into<crate::sys::GUID> for EncodePreset {
    fn into(self) -> crate::sys::GUID {
        match self {
            EncodePreset::DefaultPreset => NV_ENC_PRESET_DEFAULT_GUID,
            EncodePreset::Hp => NV_ENC_PRESET_HP_GUID,
            EncodePreset::Hq => NV_ENC_PRESET_HQ_GUID,
            EncodePreset::Bd => NV_ENC_PRESET_BD_GUID,
            EncodePreset::LowLatencyDefault => NV_ENC_PRESET_LOW_LATENCY_DEFAULT_GUID,
            EncodePreset::LowLatencyHq => NV_ENC_PRESET_LOW_LATENCY_HQ_GUID,
            EncodePreset::LowLatencyHp => NV_ENC_PRESET_LOW_LATENCY_HP_GUID,
            EncodePreset::LosslessDefault => NV_ENC_PRESET_LOSSLESS_DEFAULT_GUID,
            EncodePreset::LosslessHp => NV_ENC_PRESET_LOSSLESS_HP_GUID,
            EncodePreset::Streaming => NV_ENC_PRESET_STREAMING,
            EncodePreset::P1 => NV_ENC_PRESET_P1_GUID,
            EncodePreset::P2 => NV_ENC_PRESET_P2_GUID,
            EncodePreset::P3 => NV_ENC_PRESET_P3_GUID,
            EncodePreset::P4 => NV_ENC_PRESET_P4_GUID,
            EncodePreset::P5 => NV_ENC_PRESET_P5_GUID,
            EncodePreset::P6 => NV_ENC_PRESET_P6_GUID,
            EncodePreset::P7 => NV_ENC_PRESET_P7_GUID,
        }
    }
}

impl From<crate::sys::GUID> for EncodePreset {
    fn from(guid: crate::sys::GUID) -> Self {
        match guid {
            NV_ENC_PRESET_DEFAULT_GUID => EncodePreset::DefaultPreset,
            NV_ENC_PRESET_HP_GUID => EncodePreset::Hp,
            NV_ENC_PRESET_HQ_GUID => EncodePreset::Hq,
            NV_ENC_PRESET_BD_GUID => EncodePreset::Bd,
            NV_ENC_PRESET_LOW_LATENCY_DEFAULT_GUID => EncodePreset::LowLatencyDefault,
            NV_ENC_PRESET_LOW_LATENCY_HQ_GUID => EncodePreset::LowLatencyHq,
            NV_ENC_PRESET_LOW_LATENCY_HP_GUID => EncodePreset::LowLatencyHp,
            NV_ENC_PRESET_LOSSLESS_DEFAULT_GUID => EncodePreset::LosslessDefault,
            NV_ENC_PRESET_LOSSLESS_HP_GUID => EncodePreset::LosslessHp,
            NV_ENC_PRESET_STREAMING => EncodePreset::Streaming,
            NV_ENC_PRESET_P1_GUID => EncodePreset::P1,
            NV_ENC_PRESET_P2_GUID => EncodePreset::P2,
            NV_ENC_PRESET_P3_GUID => EncodePreset::P3,
            NV_ENC_PRESET_P4_GUID => EncodePreset::P4,
            NV_ENC_PRESET_P5_GUID => EncodePreset::P5,
            NV_ENC_PRESET_P6_GUID => EncodePreset::P6,
            NV_ENC_PRESET_P7_GUID => EncodePreset::P7,
            _ => panic!("Invalid encoder preset"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TuningInfo {
    Undefined,
    HighQuality,
    LowLatency,
    UltraLowLatency,
    Lossless,
}

impl Into<crate::sys::NV_ENC_TUNING_INFO> for TuningInfo {
    fn into(self) -> crate::sys::NV_ENC_TUNING_INFO {
        use crate::sys::NV_ENC_TUNING_INFO;
        match self {
            TuningInfo::Undefined => NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_UNDEFINED,
            TuningInfo::HighQuality => NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_HIGH_QUALITY,
            TuningInfo::LowLatency => NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOW_LATENCY,
            TuningInfo::UltraLowLatency => NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
            TuningInfo::Lossless => NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOSSLESS,
        }
    }
}

impl From<crate::sys::NV_ENC_TUNING_INFO> for TuningInfo {
    fn from(tuning_info: crate::sys::NV_ENC_TUNING_INFO) -> Self {
        use crate::sys::NV_ENC_TUNING_INFO;
        match tuning_info {
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_UNDEFINED => TuningInfo::Undefined,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_HIGH_QUALITY => TuningInfo::HighQuality,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOW_LATENCY => TuningInfo::LowLatency,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY => TuningInfo::UltraLowLatency,
            NV_ENC_TUNING_INFO::NV_ENC_TUNING_INFO_LOSSLESS => TuningInfo::Lossless,
            _ => panic!("Invalid tuning info"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MultiPassSetting {
    Disabled,
    QuarterResolution,
    FullResolution,
}

impl Into<crate::sys::NV_ENC_MULTI_PASS> for MultiPassSetting {
    fn into(self) -> crate::sys::NV_ENC_MULTI_PASS {
        use crate::sys::NV_ENC_MULTI_PASS;
        match self {
            MultiPassSetting::Disabled => NV_ENC_MULTI_PASS::NV_ENC_MULTI_PASS_DISABLED,
            MultiPassSetting::QuarterResolution => {
                NV_ENC_MULTI_PASS::NV_ENC_TWO_PASS_QUARTER_RESOLUTION
            }
            MultiPassSetting::FullResolution => NV_ENC_MULTI_PASS::NV_ENC_TWO_PASS_FULL_RESOLUTION,
        }
    }
}

impl From<crate::sys::NV_ENC_MULTI_PASS> for MultiPassSetting {
    fn from(multi_pass: crate::sys::NV_ENC_MULTI_PASS) -> Self {
        use crate::sys::NV_ENC_MULTI_PASS;
        match multi_pass {
            NV_ENC_MULTI_PASS::NV_ENC_MULTI_PASS_DISABLED => MultiPassSetting::Disabled,
            NV_ENC_MULTI_PASS::NV_ENC_TWO_PASS_QUARTER_RESOLUTION => {
                MultiPassSetting::QuarterResolution
            }
            NV_ENC_MULTI_PASS::NV_ENC_TWO_PASS_FULL_RESOLUTION => MultiPassSetting::FullResolution,
            _ => panic!("Invalid multi-pass setting"),
        }
    }
}
