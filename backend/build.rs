fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .out_dir("./src")
        .type_attribute(
            "UserResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../proto/users.proto"], &["../proto"])?;
    Ok(())
}
