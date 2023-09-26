fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = tonic_build::configure().build_server(true);

    for ty in [
        "Event",
        "Benchmark",
        "BenchmarkConfiguration",
        "Indicator",
        "SecurityType",
        "SignalType",
        "ResultQ1",
        "ResultQ2",
        "Batch",
        "CrossoverEvent",
        "Query",
    ] {
        builder = builder.type_attribute(ty, "#[derive(serde::Serialize, serde::Deserialize)]");
    }

    builder.compile(&["proto/challenger.proto"], &["proto/"])?;
    Ok(())
}
