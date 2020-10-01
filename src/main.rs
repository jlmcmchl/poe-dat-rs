use glob::{PatternError, glob};
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

use poe_dat_rs::types;

fn main() -> Result<(), PatternError> {
    let files: Vec<_> = glob("G:\\POE\\Data\\*.dat")?.filter_map(|x| x.ok()).collect();
    let results: Vec<_> = files.par_iter().map(|path| {
        let mut file = File::open(path.as_path()).map_err(|err| format!("{:?}", err)).unwrap();

        let mut contents = Vec::<u8>::new();
        let len = file
            .read_to_end(&mut contents).unwrap();

        let fname = path.as_path().file_name().unwrap().to_str().unwrap();

        print!("{} ({}): ", fname, len);
        let parsed = types::PoeData::parse_file(fname, &contents[..]);

        println!("{}", parsed.is_ok());
    }).collect();

    Ok(())
}
