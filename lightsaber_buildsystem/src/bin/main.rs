use std::process;

fn main() {
    process::exit(lightsaber_buildsystem::cli::build(lightsaber_buildsystem::cli::BuildOptions {
        profile: None,
        target_architecture: None,
        target_triple: None
    }).unwrap().0);
}
