use lazy_static::lazy_static;
use nom::{
    bytes::complete::take_until, number::complete::le_i32, number::complete::le_u32,
    number::complete::le_u64, IResult,
};

pub trait Parsable {
    fn parse<'a>(input: &'a [u8], variable_data: &[u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct AfflictionSplitDemon {
    unknown0: i32,
    monster_varieties_key: u64,
    affliction_random_mod_categories_key: u64,
}

impl Parsable for AfflictionSplitDemon {
    fn parse<'a>(input: &'a [u8], _: &[u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized,
    {
        let (input, unknown0) = le_i32(input)?;
        let (input, monster_varieties_key) = le_u64(input)?;
        let (input, affliction_random_mod_categories_key) = le_u64(input)?;
        Ok((
            input,
            AfflictionSplitDemon {
                unknown0,
                monster_varieties_key,
                affliction_random_mod_categories_key,
            },
        ))
    }
}

lazy_static! {
    static ref MAGIC_NUMBER: Vec<u8> = vec![0xbb; 8];
}

pub fn parse<T>(data: &[u8]) -> IResult<&[u8], Vec<T>>
where
    T: Parsable,
{
    let (input, table_len) = le_u32(data)?;
    let (variable_data, static_data) = take_until(&b"\xbb\xbb\xbb\xbb\xbb\xbb\xbb\xbb"[..])(input)?;

    std::iter::repeat(|row| T::parse(row, variable_data))
        .take(table_len as usize)
        .try_fold((static_data, Vec::new()), |(data, mut acc), parser| {
            parser(data).map(|(i, o)| {
                acc.push(o);
                (i, acc)
            })
        })
}
