//! Example of creating a ptvoice from scratch.

use ptcrab::data::WriteTo;
use ptcrab::ptvoice::{PtvEnvelope, PtvUnit, PtvWave, Ptvoice};
use ptcrab::Key;

use anyhow::Result as AnyResult;
use std::env::args;
use std::fs::File;

//--------------------------------------------------------------------------------------------------

fn main() -> AnyResult<()> {
    // First unit: a triangular coordinate wave that fades out over 1 second.
    let unit_1 = {
        let wave_1 = PtvWave::new_coordinate(Box::new([
            // Coordinate points...
            (0, 0),
            (64, 64),
            (128, 0),
            (192, -64),
        ]));
        let envelope_1 = PtvEnvelope::new(
            Box::new([
                // Envelope points...
                (0, 96),
                (1000, 0),
            ]),
            1, // Release time (in ms by default).
        );

        PtvUnit {
            inherent_key: Key::A6,
            wave: Some(wave_1),
            envelope: Some(envelope_1),
            ..Default::default()
        }
    };

    // Second unit: an oscillator pitched 7 semitones above the first unit with slight attack.
    let unit_2 = {
        let wave_2 = PtvWave::new_oscillator(Box::new([
            // Oscillator `(harmonic_num, amplitude)` pairs...
            (1, 128),
            (2, -64),
            (4, -32),
        ]));
        let envelope_2 = PtvEnvelope::new(
            Box::new([
                // Envelope points...
                (0, 0),
                (16, 8),
                (32, 88),
                (48, 96),
            ]),
            100, // Release time (in ms by default).
        );

        PtvUnit {
            inherent_key: Key::approx_from_a6_semis(7.),
            volume: 40.into(),
            wave: Some(wave_2),
            envelope: Some(envelope_2),
            ..Default::default()
        }
    };

    // Create a new ptvoice with these units.
    let ptv = Ptvoice::new(Box::new([unit_1, unit_2]));

    // If a filename was given, write the ptvoice to the output file. Otherwise, just print it.
    if let Some(filename) = args().nth(1) {
        let mut file = File::create(filename)?;
        ptv.write_to(&mut file)?;
    } else {
        println!("{:?}", ptv);
    }

    Ok(())
}
