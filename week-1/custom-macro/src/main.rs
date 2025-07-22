use std::fmt::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: Vec<u8>) -> Result<Swap, Error>;
}

#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_le_bytes());
        v
    }
}

impl Deserialize for Swap {
    fn deserialize(v: Vec<u8>) -> Result<Swap, Error> {
        if v.len() > 8 {
            return Err(Error);
        }
        let qty_1 = u32::from_le_bytes(v[0..4].try_into().unwrap());
        let qty_2 = u32::from_le_bytes(v[4..8].try_into().unwrap());
        Ok(Swap { qty_1, qty_2 })
    }
}

fn main() {
    let u = Swap {
        qty_1: 12,
        qty_2: 43,
    };

    let serialized = u.serialize();
    println!("{:?}", serialized);

    let deserialized = Swap::deserialize(serialized).unwrap();
    println!("{:?}", deserialized);
}
