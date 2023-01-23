#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum NvEncError {
    #[error("{}", .0)]
    Sys(NonZeroNvencStatus),

    // TODO: Maybe split these into separate enums
    #[error("The shared library for `nvEncodeAPI64` is not signed and may have been tampered.")]
    LibraryNotSigned,
    #[error("Loading the shared library for `nvEncodeAPI64` failed.")]
    LibraryLoadingFailed,
    #[error("Unable to locate `NvEncodeAPIGetMaxSupportedVersion` in the shared library.")]
    GetMaxSupportedVersionLoadingFailed,
    #[error("Unable to locate `NvEncodeAPICreateInstance` in the shared library.")]
    CreateInstanceLoadingFailed,
    #[error("The installed driver does not support the version of the NvEnc API that this library is compiled with.")]
    UnsupportedVersion,
    #[error("`NvEncodeAPICreateInstance` returned a malformed function list.")]
    MalformedFunctionList,

    #[error("The encoder for the current device does not support the codec")]
    UnsupportedCodec,
    #[error("Codec needs to be set first")]
    CodecNotSet,
    #[error("The encoder does not support the given codec profile for the current codec")]
    CodecProfileNotSupported,
    #[error("Encode preset is needed to build the encoder")]
    EncodePresetNotSet,

    #[error("Failed creating a texture buffer")]
    TextureBufferCreationFailed,

    #[error("Could not create a Windows event object")]
    EventObjectCreationFailed,
    #[error("Error while waiting for the event object to be signaled")]
    EventObjectWaitError,
    #[error("Event timed-out while waiting")]
    EventObjectWaitTimeout,

    #[error("Input has signaled end of stream")]
    EndOfStream,
}

impl NvEncError {
    /// Create a `NvEncError` from a `NVENCSTATUS`. Returns `None` if `status` is
    /// NVENCSTATUS::NV_ENC_SUCCESS.
    #[inline]
    pub fn from_nvenc_status(status: crate::sys::NVENCSTATUS) -> Option<Self> {
        NonZeroNvencStatus::from_nvenc_status(status).map(|status| NvEncError::Sys(status))
    }

    /// Try to convert `NvEncError` into a `NVENCSTATUS`.
    #[inline]
    pub fn into_nvenc_status(self) -> Option<crate::sys::NVENCSTATUS> {
        match self {
            NvEncError::Sys(status) => Some(status.into_nvenc_status()),
            _ => None,
        }
    }
}

#[repr(i32)]
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum NonZeroNvencStatus {
    #[error("No encode capable devices were detected.")]
    NV_ENC_ERR_NO_ENCODE_DEVICE = 1, // Start at 1 to exclude NV_ENC_SUCCESS = 0

    #[error("Devices pass by the client is not supported.")]
    NV_ENC_ERR_UNSUPPORTED_DEVICE,

    #[error("Encoder device supplied by the client is not valid.")]
    NV_ENC_ERR_INVALID_ENCODERDEVICE,

    #[error("Device passed to the API call is invalid.")]
    NV_ENC_ERR_INVALID_DEVICE,

    #[error("Device passed to the API call is no longer available and needs to be reinitialized.")]
    NV_ENC_ERR_DEVICE_NOT_EXIST,

    #[error("One or more of the pointers passed to the API call is invalid.")]
    NV_ENC_ERR_INVALID_PTR,

    #[error("Completion event passed in ::NvEncEncodePicture() call is invalid.")]
    NV_ENC_ERR_INVALID_EVENT,

    #[error("One or more of the parameter passed to the API call is invalid.")]
    NV_ENC_ERR_INVALID_PARAM,

    #[error("An API call was made in wrong sequence/order.")]
    NV_ENC_ERR_INVALID_CALL,

    #[error(
        "API call failed because it was unable to allocate enough memory to perform the requested \
        operation."
    )]
    NV_ENC_ERR_OUT_OF_MEMORY,

    #[error(
        "Encoder has not been initialized with ::NvEncInitializeEncoder() or that initialization \
        has failed."
    )]
    NV_ENC_ERR_ENCODER_NOT_INITIALIZED,

    #[error("Unsupported parameter was passed by the client.")]
    NV_ENC_ERR_UNSUPPORTED_PARAM,

    #[error("::NvEncLockBitstream() failed to lock the output buffer.")]
    NV_ENC_ERR_LOCK_BUSY,

    #[error(
        "Size of the user buffer passed by the client is insufficient for the requested operation."
    )]
    NV_ENC_ERR_NOT_ENOUGH_BUFFER,

    #[error("Invalid struct version was used by the client.")]
    NV_ENC_ERR_INVALID_VERSION,

    #[error("::NvEncMapInputResource() API failed to map the client provided input resource.")]
    NV_ENC_ERR_MAP_FAILED,

    #[error("Encode driver requires more input buffers to produce an output bitstream.")]
    NV_ENC_ERR_NEED_MORE_INPUT,

    #[error("HW encoder is busy encoding and is unable to encode the input.")]
    NV_ENC_ERR_ENCODER_BUSY,

    #[error(
        "Completion event passed in ::NvEncEncodePicture() API has not been registered with \
        encoder driver using ::NvEncRegisterAsyncEvent()."
    )]
    NV_ENC_ERR_EVENT_NOT_REGISTERD,

    #[error("An unknown internal error has occurred.")]
    NV_ENC_ERR_GENERIC,

    #[error(
        "Client is attempting to use a feature that is not available for the license type for the \
        current system."
    )]
    NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY,

    #[error(
        "The client is attempting to use a feature that is not implemented for the current \
        version."
    )]
    NV_ENC_ERR_UNIMPLEMENTED,

    #[error("::NvEncRegisterResource API failed to register the resource.")]
    NV_ENC_ERR_RESOURCE_REGISTER_FAILED,

    #[error(
        "Client is attempting to unregister a resource that has not been successfully registered."
    )]
    NV_ENC_ERR_RESOURCE_NOT_REGISTERED,

    #[error("Client is attempting to unmap a resource that has not been successfully mapped.")]
    NV_ENC_ERR_RESOURCE_NOT_MAPPED,
}

