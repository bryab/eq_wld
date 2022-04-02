use std::any::Any;

use super::{Fragment, FragmentType, StringHash};

use nom::IResult;

#[derive(Debug)]
/// _Unknown_
///
/// **Type ID:** 0x16
pub struct ZoneUnknownFragment {}

impl FragmentType for ZoneUnknownFragment {
    type T = Self;

    const TYPE_ID: u32 = 0x16;

    fn parse(input: &[u8]) -> IResult<&[u8], ZoneUnknownFragment> {
        Ok((input, ZoneUnknownFragment {}))
    }
}

impl Fragment for ZoneUnknownFragment {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self, string_hash: &StringHash) -> String {
        String::new()
    }
}
