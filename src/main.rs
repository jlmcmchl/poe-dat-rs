use glob::{glob, PatternError};
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

use poe_dat_rs::types;

fn main() -> Result<(), PatternError> {
    let files: Vec<_> = glob("G:\\POE\\Data\\Mods.dat")?
        .filter_map(|x| x.ok())
        .collect();
    
    let now = SystemTime::now();

    let _: Vec<_> = files
        .par_iter()
        .map(|path| {
            let mut file = File::open(path.as_path())
                .map_err(|err| format!("{:?}", err))
                .unwrap();

            let mut contents = Vec::<u8>::new();
            let _ = file.read_to_end(&mut contents).unwrap();

            let fname = path.as_path().file_name().unwrap().to_str().unwrap();

            /* Languages.dat is weird - spec:
            sequence int32
            id string
            name string
            locale string
            str4 string
            val5 int32
            IsEnabled int32
            */
            if fname == "Languages.dat" {
                return;
            }

            let current = SystemTime::now();


            let (good, bad) = types::PoeData::parse_file(fname, &contents[..]).unwrap();

            match good {
                types::PoeData::Mods(v) => for mods in &v[..10] {
                    println!("{}\t{}", mods.id, mods.name);
                }
                _ => {}
            }


            match current.elapsed() {
                Ok(elapsed) => {
                    // it prints '2'
                    println!("{} dispatched: {}", fname, elapsed.as_nanos());
                }
                Err(e) => {
                    // an error occurred!
                    println!("Error: {:?}", e);
                }
            }

            println!("{:?}", bad);


            // let current = SystemTime::now();


            // poe_dat_rs::parse::parse::<types::Mods>(&contents[..]);

            // match current.elapsed() {
            //     Ok(elapsed) => {
            //         // it prints '2'
            //         println!("{} straight: {}", fname, elapsed.as_nanos());
            //     }
            //     Err(e) => {
            //         // an error occurred!
            //         println!("Error: {:?}", e);
            //     }
            // }



            //let wfname = String::from("datas/") + fname + ".json";
            //let mut wfile = File::create(wfname).unwrap();
            //wfile.write_all(serde_json::to_string_pretty(&parsed).unwrap().as_bytes()).unwrap();
        })
        .collect();

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Total: {}", elapsed.as_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
