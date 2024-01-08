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
    /// output file path
    #[argh(positional)]
    output: PathBuf,
    /// boundary evenness
    #[argh(option, short = 'a', default = "0.0")]
    alpha: f64,
    /// dot size
    #[argh(option, short = 'd', default = "5.0")]
    dotsize: f64,
}

fn main() {
    let args: Args = argh::from_env();
    let pts = sunflower(args.n, args.alpha);
    write_to_svg(&args.output, &pts, args.dotsize);
}

fn write_to_svg(outpath: &Path, pts: &Points, dotsize: f64) {
    let mut document = Document::new().set("viewBox", (-1010, -1010, 2020, 2020));
    for (x, y) in pts {
        let dot = Circle::new()
            .set("cx", *x * 1000.0)
            .set("cy", *y * 1000.0)
            .set("r", dotsize);

        document = document.add(dot);
    }
    document = document.add(
        Circle::new()
            .set("r", 1000)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "2"),
    );
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
