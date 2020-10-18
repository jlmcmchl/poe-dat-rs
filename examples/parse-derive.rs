use std::{fs::File, io::Read};

fn main() -> Result<(), String> {
    for path in glob::glob("G:\\POE\\Data\\AreaInfluenceDoodads.dat").unwrap() {
        let path = path.unwrap();
        let mut file = File::open(path.as_path()).map_err(|err| format!("{:?}", err))?;

        let mut contents = Vec::<u8>::new();
        let _len = file
            .read_to_end(&mut contents)
            .map_err(|err| format!("{:?}", err))?;
        let (good, bad) = poe_dat_rs::parse::parse::<poe_dat_rs::types::AreaInfluenceDoodads>(
            contents.as_slice(),
        );

        for row in good {
            println!("{:?}", row);
        }

        for row in bad {
            println!("{:?}", row);
        }
    }
    Ok(())
}
