// https://doc.rust-lang.org/cargo/reference/build-scripts.html

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Hack to update package version... :)
  // If we set CARGO_PKG_VERSION this way, then it will override the default value, which is
  // taken from the `version` in Cargo.toml.
  if let Ok(val) = std::env::var("BUILD_VERSION") {
    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", val);
  }
  println!("cargo:rerun-if-env-changed=BUILD_VERSION");

  let p = protoc_bin_vendored::protoc_bin_path().unwrap();
  println!("compiling protos with {}", p.to_str().unwrap());
  std::env::set_var("PROTOC", p);

  // PROTO things
  let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR should be set by cargo but can't find");
  println!("{}", out_dir);
  let out_dir = std::path::PathBuf::from(out_dir).join("tonic");

  std::fs::create_dir_all(out_dir.clone()).expect("cannot create output dir");

  tonic_build::configure()
    .file_descriptor_set_path(out_dir.clone().join("echo_descriptor.bin"))
    .out_dir(out_dir.clone())
    .build_client(true) // used for tests
    .build_server(true)
    .compile(
      &["proto/echo/echo.proto"],
      &["proto/echo"],
    )?; 

  tonic_build::configure()
    .file_descriptor_set_path(out_dir.clone().join("health_descriptor.bin"))
    .out_dir(out_dir)
    .build_client(true) // used for tests
    .build_server(true)
    .compile(
      &["proto/health/v1/health.proto"],
      &["proto/health/v1"],
    )?;

  // let protos = list_protos(Path::new(PROTOS))?;
  // tonic_build::configure()
  //     .build_client(false)
  //     .compile(&protos, &[PROTOS])
  //     .context("compile protos")

  Ok(())
}

// fn list_protos(dir: &Path) -> Result<Vec<PathBuf>> {
//   WalkDir::new(dir)
//     .into_iter()
//     .try_fold(vec![], |mut protos, entry| {
//       let entry = entry.context("read proto file")?;
//       let path = entry.path();
//       if path.extension().and_then(OsStr::to_str) == Some("proto") {
//         protos.push(path.to_path_buf());
//       }
//       Ok(protos)
//     })
// }
