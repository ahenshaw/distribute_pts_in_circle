use clap::{Parser, Subcommand};
use printpdf::{
    calculate_points_for_circle, Color, Mm, PdfDocument, Polygon, PolygonMode, Rgb, WindingOrder,
};
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
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Svg {
        /// SVG file path
        output: PathBuf,
        /// dot size
        #[clap(short = 'd', long, default_value_t = 5.0)]
        dotsize: f64,
    },
    Pdf {
        /// PDF file path
        output: PathBuf,
        /// dot size (in mm)
        #[clap(short = 'd', long, default_value_t = 0.5)]
        dotsize: f32,
        /// circle radius (in mm)
        #[clap(short = 'r', long, default_value_t = 75.0)]
        radius: f32,
    },
}

fn main() {
    let args = Args::parse();
    let pts = sunflower(args.n, args.alpha);
    match &args.commands {
        Commands::Svg { output, dotsize } => write_to_svg(&output, &pts, *dotsize),
        Commands::Pdf {
            output,
            radius,
            dotsize,
        } => write_to_pdf(&output, &pts, *radius, *dotsize),
    }
}

fn pdf_circle(x: f32, y: f32, r: f32, mode: PolygonMode) -> Polygon {
    let circle = Polygon {
        rings: vec![calculate_points_for_circle(Mm(r), Mm(x), Mm(y))],
        mode,
        winding_order: WindingOrder::NonZero,
    };
    circle
}
fn write_to_pdf(outpath: &Path, pts: &Points, radius: f32, dotsize: f32) {
    let margin = 10.0;
    let (doc, page1, layer1) = PdfDocument::new(
        "Points in a circle".to_string(),
        Mm((radius + dotsize + margin) * 2.0),
        Mm((radius + dotsize + margin) * 2.0),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);
    current_layer.set_outline_thickness(0.05);
    current_layer.set_outline_color(Color::Rgb(Rgb::new(0.75, 0.75, 0.75, None)));
    let circle = pdf_circle(
        radius + dotsize + margin,
        radius + dotsize + margin,
        radius,
        PolygonMode::Stroke,
    );
    current_layer.add_polygon(circle);

    // current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    let offset = radius + dotsize + margin;
    for (x, y) in pts {
        let dot = pdf_circle(
            (*x as f32) * radius + offset,
            (*y as f32) * radius + offset,
            dotsize,
            PolygonMode::Fill,
        );
        current_layer.add_polygon(dot);
    }

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
            let r = calc_radius(k, n, b);
            let theta = angle_stride * (k as f64);
            (r * theta.cos(), r * theta.sin())
        })
        .collect()
}

fn calc_radius(k: usize, n: usize, b: usize) -> f64 {
    if k > n - b {
        1.0
    } else {
        let k = k as f64;
        let b = b as f64;
        let n = n as f64;
        ((k - 0.5) / (n - (b + 1.0) / 2.0)).sqrt()
    }
}
