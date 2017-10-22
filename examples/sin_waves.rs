extern crate rustfft;
extern crate gnuplot;

use std::f32;
use std::f64::consts::PI;

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::{FFTplanner};

use gnuplot::{Figure, Caption, Color};

fn sin_wave_signal(length: usize, frequency: f32, amplitude: f32, scale: f32) -> Vec<f32> {
    let mut sig = Vec::with_capacity(length);
    let len = (length) as f32;
    for i in 0..length {
        let x = ((i as f32) / len) * (scale as f32);
        sig.push(amplitude * ((frequency as f32) * (x as f32) * PI as f32 * 2.0).sin());
    }
    return sig;
}

fn plot(x: &Vec<f32>, y: &Vec<f32>, name: &str, output_file: &str) {
    let mut fg = Figure::new();
    fg.set_terminal("pdfcairo", output_file);
    fg.axes2d()
        .lines(x, y, &[Caption(name), Color("black")]);
    fg.show();
}

fn real_to_complex(real: &Vec<f32>) -> Vec<Complex<f32>> {
    return real.iter().map(|&n| Complex{re: n, im: 0.0}).collect();
}

fn compute_fft(signal: &Vec<f32>) -> Vec<Complex<f32>> {
    let mut signal_fft = real_to_complex(signal);
    let mut spectrum_fft = vec![Zero::zero(); signal.len()];
    let inverse = false;
    let mut planner = FFTplanner::new(inverse);
    let fft = planner.plan_fft(signal.len());
    fft.process(&mut signal_fft, &mut spectrum_fft);
    return spectrum_fft;
}

fn compute_amplitude_spectrum(transform: &Vec<Complex<f32>>) -> Vec<f32> {
    let l = transform.len();
    let p2: Vec<f32> = transform.iter()
        .map(|n| n.norm_sqr().sqrt() * 2.0 / l as f32).collect();
    let mut p1 = p2[0..l/2 + 1].to_vec();
    p1[0] = p1[0] / 2.0;
    let end = p1.len() - 1;
    p1[end] = p1[end] / 2.0;
    return p1;
}

fn main() {
    let sample_size = 1500;
    let sampling_freq = 1000.0;
    let freq1 = 20.0;
    let freq2 = 60.0;
    let amp1 = 1.0;
    let amp2 = 0.5;
    let scale = sample_size as f32 / sampling_freq;
    let mut time: Vec<f32> = (0..sample_size).map(|t| t as f32 / sampling_freq).collect();
    let s1 = sin_wave_signal(sample_size, freq1, amp1, scale);
    let s2 = sin_wave_signal(sample_size, freq2, amp2, scale);
    let mut signal = s1.iter().enumerate().map(|(i,si)| si + s2[i]).collect();
    let transform = compute_fft(&signal);
    let mut spectrum = compute_amplitude_spectrum(&transform);
    let mut freq_domain: Vec<f32> = (0..sample_size/2 + 1)
        .map(|f| (f as f32 / sample_size as f32) * sampling_freq).collect();

    // Trim excess to make plot clear
    let trim: usize = (sampling_freq / 6.0) as usize;
    time = time[0..trim].to_vec();
    signal = signal[0..trim].to_vec();
    spectrum = spectrum[0..trim].to_vec();
    freq_domain = freq_domain[0..trim].to_vec();

    plot(&time, &signal, "Signal", "signal-in.pdf");
    plot(&freq_domain, &spectrum, "Frequency", "fft-out.pdf");
}
