use lazy_static::lazy_static;
use nom::{
    bytes::complete::take_until, number::complete::le_u16, number::complete::le_u32, IResult,
};

use rayon::{iter::Either, prelude::*};

type Parser<'a, T> = dyn Fn(&'a [u8], &'a [u8]) -> IResult<&'a [u8], T>;

pub trait Parse {
    fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized;

    fn parse_ref_string<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], String> {
        let (input, pointer) = le_u32(input)?;

        let mut mem = &variable_data[pointer as usize..];
        let mut string = Vec::new();
        if mem.len() == 0 {
            return String::from_utf16(&string).map_or_else(
                |_| {
                    Err(nom::Err::Failure((
                        &variable_data[pointer as usize..],
                        nom::error::ErrorKind::Char,
                    )))
                },
                |res| Ok((input, res)),
            );
        }

        loop {
            let res = le_u16(mem)?;
            mem = res.0;

            if res.1 == 0 {
                break String::from_utf16(&string).map_or_else(
                    |_| {
                        Err(nom::Err::Failure((
                            &variable_data[pointer as usize..],
                            nom::error::ErrorKind::Char,
                        )))
                    },
                    |res| Ok((input, res)),
                );
            }

            string.push(res.1);

            if mem.len() == 0 {
                break String::from_utf16(&string).map_or_else(
                    |_| {
                        Err(nom::Err::Failure((
                            &variable_data[pointer as usize..],
                            nom::error::ErrorKind::Char,
                        )))
                    },
                    |res| Ok((input, res)),
                );
            }
        }
    }

    fn parse_vec<'a, T>(
        input: &'a [u8],
        variable_data: &'a [u8],
        parser: Box<Parser<'a, T>>,
    ) -> IResult<&'a [u8], Vec<T>> {
        let (input, mut len) = le_u32(input)?;
        let (input, offset) = le_u32(input)?;

        let mut arr = Vec::new();
        let mut view = &variable_data[offset as usize..];

        while len > 0 {
            let res = parser(view, variable_data)?;
            view = res.0;

            arr.push(res.1);

            len -= 1;
        }
        Ok((input, arr))
    }
}

lazy_static! {
    static ref MAGIC_NUMBER: Vec<u8> = vec![0xbb; 8];
}

pub fn parse<T>(data: &[u8]) -> (Vec<T>, Vec<nom::Err<(&[u8], nom::error::ErrorKind)>>)
where
    T: Parse + Send + Default + Clone,
{
    let (input, table_len) = le_u32::<nom::error::VerboseError<&[u8]>>(data).unwrap();

    if table_len == 0 {
        return (Vec::new(), Vec::new());
    }

    let (variable_data, static_data) =
        take_until::<_, _, nom::error::VerboseError<&[u8]>>(&MAGIC_NUMBER[..])(input).unwrap();

    if static_data.len() == 0 {
        return (vec![Default::default(); table_len as usize], Vec::new());
    }

    let rowlen = static_data.len() / table_len as usize;
    static_data
        .par_chunks_exact(rowlen)
        .map(|chunk| T::parse(chunk, variable_data))
        .partition_map(|row| match row {
            Ok((_, row)) => Either::Left(row),
            Err(e) => Either::Right(e),
        })
}
