fn main() -> sgp4::Result<()> {
    let mut omms: Vec<sgp4::Elements> = Vec::new();
    let groups: Vec<String> = serde_json::from_str(include_str!("../groups.json"))?;
    for group in groups {
        println!("{}", group);
        let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
            .query("GROUP", &group)
            .query("FORMAT", "json")
            .call();
        if response.error() {
            return Err(sgp4::Error::new(format!(
                "network error {}: {}",
                response.status(),
                response.into_string()?
            )));
        }
        omms.append(&mut response.into_json_deserialize()?);
    }
    serde_json::to_writer(
        std::io::BufWriter::new(&std::fs::File::create(
            std::fs::canonicalize(std::path::Path::new(file!()))?
                .ancestors()
                .nth(2)
                .unwrap()
                .join("omms.json"),
        )?),
        &omms,
    )?;
    Ok(())
}
