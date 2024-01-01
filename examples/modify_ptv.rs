//! Example of editing a ptvoice loaded from file.

use ptcrab::data::{FromRead, WriteTo};
use ptcrab::ptvoice::Ptvoice;

use anyhow::Result as AnyResult;
use std::env::args;
use std::fs::File;

//--------------------------------------------------------------------------------------------------

fn main() -> AnyResult<()> {
    let filename = args().nth(1).expect("input file as command argument");
    let mut file = File::open(&filename)?;

    // Load ptvoice from file.
    let mut ptv = Ptvoice::from_read(&mut file)?;
    // Double the volume of each unit.
    for unit in ptv.units.iter_mut() {
        unit.volume = unit.volume * 2.;
    }

    // Write the ptvoice to a new output file.
    let mut file = File::create(format!("modified {filename}"))?;
    ptv.write_to(&mut file)?;

    Ok(())
}
