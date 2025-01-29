#[cfg(feature = "binary")]
use crate::{DeBin, DeBinErr, SerBin};

#[cfg(feature = "binary")]
impl<A: smallvec::Array> SerBin for smallvec::SmallVec<A>
where
    A::Item: SerBin,
{
    fn ser_bin(&self, s: &mut Vec<u8>) {
        let len = self.len();
        len.ser_bin(s);
        for item in self {
            item.ser_bin(s);
        }
    }
}

#[cfg(feature = "binary")]
impl<A: smallvec::Array> DeBin for smallvec::SmallVec<A>
where
    A::Item: DeBin,
{
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let len: usize = DeBin::de_bin(o, d)?;
        let mut out = smallvec::SmallVec::<A>::with_capacity(len);
        for _ in 0..len {
            out.push(DeBin::de_bin(o, d)?)
        }
        Ok(out)
    }
}

#[cfg(feature = "ron")]
use crate::{DeRon, DeRonErr, DeRonState, DeRonTok, SerRon, SerRonState};
#[cfg(feature = "ron")]
use core::str::Chars;

#[cfg(feature = "ron")]
impl<A: smallvec::Array> SerRon for smallvec::SmallVec<A>
where
    A::Item: SerRon,
{
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.out.push_str("[\n");
        for item in self {
            s.indent(d + 1);
            item.ser_ron(d + 1, s);
            s.conl();
        }
        s.indent(d);
        s.out.push(']');
    }
}

#[cfg(feature = "ron")]
impl<A: smallvec::Array> DeRon for smallvec::SmallVec<A>
where
    A::Item: DeRon,
{
    fn de_ron(s: &mut DeRonState, i: &mut Chars) -> Result<Self, DeRonErr> {
        let mut out = smallvec::SmallVec::<A>::new();
        s.block_open(i)?;
        while s.tok != DeRonTok::BlockClose {
            out.push(DeRon::de_ron(s, i)?);
            s.eat_comma_block(i)?;
        }
        s.block_close(i)?;
        Ok(out)
    }
}
