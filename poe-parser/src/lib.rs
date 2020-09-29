use lazy_static::lazy_static;
use nom::{
    bytes::complete::take_until, combinator::all_consuming, number::complete::le_u16,
    number::complete::le_u32, IResult,
};

type Parser<'a, T> = dyn Fn(&'a [u8], &'a [u8]) -> IResult<&'a [u8], T>;

pub trait Parse {
    fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized;

    fn parse_ref_string<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], String> {
        let (input, pointer) = le_u32(input)?;

        let mut mem = &variable_data[pointer as usize..];
        let mut string = Vec::new();

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

pub fn parse<T>(data: &[u8]) -> IResult<&[u8], Vec<T>>
where
    T: Parse,
{
    all_consuming(|data| {
        let (input, table_len) = le_u32(data)?;
        let (variable_data, static_data) = take_until(&MAGIC_NUMBER[..])(input)?;

        std::iter::repeat(|row| T::parse(row, variable_data))
            .take(table_len as usize)
            .try_fold((static_data, Vec::new()), |(data, mut acc), parser| {
                parser(data).map(|(i, o)| {
                    acc.push(o);
                    (i, acc)
                })
            })
    })(data)
}
