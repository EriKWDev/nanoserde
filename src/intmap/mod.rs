#[cfg(feature = "binary")]
pub mod bin {
    use crate::{DeBin, DeBinErr, SerBin};

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

    impl<K: intmap::IntKey, V> DeBin for intmap::IntMap<K, V>
    where
        K: DeBin + core::cmp::Eq + core::hash::Hash,
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
}
