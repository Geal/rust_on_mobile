extern crate cheddar;

fn main() {
  cheddar::Cheddar::new().expect("could not read manifest")
  .insert_code("#include \"types.h\"")
  .run_build("include/inrustwetrust.h");
}
