#![feature(plugin)]

extern crate erased_serde;
extern crate serde;
extern crate bincode;
#[macro_use]
extern crate serde_derive;

pub trait Vaapad {
    fn get_type_id() -> u64;
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;
    use erased_serde;
    use erased_serde::{Serialize, Serializer, Deserializer};
    use bincode;
    use std::io;

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct A {
        v :u32,
        w: u32
    }
    #[derive(Serialize, Deserialize)]
    struct B;

    #[test]
    pub fn test_type_erase_serde() {
        let mut m:HashMap<&str, Box<Serialize>> = HashMap::new();
        m.insert("A", Box::new(A {v: 5, w: 6}));
        m.insert("B", Box::new(B));

        let mut buffer = Vec::<u8>::new();
        {
            let serializer = &mut bincode::Serializer::new(&mut buffer);
            let format = &mut Serializer::erase(serializer);
            let value = m.get("A").unwrap();
            let res = value.erased_serialize(format).unwrap();
        }
        assert_eq!(buffer, vec![5, 0, 0, 0, 6, 0, 0, 0]);
        {
            let reader = bincode::read_types::SliceReader::new(buffer.as_slice());
            let bin =
                &mut bincode::Deserializer::new(
                    reader, bincode::Infinite
                );
            let mut format = &mut Deserializer::erase(bin);
            let data: A = erased_serde::deserialize(format).unwrap();
            assert_eq!(data, A {v: 5, w: 6});
        }
    }
}