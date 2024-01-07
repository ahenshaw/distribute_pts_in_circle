use argh::FromArgs;
use std::f64::consts::PI;
use std::path::{Path, PathBuf};
use svg::node::element::Circle;
use svg::Document;

type Point = (f64, f64);
type Points = Vec<Point>;

#[derive(FromArgs)]
/// Distribute points in a circle
struct Args {
    /// number of points
    #[argh(positional)]
    n: usize,
    /// boundary evenness
    #[argh(option, short = 'a', default = "0.0")]
    alpha: f64,
    /// output file path
    #[argh(option, short = 'o')]
    output: PathBuf,
}

fn main() {
    let args: Args = argh::from_env();
    let pts = sunflower(args.n, args.alpha);
    write_to_svg(&args.output, &pts, 100.0);
}

fn write_to_svg(outpath: &Path, pts: &Points, radius: f64) {
    let mut document = Document::new().set(
        "viewBox",
        (-0.1 * radius, -0.1 * radius, 2.3 * radius, 2.3 * radius),
    );
    for (x, y) in pts {
        let dot = Circle::new()
            .set("cx", *x * radius + radius)
            .set("cy", *y * radius + radius)
            .set("r", 1.0);

        document = document.add(dot);
    }
    svg::save(outpath, &document).unwrap();
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
