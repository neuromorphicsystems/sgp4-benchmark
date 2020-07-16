mod configuration;

use rand::seq::SliceRandom;

const SAMPLES: usize = 2;

fn main() -> std::io::Result<()> {
    let root = configuration::root();
    let benchmarks = configuration::Configuration::parse();
    benchmarks.build()?;
    let mut tasks = Vec::with_capacity(SAMPLES * benchmarks.executables.len());
    let mut results = std::collections::HashMap::new();
    for executable in &benchmarks.executables {
        for _ in 0..SAMPLES {
            tasks.push(executable);
        }
        results.insert(&executable.id, Vec::with_capacity(SAMPLES));
    }
    {
        let mut rng = rand::thread_rng();
        tasks.shuffle(&mut rng);
    }
    for (index, task) in tasks.iter().enumerate() {
        println!("{} / {} {}", index + 1, tasks.len(), task.id);
        results
            .get_mut(&task.id)
            .unwrap()
            .push(task.run(root.join("output.f64").to_str().unwrap())?);
    }
    serde_json::to_writer(
        std::io::BufWriter::new(&std::fs::File::create(std::path::Path::new(
            &root.join("results.json"),
        ))?),
        &results,
    )?;
    Ok(())
}
