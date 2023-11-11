//! Example of creating a ptvoice from scratch.

use ptcrab::data::WriteTo;
use ptcrab::ptvoice::{PtvEnvelope, PtvUnit, PtvWave, Ptvoice};
use ptcrab::Key;

use anyhow::Result as AnyResult;
use std::env::args;
use std::fs::File;

//--------------------------------------------------------------------------------------------------

fn main() -> AnyResult<()> {
    // First unit: a coordinate wave that fades out over 1 second.
    let unit_1 = PtvUnit {
        inherent_key: Key::A6,
        wave: Some(PtvWave::Coordinate {
            points: Box::new([(0, 0)]),
            x_width: 256,
        }),
        ..Default::default()
    };
    // Second unit: an oscillator pitched 7 semitones above the first unit.
    let unit_2 = PtvUnit {
        inherent_key: Key::approx_from_a6_semis(7.),
        wave: Some(PtvWave::Oscillator {
            overtones: Box::new([(1, 128), (2, 64), (4, 32)]),
        }),
        envelope: Some(PtvEnvelope::new(
            Box::new([(0, 0), (96, 96)]), // Envelope points.
            100,                          // Release time (in ms by default).
        )),
        ..Default::default()
    };

    // Create a new ptvoice with these units.
    let units = Box::new([unit_1, unit_2]);
    let ptv = Ptvoice::new(units);

    // If a filename was given, write the ptvoice to the output file. Otherwise, just print it.
    if let Some(filename) = args().nth(1) {
        let mut file = File::create(filename)?;
        ptv.write_to(&mut file)?;
    } else {
        println!("{:?}", ptv);
    }

    Ok(())
}
