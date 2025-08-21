use bytes::{Buf, BufMut};
use commonware_codec::{EncodeSize, Error, Read, ReadExt, Write};
use std::collections::HashMap;

const SIGNATURE_BYTES: usize = 32;

/// Represents a top-level message for the Aggregation protocol,
/// typically sent over a dedicated aggregation communication channel.
///
/// It encapsulates a specific round number, flexible metadata, and a payload containing the actual
/// aggregation protocol message content.
#[derive(Clone, Debug, PartialEq)]
pub struct Aggregation {
    pub round: u64,
    pub metadata: HashMap<String, String>,
    pub payload: Option<aggregation::Payload>,
}

impl Write for Aggregation {
    fn write(&self, buf: &mut impl BufMut) {
        self.round.write(buf);

        // Write metadata as length-prefixed key-value pairs
        (self.metadata.len() as u32).write(buf);
        for (key, value) in &self.metadata {
            (key.len() as u32).write(buf);
            buf.put_slice(key.as_bytes());
            (value.len() as u32).write(buf);
            buf.put_slice(value.as_bytes());
        }

        self.payload.write(buf);
    }
}

impl Read for Aggregation {
    type Cfg = ();

    fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
        let round = u64::read(buf)?;

        // Read metadata as length-prefixed key-value pairs
        let metadata_count = u32::read(buf)? as usize;
        let mut metadata = HashMap::new();

        for _ in 0..metadata_count {
            // Read key
            let key_len = u32::read(buf)? as usize;
            if buf.remaining() < key_len {
                return Err(Error::EndOfBuffer);
            }
            let mut key_bytes = vec![0u8; key_len];
            buf.copy_to_slice(&mut key_bytes);
            let key = String::from_utf8(key_bytes)
                .map_err(|_| Error::Invalid("metadata_key", "decoding from utf8 failed"))?;

            // Read value
            let value_len = u32::read(buf)? as usize;
            if buf.remaining() < value_len {
                return Err(Error::EndOfBuffer);
            }
            let mut value_bytes = vec![0u8; value_len];
            buf.copy_to_slice(&mut value_bytes);
            let value = String::from_utf8(value_bytes)
                .map_err(|_| Error::Invalid("metadata_value", "decoding from utf8 failed"))?;

            metadata.insert(key, value);
        }

        let payload = Option::<aggregation::Payload>::read(buf)?;
        Ok(Self {
            round,
            metadata,
            payload,
        })
    }
}

impl EncodeSize for Aggregation {
    fn encode_size(&self) -> usize {
        let mut size = self.round.encode_size() + 4; // round + metadata_count

        // Add size for each key-value pair (key_len + key + value_len + value)
        for (key, value) in &self.metadata {
            size += 4 + key.len() + 4 + value.len();
        }

        size + self.payload.encode_size()
    }
}

pub mod aggregation {

    use bytes::{Buf, BufMut};
    use commonware_codec::{EncodeSize, Error, Read, ReadExt, ReadRangeExt, Write};

    use super::SIGNATURE_BYTES;

    /// Defines the different types of messages exchanged during the aggregation protocol.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Payload {
        /// Message sent by orchestrator to start aggregation
        Start,
        /// Sent by signer to all others
        Signature(Vec<u8>),
    }

    impl Write for Payload {
        fn write(&self, buf: &mut impl BufMut) {
            match self {
                Payload::Start => {
                    buf.put_u8(0);
                }
                Payload::Signature(signature) => {
                    buf.put_u8(1);
                    signature.write(buf);
                }
            }
        }
    }

    impl Read for Payload {
        type Cfg = ();

        fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
            let tag = u8::read(buf)?;
            let result = match tag {
                0 => Payload::Start,
                1 => Payload::Signature(Vec::<u8>::read_range(buf, 1..(SIGNATURE_BYTES + 1))?),
                _ => return Err(Error::InvalidEnum(tag)),
            };
            Ok(result)
        }
    }

    impl EncodeSize for Payload {
        fn encode_size(&self) -> usize {
            1 + match self {
                Payload::Start => 0,
                Payload::Signature(signature) => signature.encode_size(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::hex;

    const SAMPLE_SIGNATURE_HEX: &str =
        "4ffa4441848335dace97935d3c167d212fe5563c1ce9a626cc6d69b4fe06449c";

    #[test]
    fn test_aggregation_start_codec() {
        let mut metadata = HashMap::new();
        metadata.insert("var1".to_string(), "test1".to_string());
        metadata.insert("var2".to_string(), "test2".to_string());
        metadata.insert("var3".to_string(), "test3".to_string());

        let original = Aggregation {
            round: 1,
            metadata,
            payload: Some(aggregation::Payload::Start),
        };
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_aggregation_signature_codec() {
        let mut metadata = HashMap::new();
        metadata.insert("var1".to_string(), "test1".to_string());
        metadata.insert("var2".to_string(), "test2".to_string());
        metadata.insert("var3".to_string(), "test3".to_string());

        let original = Aggregation {
            round: 1,
            metadata,
            payload: Some(aggregation::Payload::Signature(
                hex::decode(SAMPLE_SIGNATURE_HEX).expect("hex decode failed"),
            )),
        };
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }
}
