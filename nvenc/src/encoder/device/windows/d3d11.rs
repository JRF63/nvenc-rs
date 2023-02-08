use crate::{
    encoder::device::{DeviceImplTrait, IntoDevice, TextureBufferImplTrait},
    NvEncError, Result,
};
use windows::{
    core::{InParam, Vtable},
    Win32::Graphics::{
        Direct3D11::{
            ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_BIND_RENDER_TARGET,
            D3D11_CPU_ACCESS_FLAG, D3D11_RESOURCE_MISC_FLAG, D3D11_TEXTURE2D_DESC,
            D3D11_USAGE_DEFAULT,
        },
        Dxgi::Common::DXGI_SAMPLE_DESC,
    },
};

pub struct DirectX11Device {
    device: ID3D11Device,
    immediate_context: ID3D11DeviceContext,
}

impl DeviceImplTrait for DirectX11Device {
    type Buffer = ID3D11Texture2D;
    type Texture = ID3D11Texture2D;

    fn device_type() -> crate::sys::NV_ENC_DEVICE_TYPE {
        crate::sys::NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_DIRECTX
    }

    fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.device.as_raw()
    }

    fn params_require_buffer_format() -> bool {
        false
    }

    fn create_texture_buffer(
        &self,
        width: u32,
        height: u32,
        texture_format: <Self::Buffer as TextureBufferImplTrait>::TextureFormat,
        buf_size: u32,
    ) -> Result<Self::Texture> {
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            // Plain display output has only one mip
            MipLevels: 1,
            ArraySize: buf_size,
            Format: texture_format,
            SampleDesc: DXGI_SAMPLE_DESC {
                // Default sampler mode
                Count: 1,
                // Default sampler mode
                Quality: 0,
            },
            // GPU needs read/write access
            Usage: D3D11_USAGE_DEFAULT,
            // https://github.com/NVIDIA/video-sdk-samples/blob/aa3544dcea2fe63122e4feb83bf805ea40e58dbe/Samples/NvCodec/NvEncoder/NvEncoderD3D11.cpp#L90
            BindFlags: D3D11_BIND_RENDER_TARGET,
            // Don't need to be accessed by the CPU
            CPUAccessFlags: D3D11_CPU_ACCESS_FLAG(0),
            MiscFlags: D3D11_RESOURCE_MISC_FLAG(0),
        };

        // SAFETY: Windows API call
        unsafe {
            self.device
                .CreateTexture2D(&texture_desc, None)
                .map_err(|_| NvEncError::TextureBufferCreationFailed)
        }
    }

    fn copy_texture(
        &self,
        buffer: &Self::Texture,
        texture: Self::Texture,
        subresource_index: usize,
    ) {
        // SAFETY: Windows API call
        unsafe {
            self.immediate_context.CopySubresourceRegion(
                buffer,
                subresource_index as u32,
                0,
                0,
                0,
                InParam::owned(texture.into()), // TODO: Revisit this on next windows-rs versions
                0,
                None,
            );
        }
    }
}

impl IntoDevice for ID3D11Device {
    type Device = DirectX11Device;

    fn into_device(self) -> Self::Device {
        let mut immediate_context = None;

        // SAFETY: Windows API call. `GetImmediateContext` always succeeds.
        unsafe {
            self.GetImmediateContext(&mut immediate_context);
        }
        DirectX11Device {
            device: self,
            immediate_context: immediate_context.unwrap(),
        }
    }
}
