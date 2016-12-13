extern crate pkg_config;

fn main() {
    if std::process::Command::new("pkg-config").output().is_err() {
        return;
    }

    let lib = pkg_config::probe_library("gsl").unwrap();

    if lib.version.starts_with("2.") {
        println!(r#"cargo:rustc-cfg=feature="v2""#);
    }
}
