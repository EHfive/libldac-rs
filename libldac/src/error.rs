use ldacbt_enc_sys as sys;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct ErrorCode {
    api: u32,
    handle: u32,
    block: u32,
}

impl ErrorCode {
    pub fn new(err: u32) -> Self {
        let api = (err >> 20) & 0x0fff;
        let handle = (err >> 10) & 0x03ff;
        let block = err & 0x03ff;
        Self { api, handle, block }
    }

    pub fn api(&self) -> u32 {
        self.api
    }

    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn block(&self) -> u32 {
        self.block
    }
}

impl ErrorCode {
    pub(crate) fn to_error(self) -> EncoderError {
        use EncoderError::*;
        'api: {
            return match self.api {
                sys::LDACBT_ERR_FATAL => break 'api,
                sys::LDACBT_ERR_ILL_EQMID => Eqmid,
                sys::LDACBT_ERR_ASSERT_CHANNEL_MODE => ChannelMode,
                sys::LDACBT_ERR_ILL_SMPL_FORMAT => SampleFormat,
                sys::LDACBT_ERR_ILL_SAMPLING_FREQ => SamplingFreq,
                _ => Unhandled(self),
            };
        }
        'handle: {
            return match self.handle {
                0 => break 'handle,
                _ => Unhandled(self),
            };
        }

        Unhandled(self)
    }
}

#[derive(Error, Debug)]
pub enum EncoderError {
    #[error("out of memory")]
    NoMem,
    #[error("unhandled EQMID {0}")]
    UnhandledEqmid(i32),
    #[error("buffer size too small")]
    BufSize,
    #[error("illegal EQMID")]
    Eqmid,
    #[error("illegal channel mode")]
    ChannelMode,
    #[error("illegal sample format")]
    SampleFormat,
    #[error("illegal sampling frequency")]
    SamplingFreq,
    #[error("unhandled error {0:?}")]
    Unhandled(ErrorCode),
}
