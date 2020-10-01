use glob::glob;
use std::fs::File;
use std::io::prelude::*;

use poe_dat_rs::types;

fn main() -> Result<(), String> {
    for path in glob("G:\\POE\\Data\\*.dat").unwrap() {
        let path = path.unwrap();
        let mut file = File::open(path.as_path()).map_err(|err| format!("{:?}", err))?;

        let mut contents = Vec::<u8>::new();
        let len = file
            .read_to_end(&mut contents)
            .map_err(|err| format!("{:?}", err))?;

        let fname = path.as_path().file_name().unwrap().to_str().unwrap();

        print!("{} ({}): ", fname, len);
        let parsed = types::PoeData::parse_file(fname, &contents[..]);

        println!("{}", parsed.is_ok());
    }

    Ok(())
}
