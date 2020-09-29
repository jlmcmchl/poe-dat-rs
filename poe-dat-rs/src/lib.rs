pub use poe_parser as parse;

pub mod types {
    use poe_type_gen::genstructs;
    use poe_type_gen::Parse;
    use serde::{Deserialize, Serialize};

    genstructs!(r"C:\Code\poe-dat-converter\spec.json");
}
