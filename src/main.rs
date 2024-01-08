use clap::{Parser, Subcommand};
use printpdf::{Mm, PdfDocument, Point as PdfPoint, Polygon, PolygonMode, WindingOrder};
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use svg::node::element::Circle;
use svg::Document;
type Point = (f64, f64);
type Points = Vec<Point>;

/// Distribute points in a circle
#[derive(Parser)]
#[clap(name = "distribute-pts-in-circle", version)]
struct Args {
    /// number of points
    n: usize,
    /// boundary evenness
    #[clap(short = 'a', long, default_value_t = 0.0)]
    alpha: f64,
    /// dot size
    #[clap(short = 'd', long, default_value_t = 5.0)]
    dotsize: f64,
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Svg {
        /// SVG file path
        output: PathBuf,
    },
    Pdf {
        /// PDF file path
        output: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    let pts = sunflower(args.n, args.alpha);
    match &args.commands {
        Commands::Svg { output } => write_to_svg(&output, &pts, args.dotsize),
        Commands::Pdf { output } => write_to_pdf(&output, &pts, args.dotsize),
    }
}

fn write_to_pdf(outpath: &Path, pts: &Points, _dotsize: f64) {
    let (doc, page1, layer1) = PdfDocument::new(
        "Points in a circle".to_string(),
        Mm(297.0),
        Mm(210.0),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![
        (PdfPoint::new(Mm(100.0), Mm(100.0)), false),
        (PdfPoint::new(Mm(100.0), Mm(200.0)), false),
        (PdfPoint::new(Mm(300.0), Mm(200.0)), false),
        (PdfPoint::new(Mm(300.0), Mm(100.0)), false),
    ];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Polygon {
        rings: vec![points1],
        mode: PolygonMode::Stroke,
        winding_order: WindingOrder::NonZero,
    };
    current_layer.add_polygon(line1);
    doc.save(&mut BufWriter::new(File::create(outpath).unwrap()))
        .unwrap();
    println!("Created PDF ({:?}) with {} points", outpath, pts.len());
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
    println!("Created SVG ({:?}) with {} points", outpath, pts.len());
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
