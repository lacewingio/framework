use ructe::Ructe;
use std::env;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {

    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("dist").unwrap();
    statics.add_files("assets/images").unwrap();

    ructe.compile_templates("templates").unwrap();

    cornucopia()?;

    Ok(())
}

fn cornucopia() -> Result<(), std::io::Error> {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = Path::new("db/queries").canonicalize().unwrap();

    // Again, for simplicity, we generate the module in our project, but
    // we could've also generated it elsewhere if we wanted to.
    // For example, you could make the destination the `target` folder
    // and include the generated file with a `include_str` statement in your project.

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs").canonicalize().unwrap();

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    let queries_path_str = queries_path.to_str().unwrap();
    println!("cargo:rerun-if-changed={queries_path_str}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}
