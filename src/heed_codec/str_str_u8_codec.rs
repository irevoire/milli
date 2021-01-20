use std::borrow::Cow;
use std::{str, marker};

pub struct StrStrU8Codec<'a>(marker::PhantomData<&'a ()>);

impl<'a> heed::BytesDecode<'a> for StrStrU8Codec<'_> {
    type DItem = (&'a str, &'a str, u8);

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        let (n, bytes) = bytes.split_last()?;
        let s1_end = bytes.iter().position(|b| *b == 0)?;
        let (s1_bytes, s2_bytes) = bytes.split_at(s1_end);
        let s1 = str::from_utf8(s1_bytes).ok()?;
        let s2 = str::from_utf8(&s2_bytes[1..]).ok()?;
        Some((s1, s2, *n))
    }
}

impl<'a> heed::BytesEncode for StrStrU8Codec<'a> {
    type EItem = (&'a str, &'a str, u8);

    fn bytes_encode((s1, s2, n): &Self::EItem) -> Option<Cow<[u8]>> {
        let mut bytes = Vec::with_capacity(s1.len() + s2.len() + 1 + 1);
        bytes.extend_from_slice(s1.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(s2.as_bytes());
        bytes.push(*n);
        Some(Cow::Owned(bytes))
    }
}
