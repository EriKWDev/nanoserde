#[cfg(feature = "binary")]
use crate::{DeBin, DeBinErr, SerBin};

#[cfg(feature = "binary")]
impl<K, V, S> SerBin for indexmap::IndexMap<K, V, S>
where
    K: SerBin,
    V: SerBin,
    S: std::hash::BuildHasher,
{
    fn ser_bin(&self, s: &mut Vec<u8>) {
        let len = self.len();
        len.ser_bin(s);
        for (k, v) in self.iter() {
            k.ser_bin(s);
            v.ser_bin(s);
        }
    }
}

#[cfg(feature = "binary")]
impl<K, V, S> DeBin for indexmap::IndexMap<K, V, S>
where
    K: DeBin + std::hash::Hash + Eq,
    V: DeBin,
    S: std::hash::BuildHasher + Default,
{
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let len: usize = DeBin::de_bin(o, d)?;
        let mut h = indexmap::IndexMap::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            let k = DeBin::de_bin(o, d)?;
            let v = DeBin::de_bin(o, d)?;
            h.insert(k, v);
        }
        Ok(h)
    }
}

#[cfg(feature = "binary")]
impl<K, S> SerBin for indexmap::IndexSet<K, S>
where
    K: SerBin,
    S: std::hash::BuildHasher + Default,
{
    fn ser_bin(&self, s: &mut Vec<u8>) {
        let len = self.len();
        len.ser_bin(s);
        for item in self.iter() {
            item.ser_bin(s);
        }
    }
}

#[cfg(feature = "binary")]
impl<K, S> DeBin for indexmap::IndexSet<K, S>
where
    K: DeBin + std::hash::Hash + Eq,
    S: std::hash::BuildHasher + Default,
{
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let len: usize = DeBin::de_bin(o, d)?;
        let mut out = indexmap::IndexSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            out.insert(DeBin::de_bin(o, d)?);
        }
        Ok(out)
    }
}

#[cfg(feature = "ron")]
use crate::{DeRon, DeRonErr, DeRonState, DeRonTok, SerRon, SerRonState};
#[cfg(feature = "ron")]
use core::str::Chars;

#[cfg(feature = "ron")]
impl<K, V, S> SerRon for indexmap::IndexMap<K, V, S>
where
    K: SerRon,
    V: SerRon,
    S: std::hash::BuildHasher,
{
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.out.push_str("{\n");
        for (k, v) in self.iter() {
            s.indent(d + 1);
            k.ser_ron(d + 1, s);
            s.out.push(':');
            v.ser_ron(d + 1, s);
            s.conl();
        }
        s.indent(d);
        s.out.push('}');
    }
}

#[cfg(feature = "ron")]
impl<K, V, S> DeRon for indexmap::IndexMap<K, V, S>
where
    K: DeRon + std::hash::Hash + Eq,
    V: DeRon,
    S: std::hash::BuildHasher + Default,
{
    fn de_ron(s: &mut DeRonState, i: &mut Chars) -> Result<Self, DeRonErr> {
        let mut h = indexmap::IndexMap::with_hasher(S::default());
        s.curly_open(i)?;
        while s.tok != DeRonTok::CurlyClose {
            let k = DeRon::de_ron(s, i)?;
            s.colon(i)?;
            let v = DeRon::de_ron(s, i)?;
            s.eat_comma_curly(i)?;
            h.insert(k, v);
        }
        s.curly_close(i)?;
        Ok(h)
    }
}

#[cfg(feature = "ron")]
impl<K, S> SerRon for indexmap::IndexSet<K, S>
where
    K: SerRon,
    S: std::hash::BuildHasher,
{
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.out.push_str("[\n");
        if !self.is_empty() {
            let last = self.len() - 1;
            for (index, item) in self.iter().enumerate() {
                s.indent(d + 1);
                item.ser_ron(d + 1, s);
                s.conl();
            }
        }
        s.indent(d);
        s.out.push(']');
    }
}

#[cfg(feature = "ron")]
impl<K, S> DeRon for indexmap::IndexSet<K, S>
where
    K: DeRon + std::hash::Hash + Eq,
    S: std::hash::BuildHasher + Default,
{
    fn de_ron(s: &mut DeRonState, i: &mut Chars) -> Result<Self, DeRonErr> {
        let mut out = indexmap::IndexSet::with_hasher(S::default());
        s.block_open(i)?;
        while s.tok != DeRonTok::BlockClose {
            out.insert(DeRon::de_ron(s, i)?);
            s.eat_comma_block(i)?;
        }
        s.block_close(i)?;
        Ok(out)
    }
}
