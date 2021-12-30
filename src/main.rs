use clap::Arg;
use rayon::prelude::*;
use std::{
    error::Error,
    ops::{Index, IndexMut},
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("Forest fires")
        .author("Ivanek R. <richard.ivanek@gmail.com>")
        .version("1.0")
        .args(&[
            Arg::with_name("SIDE")
                .index(1)
                .help("Lattice side length")
                .required(true),
            Arg::with_name("resolution")
                .short("r")
                .long("resolution")
                .help("How many equidistant points to plot in the 0-1 interval")
                .required(false)
                .takes_value(true),
            Arg::with_name("sample")
                .short("s")
                .long("sample")
                .help("Statistical sample size")
                .required(false)
                .takes_value(true),
        ])
        .get_matches();

    // Load arguments
    let n: usize = matches.value_of("SIDE").unwrap().parse()?;
    let resolution: usize = if let Some(val) = matches.value_of("resolution") {
        val.parse()?
    } else {
        100
    };
    let sample_size: usize = if let Some(val) = matches.value_of("sample") {
        val.parse()?
    } else {
        10_000
    };

    // Check argument validity
    if n == 0 {
        eprintln!("You must set a non-zero lattice");
        return Ok(());
    }

    if resolution <= 2 {
        eprintln!("The resolution must be higher than 2");
        return Ok(());
    }

    // Run for each probability point parallelly
    let result: Vec<(f64, f64)> = (0..=resolution)
        .into_par_iter()
        .map(|pidx| {
            // Calculate equidistant prob. points
            let p = pidx as f64 / (resolution as f64);

            (
                p,
                // Parallelly calculate independent samples
                (0..sample_size)
                    .into_par_iter()
                    .map(|_| {
                        let mut lattice = Lattice::generate(n, p);
                        let mut sweeps = 0;

                        while let SweepResult::Ignited = lattice.sweep() {
                            sweeps += 1;
                        }

                        sweeps as f64
                    })
                    .sum::<f64>()
                    / (sample_size as f64))
        })
        .collect();

    // Print the result to `stdout`
    for (p, t) in result {
        println!("{:.4}\t{:.5}", p, t);
    }

    Ok(())
}

/// What resides at a point of a lattice
#[derive(Clone, Copy)]
enum LatticePoint {
    Empty,
    Tree,
    Burning,
}

/// Did the sweep result in a new burning tree?
enum SweepResult {
    /// No new burning tree
    Identity,
    /// One or more trees were ignited
    Ignited,
}

/// A structure defining the lattice
struct Lattice {
    side: usize,
    current: Box<[LatticePoint]>,
}

impl Lattice {
    /// Generate a new lattice of size `n*n`
    /// 
    /// `n` - the size of a side
    /// 
    /// `p` - the occupation probability
    pub fn generate(n: usize, p: f64) -> Self {
        let field = (0..(n * n))
            .map(|i| {
                if rand::random::<f64>() < p {
                    if i < n {
                        // Ignite the first row
                        LatticePoint::Burning
                    } else {
                        LatticePoint::Tree
                    }
                } else {
                    LatticePoint::Empty
                }
            })
            .collect();

        Self {
            side: n,
            current: field,
        }
    }

    /// Perform a sweep
    pub fn sweep(&mut self) -> SweepResult {
        let mut result = SweepResult::Identity;

        let side = self.side;

        for i in 0..side { // Row
            for j in 0..side { // Col
                if let LatticePoint::Tree = self[(i, j)] {
                    let should_burn = (i > 0 && matches!(self[(i - 1, j)], LatticePoint::Burning))
                        || (i < side - 1 && matches!(self[(i + 1, j)], LatticePoint::Burning))
                        || (j > 0 && matches!(self[(i, j - 1)], LatticePoint::Burning))
                        || (j < side - 1 && matches!(self[(i, j + 1)], LatticePoint::Burning));

                    if should_burn {
                        self[(i, j)] = LatticePoint::Burning;
                        result = SweepResult::Ignited;
                    }
                }
            }
        }

        result
    }
}

impl Index<(usize, usize)> for Lattice {
    type Output = LatticePoint;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.current[row * self.side + col]
    }
}

impl IndexMut<(usize, usize)> for Lattice {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.current[row * self.side + col]
    }
}
