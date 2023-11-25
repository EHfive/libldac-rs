use core::ptr::NonNull;

use ldacbt_enc_sys as sys;

use crate::error::*;

type Result<T> = core::result::Result<T, EncoderError>;

const LDACBT_FAIL: i32 = -1;
// these eqmid can only be set indirectly via ldacBT_alter_eqmid_priority()
const LDACBT_EQMID_Q0: u32 = sys::LDACBT_EQMID_MQ + 1;
const LDACBT_EQMID_Q1: u32 = sys::LDACBT_EQMID_MQ + 2;

pub const LDACBT_MTU_REQUIRED: u16 = 679;
pub const LDACBT_ENC_NUM_SAMPLES: usize = 128;
pub const LDACBT_PACKET_MAX_SIZE: usize = 660;

#[derive(Debug)]
struct Handle(NonNull<sys::_st_ldacbt_handle>);
impl Handle {
    fn new() -> Option<Self> {
        unsafe { NonNull::new(sys::ldacBT_get_handle()) }.map(Handle)
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            sys::ldacBT_free_handle(self.0.as_mut());
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SamplingFreq {
    F44100,
    F48000,
    F88200,
    F96000,
}
impl SamplingFreq {
    const fn to_freq_num(self) -> u32 {
        use SamplingFreq::*;
        match self {
            F44100 => 44100,
            F48000 => 48000,
            F88200 => 88200,
            F96000 => 96000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QualityMode {
    /// 990kbps
    Hq = 0,
    /// 660kbps
    Sq,
    /// 492kbps
    Q0,
    /// 396kbps
    Q1,
    /// 330kbps
    Mq,
}
impl QualityMode {
    const fn to_eqmid(self) -> u32 {
        use QualityMode::*;
        match self {
            Hq => sys::LDACBT_EQMID_HQ,
            Sq => sys::LDACBT_EQMID_SQ,
            Mq => sys::LDACBT_EQMID_MQ,
            Q0 => LDACBT_EQMID_Q0,
            Q1 => LDACBT_EQMID_Q1,
        }
    }
    const fn from_eqmid(eqmid: u32) -> Option<Self> {
        use QualityMode::*;
        let mode = match eqmid {
            sys::LDACBT_EQMID_HQ => Hq,
            sys::LDACBT_EQMID_SQ => Sq,
            sys::LDACBT_EQMID_MQ => Mq,
            LDACBT_EQMID_Q0 => Q0,
            LDACBT_EQMID_Q1 => Q1,
            _ => return None,
        };
        Some(mode)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChannelMode {
    Mono,
    DualChannel,
    Stereo,
}
impl ChannelMode {
    const fn num_channels(self) -> usize {
        use ChannelMode::*;
        match self {
            Stereo | DualChannel => 2,
            Mono => 1,
        }
    }

    const fn to_sys(self) -> u32 {
        use ChannelMode::*;
        match self {
            Stereo => sys::LDACBT_CHANNEL_MODE_STEREO,
            DualChannel => sys::LDACBT_CHANNEL_MODE_DUAL_CHANNEL,
            Mono => sys::LDACBT_CHANNEL_MODE_MONO,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SampleFormat {
    S16,
    S24,
    S32,
    F32,
}
impl SampleFormat {
    const fn size(self) -> usize {
        use SampleFormat::*;
        match self {
            S16 => 2,
            S24 | S32 | F32 => 4,
        }
    }

    const fn to_sys(self) -> u32 {
        use SampleFormat::*;
        match self {
            S16 => sys::LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S16,
            S24 => sys::LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S24,
            S32 => sys::LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_S32,
            F32 => sys::LDACBT_SMPL_FMT_T_LDACBT_SMPL_FMT_F32,
        }
    }
}

#[derive(Debug)]
pub struct EncoderOptions {
    pub mtu: u16,
    pub channel_mode: ChannelMode,
    pub sample_format: SampleFormat,
    pub sampling_freq: SamplingFreq,
}

#[derive(Debug)]
pub struct EncodeResult {
    pub input_size: usize,
    pub output_size: usize,
    pub frame_num: usize,
}

#[derive(Debug)]
pub struct Encoder {
    handle: Handle,
    input_buf_size: usize,
}

unsafe fn get_error_code(handle: &mut sys::_st_ldacbt_handle) -> ErrorCode {
    let err = sys::ldacBT_get_error_code(handle);
    ErrorCode::new(err as _)
}

impl Encoder {
    pub fn version() -> (u8, u8, u8) {
        let version = unsafe { sys::ldacBT_get_version() } as u32;
        let major = ((version >> 16) & 0xff) as u8;
        let minor = ((version >> 8) & 0xff) as u8;
        let branch = (version & 0xff) as u8;
        (major, minor, branch)
    }

    pub fn new(options: EncoderOptions) -> Result<Self> {
        let mut handle = Handle::new().ok_or(EncoderError::NoMem)?;
        let res = unsafe {
            sys::ldacBT_init_handle_encode(
                handle.0.as_mut(),
                options.mtu as _,
                sys::LDACBT_EQMID_HQ as _,
                options.channel_mode.to_sys() as _,
                options.sample_format.to_sys() as _,
                options.sampling_freq.to_freq_num() as _,
            )
        };
        if res == LDACBT_FAIL {
            let err = unsafe { get_error_code(handle.0.as_mut()) };
            unsafe { sys::ldacBT_close_handle(handle.0.as_mut()) }
            return Err(err.to_error());
        }

        let input_buf_size = LDACBT_ENC_NUM_SAMPLES
            * options.sample_format.size()
            * options.channel_mode.num_channels();

        Ok(Self {
            handle,
            input_buf_size,
        })
    }

    pub fn sampling_freq(&mut self) -> Result<u32> {
        let res = unsafe { sys::ldacBT_get_sampling_freq(self.lib_handle()) };
        if res == LDACBT_FAIL {
            return Err(self.get_error());
        }
        Ok(res as _)
    }

    pub fn bitrate(&mut self) -> Result<u32> {
        let res = unsafe { sys::ldacBT_get_bitrate(self.lib_handle()) };
        if res == LDACBT_FAIL {
            return Err(self.get_error());
        }
        Ok(res as _)
    }

    pub fn quality_mode(&mut self) -> Result<QualityMode> {
        let res = unsafe { sys::ldacBT_get_eqmid(self.lib_handle()) };
        if res == LDACBT_FAIL {
            return Err(self.get_error());
        }
        QualityMode::from_eqmid(res as _).ok_or(EncoderError::UnhandledEqmid(res))
    }

    pub fn set_quality_mode(&mut self, mode: QualityMode) -> Result<()> {
        let prev_mode = self.quality_mode()?;
        if prev_mode == mode {
            return Ok(());
        }
        if let QualityMode::Hq | QualityMode::Sq | QualityMode::Mq = mode {
            let res = unsafe { sys::ldacBT_set_eqmid(self.lib_handle(), mode.to_eqmid() as _) };
            return if res == LDACBT_FAIL {
                Err(self.get_error())
            } else {
                Ok(())
            };
        }

        let direction = if mode > prev_mode {
            sys::LDACBT_EQMID_INC_CONNECTION
        } else {
            sys::LDACBT_EQMID_INC_QUALITY as i32
        };
        loop {
            let res = unsafe { sys::ldacBT_alter_eqmid_priority(self.lib_handle(), direction) };
            if res == LDACBT_FAIL {
                return Err(self.get_error());
            }
            if self.quality_mode()? == mode {
                break;
            }
        }
        Ok(())
    }

    pub fn input_buffer_size(&self) -> usize {
        self.input_buf_size
    }

    pub fn encode(&mut self, input: Option<&[u8]>, output: &mut [u8]) -> Result<EncodeResult> {
        if input.is_some_and(|i| i.len() < self.input_buf_size)
            || output.len() < LDACBT_PACKET_MAX_SIZE
        {
            return Err(EncoderError::BufSize);
        }
        let mut input_size: i32 = input.map_or(0, |i| i.len() as _);
        let mut output_size: i32 = output.len() as _;
        let mut frame_num: i32 = 0;
        let res = unsafe {
            let input_buf = input.map_or(core::ptr::null(), |i| i.as_ptr());
            sys::ldacBT_encode(
                self.lib_handle(),
                input_buf as *mut _,
                &mut input_size,
                output.as_mut_ptr().cast(),
                &mut output_size,
                &mut frame_num,
            )
        };
        if res == LDACBT_FAIL {
            return Err(self.get_error());
        }
        assert_eq!(output_size == 0, frame_num == 0);
        Ok(EncodeResult {
            input_size: input_size as _,
            output_size: output_size as _,
            frame_num: frame_num as _,
        })
    }

    unsafe fn lib_handle(&mut self) -> &mut sys::_st_ldacbt_handle {
        self.handle.0.as_mut()
    }

    fn get_error(&mut self) -> EncoderError {
        unsafe { get_error_code(self.lib_handle()) }.to_error()
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            sys::ldacBT_close_handle(self.lib_handle());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let (major, minor, branch) = Encoder::version();
        eprintln!("version: {}.{}.{}", major, minor, branch);

        #[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
        {
            assert_eq!(major, 2);
            assert!(minor >= 0);
            assert!(branch >= 2);
        }
    }

    #[test]
    fn quality_mode() {
        use QualityMode::*;
        let mut encoder = Encoder::new(EncoderOptions {
            mtu: 800,
            channel_mode: ChannelMode::Stereo,
            sample_format: SampleFormat::F32,
            sampling_freq: SamplingFreq::F48000,
        })
        .unwrap();

        let seq = [Hq, Mq, Sq, Q0, Hq, Q0, Q1, Q0, Hq];

        for mode in seq {
            encoder.set_quality_mode(mode).unwrap();
            assert_eq!(mode, encoder.quality_mode().unwrap());
        }
    }

    #[test]
    fn options_matrix() {
        use ChannelMode::*;
        use SampleFormat::*;
        use SamplingFreq::*;
        let channel_mode_list = [Mono, DualChannel, Stereo];
        let sample_format_list = [S16, S24, S32, F32];
        let freq_list = [F44100, F48000, F88200, F96000];

        for &channel_mode in channel_mode_list.iter() {
            for &sample_format in sample_format_list.iter() {
                for &sampling_freq in freq_list.iter() {
                    let mut encoder = Encoder::new(EncoderOptions {
                        mtu: LDACBT_MTU_REQUIRED,
                        channel_mode,
                        sample_format,
                        sampling_freq,
                    })
                    .unwrap();
                    assert_eq!(
                        sampling_freq.to_freq_num(),
                        encoder.sampling_freq().unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn mtu() {
        let _encoder = Encoder::new(EncoderOptions {
            mtu: LDACBT_MTU_REQUIRED,
            channel_mode: ChannelMode::Stereo,
            sample_format: SampleFormat::F32,
            sampling_freq: SamplingFreq::F48000,
        })
        .unwrap();

        let encoder = Encoder::new(EncoderOptions {
            mtu: LDACBT_MTU_REQUIRED - 1,
            channel_mode: ChannelMode::Stereo,
            sample_format: SampleFormat::F32,
            sampling_freq: SamplingFreq::F48000,
        });
        assert!(matches!(encoder, Err(EncoderError::MtuSize)));
    }

    #[test]
    fn encode() {
        let mut encoder = Encoder::new(EncoderOptions {
            mtu: LDACBT_MTU_REQUIRED,
            channel_mode: ChannelMode::Stereo,
            sample_format: SampleFormat::S16,
            sampling_freq: SamplingFreq::F48000,
        })
        .unwrap();

        let input = [0u8; 1024];
        let mut output = [0u8; 1024];
        encoder.set_quality_mode(QualityMode::Mq).unwrap();

        let res = encoder.encode(None, &mut output).unwrap();
        eprintln!("{:?}", res);
        let res = encoder.encode(None, &mut output).unwrap();
        eprintln!("{:?}", res);
        for i in 0..6 {
            let EncodeResult {
                input_size,
                output_size,
                frame_num,
            } = encoder.encode(Some(&input), &mut output).unwrap();
            assert_eq!(512, input_size);
            if i == 5 {
                assert_eq!(6, frame_num);
                assert_eq!(660, output_size);
            } else {
                assert_eq!(0, frame_num);
                assert_eq!(0, output_size);
            }
            eprintln!("{} {} {}", input_size, output_size, frame_num);
        }
    }
}
