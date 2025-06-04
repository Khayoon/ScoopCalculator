//! compute the volume (and mass) of a powder scoop (base + mound).

use clap::{ArgEnum, Parser};
use shapes::{Cone, HalfEllipsoid, RectangularPrism, Volumeable};
use scoop::Scoop;

mod shapes;
mod scoop;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum MoundType {
    Ellipsoid,
    Cone,
}

.
#[derive(Parser, Debug)]
#[command(name = "scoop_calculator")]
#[command(about = "Compute volume and mass of a base + mound scoop", long_about = None)]
struct Cli {
    #[arg(long)]
    base_length: f64,

    #[arg(long)]
    base_width: f64,

    #[arg(long, default_value_t = 0.05)]
    base_height: f64,

    #[arg(long, arg_enum)]
    mound_type: MoundType,

    #[arg(long)]
    mound_height: f64,

    #[arg(long)]
    density: f64,
}

fn main() {
    let args = Cli::parse();

    // Create the rectangular base
    let base = RectangularPrism::new(args.base_length, args.base_width, args.base_height);

    // fully loaded or only a modest scoop?
    let mound: Box<dyn Volumeable> = match args.mound_type {
        MoundType::Ellipsoid => {
            // For half-ellipsoid- modest scoop on a flat spatula
            let a = args.base_length / 2.0;
            let b = args.base_width / 2.0;
            let c = args.mound_height;
            Box::new(HalfEllipsoid::new(a, b, c))
        }
        MoundType::Cone => {
            // For cone- big scoop
            let radius = args.base_width.min(args.base_length) / 2.0;
            let height = args.mound_height;
            Box::new(Cone::new(radius, height))
        }
    };

    // imagine a scoop
    let scoop = Scoop::new(Box::new(base), mound);

    // do the math
    let total_vol = scoop.total_volume_in_cubic_inches();
    let mass = scoop.mass_in_grams(args.density);

    println!("--- Scoop Calculator Results ---");
    println!("Base dimensions: {:.3}\" × {:.3}\" × {:.3}\"", 
             args.base_length, args.base_width, args.base_height);
    println!("Mound type: {:?}", args.mound_type);
    println!("Mound height: {:.3}\"", args.mound_height);
    println!("Bulk density: {:.5} g/in³", args.density);
    println!();
    println!("Total volume: {:.5} in³", total_vol);
    println!("Estimated mass: {:.5} g", mass);
}
