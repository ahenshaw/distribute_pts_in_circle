use std::f64::consts::PI;

type Point = (f64, f64);
type Points = Vec<Point>;

fn main() {
    let pts = sunflower(10, 0.0);
    dbg!(pts);
}

fn sunflower(n: usize, alpha: f64) -> Points {
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0; // golden ratio
    let angle_stride = 2.0 * PI / phi.powi(2);
    let b = (alpha * (n as f64).sqrt()).round() as usize;
    (1..=n)
        .map(|k| {
            let r = radius(k, n, b);
            let theta = angle_stride * (k as f64);
            (r * theta.cos(), r * theta.sin())
        })
        .collect()
}

fn radius(k: usize, n: usize, b: usize) -> f64 {
    if k > n - b {
        1.0
    } else {
        let k = k as f64;
        let b = b as f64;
        let n = n as f64;
        ((k - 0.5) / (n - (b + 1.0) / 2.0)).sqrt()
    }
}
