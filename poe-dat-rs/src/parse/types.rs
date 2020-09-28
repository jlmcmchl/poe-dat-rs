use nom::{
    number::complete::le_f32, number::complete::le_i32, number::complete::le_u64,
    number::complete::le_u8, IResult,
};

use super::{parse_ref_string, parse_vec, Parse};

#[derive(Debug)]
pub struct AbyssObjects {
    id: String,
    unknown1: i32,
    unknown2: i32,
    unknown3: i32,
    unknown4: i32,
    metadata_file: String,
    unknown6: i32,
    keys0: Vec<u64>,
    unknown9: i32,
    unknown10: i32,
    key0: u64,
    unknown12: i32,
    unknown13: i32,
    unknown14: i32,
    unknown15: i32,
    unknown16: i32,
    unknown17: i32,
    unknown18: i32,
    unknown19: i32,
    unknown20: i32,
}

impl Parse for AbyssObjects {
    fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized,
    {
        let (input, id) = parse_ref_string(input, variable_data)?;
        let (input, unknown1) = le_i32(input)?;
        let (input, unknown2) = le_i32(input)?;
        let (input, unknown3) = le_i32(input)?;
        let (input, unknown4) = le_i32(input)?;
        let (input, metadata_file) = parse_ref_string(input, variable_data)?;
        let (input, unknown6) = le_i32(input)?;
        let (input, keys0) = parse_vec(input, variable_data, Box::new(|input, _| le_u64(input)))?;
        let (input, unknown9) = le_i32(input)?;
        let (input, unknown10) = le_i32(input)?;
        let (input, key0) = le_u64(input)?;
        let (input, unknown12) = le_i32(input)?;
        let (input, unknown13) = le_i32(input)?;
        let (input, unknown14) = le_i32(input)?;
        let (input, unknown15) = le_i32(input)?;
        let (input, unknown16) = le_i32(input)?;
        let (input, unknown17) = le_i32(input)?;
        let (input, unknown18) = le_i32(input)?;
        let (input, unknown19) = le_i32(input)?;
        let (input, unknown20) = le_i32(input)?;

        Ok((
            input,
            AbyssObjects {
                id,
                unknown1,
                unknown2,
                unknown3,
                unknown4,
                metadata_file,
                unknown6,
                keys0,
                unknown9,
                unknown10,
                key0,
                unknown12,
                unknown13,
                unknown14,
                unknown15,
                unknown16,
                unknown17,
                unknown18,
                unknown19,
                unknown20,
            },
        ))
    }
}

#[derive(Debug)]
pub struct AchievementItemRewards {
    achievement_items_key: u64,
    base_item_types_key: u64,
    message: String,
}

impl Parse for AchievementItemRewards {
    fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized,
    {
        let (input, achievement_items_key) = le_u64(input)?;
        let (input, base_item_types_key) = le_u64(input)?;
        let (input, message) = parse_ref_string(input, variable_data)?;

        Ok((
            input,
            AchievementItemRewards {
                achievement_items_key,
                base_item_types_key,
                message,
            },
        ))
    }
}

#[derive(Debug)]
pub struct AfflictionSplitDemons {
    unknown0: i32,
    monster_varieties_key: u64,
    affliction_random_mod_categories_key: u64,
}

impl Parse for AfflictionSplitDemons {
    fn parse<'a>(input: &'a [u8], _: &[u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized,
    {
        let (input, unknown0) = le_i32(input)?;
        let (input, monster_varieties_key) = le_u64(input)?;
        let (input, affliction_random_mod_categories_key) = le_u64(input)?;
        Ok((
            input,
            AfflictionSplitDemons {
                unknown0,
                monster_varieties_key,
                affliction_random_mod_categories_key,
            },
        ))
    }
}

#[derive(Debug)]
pub struct AreaInfluenceDoodads {
    stats_key: u64,
    stat_value: i32,
    unknown2: f32,
    aofiles: Vec<String>,
    unknown4: i32,
    unknown7: u8,
}

impl Parse for AreaInfluenceDoodads {
    fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> IResult<&'a [u8], Self>
    where
        Self: Sized,
    {
        let (input, stats_key) = le_u64(input)?;
        let (input, stat_value) = le_i32(input)?;
        let (input, unknown2) = le_f32(input)?;
        let (input, aofiles) = parse_vec(
            input,
            variable_data,
            Box::new(|input, variable_data| parse_ref_string(input, variable_data)),
        )?;
        let (input, unknown4) = le_i32(input)?;
        let (input, unknown7) = le_u8(input)?;

        Ok((
            input,
            AreaInfluenceDoodads {
                stats_key,
                stat_value,
                unknown2,
                aofiles,
                unknown4,
                unknown7,
            },
        ))
    }
}
