//! Complex FFT using `microfft::complex`. Results should be equal to the ones from `rustfft`.
//! The difference is that this implementation works in `no_std`-environments but it is
//! limited to a maximum sample length of 4096.

use alloc::vec::Vec;

use microfft::{Complex32, complex};
use crate::fft::Fft;
use core::convert::TryInto;

pub type FftResultType = Complex32;

/// Dummy struct with no properties but used as a type
/// to implement a concrete FFT strategy using (`microfft::complex`).
pub struct FftImpl;

impl FftImpl {
    /// Converts all samples to a complex number (imaginary part is set to zero)
    /// as preparation for the FFT.
    ///
    /// ## Parameters
    /// `samples` Input samples.
    ///
    /// ## Return value
    /// New vector with elements of FFT output/result.
    #[inline(always)]
    fn samples_to_complex(samples: &[f32]) -> Vec<Complex32> {
        samples
            .iter()
            .map(|x| Complex32::new(x.clone(), 0.0))
            .collect::<Vec<Complex32>>()
    }
}

impl Fft<FftResultType> for FftImpl {

    #[inline(always)]
    fn fft_apply(samples: &[f32]) -> Vec<FftResultType> {
        let buffer = Self::samples_to_complex(samples);

        if buffer.len() == 2 {
            let mut buffer: [_; 2] = buffer.try_into().unwrap();
            complex::cfft_2(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 4 {
            let mut buffer: [_; 4] = buffer.try_into().unwrap();
            complex::cfft_4(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 8 {
            let mut buffer: [_; 8] = buffer.try_into().unwrap();
            complex::cfft_8(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 16 {
            let mut buffer: [_; 16] = buffer.try_into().unwrap();
            complex::cfft_16(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 32 {
            let mut buffer: [_; 32] = buffer.try_into().unwrap();
            complex::cfft_32(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 64 {
            let mut buffer: [_; 64] = buffer.try_into().unwrap();
            complex::cfft_64(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 128 {
            let mut buffer: [_; 128] = buffer.try_into().unwrap();
            complex::cfft_128(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 256 {
            let mut buffer: [_; 256] = buffer.try_into().unwrap();
            complex::cfft_256(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 512 {
            let mut buffer: [_; 512] = buffer.try_into().unwrap();
            complex::cfft_512(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 1024 {
            let mut buffer: [_; 1024] = buffer.try_into().unwrap();
            complex::cfft_1024(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 2048 {
            let mut buffer: [_; 2048] = buffer.try_into().unwrap();
            complex::cfft_2048(&mut buffer);
            buffer.to_vec()
        } else if buffer.len() == 4096 {
            let mut buffer: [_; 4096] = buffer.try_into().unwrap();
            complex::cfft_4096(&mut buffer);
            buffer.to_vec()
        } else {
            panic!("`microfft::complex` only supports powers of 2 between and 4096!");
        }
    }

    #[inline(always)]
    fn fft_map_result_to_f32(val: &FftResultType) -> f32 {
        // calculates sqrt(re*re + im*im), i.e. magnitude of complex number
        let sum = val.re * val.re + val.im * val.im;
        let sqrt = libm::sqrtf(sum);
        debug_assert!(sqrt != f32::NAN, "sqrt is NaN!");
        sqrt
    }

    #[inline(always)]
    fn fft_calc_frequency_resolution(sampling_rate: u32, samples_len: u32) -> f32 {
        sampling_rate as f32 / samples_len as f32
    }

    #[inline(always)]
    fn fft_relevant_res_samples_count(samples_len: usize) -> usize {
        // See https://stackoverflow.com/a/4371627/2891595 for more information as well as
        // https://www.gaussianwaves.com/2015/11/interpreting-fft-results-complex-dft-frequency-bins-and-fftshift/
        //
        // The indices 0 to N/2 (inclusive) are usually the most relevant. Although, index
        // N/2-1 is declared as the last useful one on stackoverflow (because in typical applications
        // Nyquist-frequency + above are filtered out), we include everything here.
        // with 0..=(samples_len / 2) (inclusive) we get all frequencies from 0 to Nyquist theorem.
        //
        // Indices (samples_len / 2)..len() are mirrored/negative. You can also see this here:
        // https://www.gaussianwaves.com/gaussianwaves/wp-content/uploads/2015/11/realDFT_complexDFT.png
        samples_len / 2 + 1
    }
}
