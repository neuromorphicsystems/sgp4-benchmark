mod configuration;

use std::convert::TryInto;
use std::io::Read;

fn read(path: &std::path::Path) -> std::io::Result<Vec<sgp4::Prediction>> {
    let mut file = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut predictions = Vec::new();
    let mut buffer = [0; 48];
    while file.read_exact(&mut buffer).is_ok() {
        predictions.push(sgp4::Prediction {
            position: [
                f64::from_le_bytes(buffer[0..8].try_into().unwrap()),
                f64::from_le_bytes(buffer[8..16].try_into().unwrap()),
                f64::from_le_bytes(buffer[16..24].try_into().unwrap()),
            ],
            velocity: [
                f64::from_le_bytes(buffer[24..32].try_into().unwrap()),
                f64::from_le_bytes(buffer[32..40].try_into().unwrap()),
                f64::from_le_bytes(buffer[40..48].try_into().unwrap()),
            ],
        });
    }
    Ok(predictions)
}

fn main() -> std::io::Result<()> {
    let root = configuration::root();
    let benchmarks = configuration::Configuration::parse();
    benchmarks.build()?;
    if let Some((first, others)) = benchmarks.executables.split_first() {
        println!("{}", first.id);
        println!("    running");
        first.run(root.join("reference_output.f64").to_str().unwrap())?;
        println!("    loading reference predictions");
        let reference_predictions = read(&root.join("reference_output.f64"))?;
        for other in others {
            println!("{}", other.id);
            println!("    running");
            other.run(root.join("output.f64").to_str().unwrap())?;
            println!("    loading predictions");
            let predictions = read(&root.join("output.f64"))?;
            assert_eq!(reference_predictions.len(), predictions.len());
            let (maximum_position_error, maximum_velocity_error) =
                reference_predictions.iter().zip(predictions.iter()).fold(
                    (0.0, 0.0),
                    |(maximum_position_error, maximum_velocity_error),
                     (reference_prediction, prediction)| {
                        (
                            reference_prediction
                                .position
                                .iter()
                                .zip(prediction.position.iter())
                                .map(|(reference, value)| (reference - value).abs())
                                .fold(0.0, f64::max)
                                .max(maximum_position_error),
                            reference_prediction
                                .velocity
                                .iter()
                                .zip(prediction.velocity.iter())
                                .map(|(reference, value)| (reference - value).abs())
                                .fold(0.0, f64::max)
                                .max(maximum_velocity_error),
                        )
                    },
                );
            println!("    maximum position error: {} km", maximum_position_error);
            println!(
                "    maximum velocity error: {} km.s⁻¹",
                maximum_velocity_error
            );
        }
    }
    Ok(())
}
