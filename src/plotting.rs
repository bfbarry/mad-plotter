use crate::win;

pub fn linspace(start: f32, end: f32, num: usize) -> Vec<f32> {
    let step = (end - start) / (num as f32 - 1.0);
    (0..num).map(|i| start + i as f32 * step).collect()
}

pub fn transform(x: &Vec<f32>, func: fn(f32) -> f32) -> Vec<f32> {
    x.iter().map(|&i| func(i)).collect()
}

// 2 4 6 8 10
pub fn plot(xs: Vec<f32>, ys: Vec<f32>) {
    pollster::block_on(win::run(xs, ys));
}