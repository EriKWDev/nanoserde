#[cfg(feature = "binary")]
use crate::{DeBin, DeBinErr, SerBin};

#[cfg(feature = "binary")]
impl<K: intmap::IntKey, V> SerBin for intmap::IntMap<K, V>
where
    K: SerBin,
    V: SerBin,
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
impl<K: intmap::IntKey, V> DeBin for intmap::IntMap<K, V>
where
    K: DeBin,
    V: DeBin,
{
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let len: usize = DeBin::de_bin(o, d)?;
        let mut h = intmap::IntMap::with_capacity(len);
        for _ in 0..len {
            let k = DeBin::de_bin(o, d)?;
            let v = DeBin::de_bin(o, d)?;
            h.insert(k, v);
        }
        Ok(h)
    }
}

#[cfg(feature = "ron")]
use crate::{DeRon, DeRonErr, DeRonState, DeRonTok, SerRon, SerRonState};
#[cfg(feature = "ron")]
use core::str::Chars;

#[cfg(feature = "ron")]
impl<K: intmap::IntKey, V> SerRon for intmap::IntMap<K, V>
where
    K: SerRon,
    V: SerRon,
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
impl<K: intmap::IntKey, V> DeRon for intmap::IntMap<K, V>
where
    K: DeRon,
    V: DeRon,
{
    fn de_ron(s: &mut DeRonState, i: &mut Chars) -> Result<Self, DeRonErr> {
        let mut h = intmap::IntMap::new();
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
