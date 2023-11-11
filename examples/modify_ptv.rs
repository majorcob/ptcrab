//! Example of editing a ptvoice loaded from file.

use ptcrab::data::{FromRead, WriteTo};
use ptcrab::ptvoice::{PtvUnit, PtvWave, Ptvoice};

use anyhow::Result as AnyResult;
use std::env::args;
use std::fs::File;

//--------------------------------------------------------------------------------------------------

fn main() -> AnyResult<()> {
    let filename = args().nth(1).expect("input file as command argument");
    let mut file = File::open(filename)?;

    // Load ptvoice from file.
    let mut ptv = Ptvoice::from_read(&mut file)?;
    let mut units = ptv.units.into_vec();
    for (i, unit) in units.iter().enumerate() {
        println!("{i}\t{unit:?}");
    }

    // Write the ptvoice to a new output file.
    ptv.units = units.into_boxed_slice();
    let mut file = File::create("modified.ptvoice")?;
    ptv.write_to(&mut file)?;

    Ok(())
}
