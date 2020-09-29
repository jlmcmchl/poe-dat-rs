use glob::glob;
use std::fs::File;
use std::io::prelude::*;

use poe_dat_rs::{parse, types};

fn main() -> Result<(), String> {
    for path in glob("G:\\POE\\Data\\AbyssObjects.dat").unwrap() {
        let path = path.unwrap();
        let mut file = File::open(path.as_path()).map_err(|err| format!("{:?}", err))?;

        let mut contents = Vec::<u8>::new();
        let len = file
            .read_to_end(&mut contents)
            .map_err(|err| format!("{:?}", err))?;

        println!("len: {}", len);

        let (_, parsed) = parse::parse::<types::AbyssObjects>(contents.as_slice())
            .map_err(|err| format!("{:?}", err))?;

        for row in parsed {
            println!("{:?}", row);
        }
    }

    Ok(())
}
