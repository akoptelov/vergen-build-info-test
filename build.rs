fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder()
        .all_git()
        .emit_and_set()?;
    Ok(())
}
