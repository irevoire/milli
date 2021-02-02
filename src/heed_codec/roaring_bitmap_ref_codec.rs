use std::borrow::Cow;
use roaring::{RoaringBitmap, RoaringBitmapRef};

pub struct RoaringBitmapRefCodec;

impl<'t> heed::BytesDecode<'t> for RoaringBitmapRefCodec {
    type DItem = RoaringBitmapRef<'t>;

    fn bytes_decode(bytes: &'t [u8]) -> Option<Self::DItem> {
        RoaringBitmapRef::deserialize_from_slice(bytes).ok()
    }
}

impl heed::BytesEncode<'_> for RoaringBitmapRefCodec {
    type EItem = RoaringBitmap;

    fn bytes_encode(item: &Self::EItem) -> Option<Cow<[u8]>> {
        let mut bytes = Vec::with_capacity(item.serialized_size());
        item.serialize_into(&mut bytes).ok()?;
        Some(Cow::Owned(bytes))
    }
}
