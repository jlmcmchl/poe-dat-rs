use glob::glob;
use std::fs::File;
use std::io::prelude::*;

mod parse;

fn main() {
    for path in glob("G:\\POE\\Data\\AfflictionSplitDemons.dat").unwrap() {
        let path = path.unwrap();
        let mut file = match File::open(path.as_path()) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut contents = Vec::<u8>::new();
        match file.read_to_end(&mut contents) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(_size) => {
                let parsed =
                    parse::parse::<parse::AfflictionSplitDemon>(contents.as_slice()).unwrap().1;
                println!("path: {:?}\ncontent: {:?}", path, parsed);
            }
        }
        break;
    }
}
