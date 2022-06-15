use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn main() {
    generate23();
    generate24();
}

fn generate23() {
    let path = PathBuf::from("generated23/src");
    let input = fs::read_to_string("test.svd").unwrap();
    let mut config = svd2rust23::Config::default();
    config.target = svd2rust23::Target::XtensaLX;
    config.output_dir = path.clone();
    let device = svd2rust23::load_from(&input, &config).unwrap();
    let mut device_x = String::new();
    let items = svd2rust23::generate::device::render(&device, &config, &mut device_x).unwrap();
    let data = items.to_string();
    let mut file = File::create(path.join("lib.rs")).unwrap();
    file.write_all(data.as_ref()).unwrap();
}

fn generate24() {
    let path = PathBuf::from("generated24/src");
    let input = fs::read_to_string("test.svd").unwrap();
    let mut config = svd2rust24::Config::default();
    config.target = svd2rust24::Target::XtensaLX;
    config.output_dir = path.clone();
    let device = svd2rust24::load_from(&input, &config).unwrap();
    let mut device_x = String::new();
    let items = svd2rust24::generate::device::render(&device, &config, &mut device_x).unwrap();
    let data = items.to_string();
    let mut file = File::create(path.join("lib.rs")).unwrap();
    file.write_all(data.as_ref()).unwrap();
}
