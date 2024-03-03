use std::env;
use ructe::Ructe;
use postgres::{Client, NoTls};
use cornucopia::{CodegenSettings};

fn main() -> Result<(), std::io::Error> {
    ructe()?;
    cornucopia()?;
    Ok(())
}

fn ructe() -> Result<(), std::io::Error> {
    let mut ructe = Ructe::from_env().unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("dist").unwrap();
    statics.add_files("assets/images").unwrap();
    ructe.compile_templates("templates").unwrap();
    Ok(())
}

fn cornucopia() -> Result<(), std::io::Error> {
    let db_url = env::var_os("DATABASE_URL").unwrap();
    let mut client = Client::connect(db_url.to_str().unwrap(), NoTls).unwrap();

    let schema_file = "db/schema.sql";
    let queries_path = "db/queries";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");

    cornucopia::generate_live(
        &mut client,
        queries_path,
        Some(destination),
        settings,
    ).unwrap();

    Ok(())
}