impl NonZeroNvencStatus {
    /// Create a `NonZeroNvencStatus` from a `NVENCSTATUS`. Returns `None` if `status` is
    /// NVENCSTATUS::NV_ENC_SUCCESS.
    #[inline]
    pub fn from_nvenc_status(status: crate::sys::NVENCSTATUS) -> Option<NonZeroNvencStatus> {
        match status {
            crate::sys::NVENCSTATUS::NV_ENC_SUCCESS => None,
            status => Some(unsafe { std::mem::transmute(status) }),
        }
    }

    /// Reinterpret `NonZeroNvencStatus` as a `NVENCSTATUS`.
    #[inline]
    pub fn into_nvenc_status(self) -> crate::sys::NVENCSTATUS {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equiv_to_nvencstatus() {
        let pairs = [
            (
                NonZeroNvencStatus::NV_ENC_ERR_NO_ENCODE_DEVICE,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_NO_ENCODE_DEVICE,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_UNSUPPORTED_DEVICE,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_UNSUPPORTED_DEVICE,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_ENCODERDEVICE,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_ENCODERDEVICE,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_DEVICE,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_DEVICE,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_DEVICE_NOT_EXIST,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_DEVICE_NOT_EXIST,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_PTR,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_PTR,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_EVENT,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_EVENT,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_PARAM,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_PARAM,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_CALL,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_CALL,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_OUT_OF_MEMORY,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_OUT_OF_MEMORY,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_ENCODER_NOT_INITIALIZED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_ENCODER_NOT_INITIALIZED,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_UNSUPPORTED_PARAM,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_UNSUPPORTED_PARAM,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_LOCK_BUSY,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_LOCK_BUSY,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_NOT_ENOUGH_BUFFER,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_NOT_ENOUGH_BUFFER,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INVALID_VERSION,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INVALID_VERSION,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_MAP_FAILED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_MAP_FAILED,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_NEED_MORE_INPUT,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_NEED_MORE_INPUT,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_ENCODER_BUSY,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_ENCODER_BUSY,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_EVENT_NOT_REGISTERD,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_EVENT_NOT_REGISTERD,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_GENERIC,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_GENERIC,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_INCOMPATIBLE_CLIENT_KEY,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_UNIMPLEMENTED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_UNIMPLEMENTED,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_RESOURCE_REGISTER_FAILED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_RESOURCE_REGISTER_FAILED,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_RESOURCE_NOT_REGISTERED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_REGISTERED,
            ),
            (
                NonZeroNvencStatus::NV_ENC_ERR_RESOURCE_NOT_MAPPED,
                crate::sys::NVENCSTATUS::NV_ENC_ERR_RESOURCE_NOT_MAPPED,
            ),
        ];

        for (a, b) in pairs {
            assert_eq!(a.into_nvenc_status(), b);
        }
        for (a, b) in pairs {
            assert_eq!(a, NonZeroNvencStatus::from_nvenc_status(b).unwrap());
        }
    }

    #[test]
    fn nv_enc_success_is_zero() {
        let status = crate::sys::NVENCSTATUS::NV_ENC_SUCCESS;
        assert_eq!(status as i32, 0);
    }

    #[test]
    fn option_none_is_zero() {
        let err: Option<NvEncError> = None;
        let num: i32 = unsafe { std::mem::transmute(err) };
        assert_eq!(num, 0);
    }

    #[test]
    fn error_same_size() {
        assert_eq!(
            std::mem::size_of::<crate::sys::NVENCSTATUS>(),
            std::mem::size_of::<NvEncError>()
        );
    }

    #[test]
    fn option_error_same_size() {
        assert_eq!(
            std::mem::size_of::<Option<NvEncError>>(),
            std::mem::size_of::<NvEncError>()
        );
    }
}
