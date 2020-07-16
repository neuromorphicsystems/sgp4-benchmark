use std::io::Write;

fn main() -> sgp4::Result<()> {
    assert_eq!(std::env::args().len(), 3);
    let omms: Vec<sgp4::Elements> = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(std::env::args().nth(1).unwrap())?,
    ))?;
    let mut predictions = Vec::with_capacity(omms.len() * 1440);
    let start = std::time::Instant::now();
    for elements in &omms {
        let constants = sgp4::Constants::from_elements(elements)?;
        let mut state = constants.initial_state();
        for t in 0..1440 {
            predictions.push(constants.propagate_from_state(
                t as f64,
                state.as_mut(),
                false,
            )?);
        }
    }
    let duration = start.elapsed();
    println!("{}", duration.as_micros());
    let mut output =
        std::io::BufWriter::new(std::fs::File::create(std::env::args().nth(2).unwrap())?);
    for prediction in predictions {
        let mut buffer = [0; 48];
        buffer[0..8].copy_from_slice(&f64::to_le_bytes(prediction.position[0]));
        buffer[8..16].copy_from_slice(&f64::to_le_bytes(prediction.position[1]));
        buffer[16..24].copy_from_slice(&f64::to_le_bytes(prediction.position[2]));
        buffer[24..32].copy_from_slice(&f64::to_le_bytes(prediction.velocity[0]));
        buffer[32..40].copy_from_slice(&f64::to_le_bytes(prediction.velocity[1]));
        buffer[40..48].copy_from_slice(&f64::to_le_bytes(prediction.velocity[2]));
        output.write(&buffer)?;
    }
    Ok(())
}
