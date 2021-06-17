fn main() {
    lightsaber_buildsystem::cli::build(lightsaber_buildsystem::cli::BuildOptions {
        profile: None,
        target_architecture: None,
        target_triple: None
    });
}
