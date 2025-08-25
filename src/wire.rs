use bytes::{Buf, BufMut};
use commonware_codec::{EncodeSize, Error, Read, ReadExt, Write};

const SIGNATURE_BYTES: usize = 32;

/// Represents a top-level message for the Aggregation protocol,
/// typically sent over a dedicated aggregation communication channel.
///
/// It encapsulates a specific round number, flexible metadata, and a payload containing the actual
/// aggregation protocol message content.
#[derive(Clone, Debug, PartialEq)]
pub struct Aggregation<T>
where
    T: Clone + Send + Sync + Write + Read<Cfg = ()> + EncodeSize,
{
    pub round: u64,
    pub metadata: T,
    pub payload: Option<aggregation::Payload>,
}

impl<T> Aggregation<T>
where
    T: Clone + Send + Sync + Write + Read<Cfg = ()> + EncodeSize,
{
    /// Create a new Aggregation message with the given metadata
    pub fn new(round: u64, metadata: T, payload: Option<aggregation::Payload>) -> Self {
        Self {
            round,
            metadata,
            payload,
        }
    }
}

impl<T> Write for Aggregation<T>
where
    T: Clone + Send + Sync + Write + Read<Cfg = ()> + EncodeSize,
{
    fn write(&self, buf: &mut impl BufMut) {
        self.round.write(buf);
        self.metadata.write(buf);
        self.payload.write(buf);
    }
}

impl<T> Read for Aggregation<T>
where
    T: Clone + Send + Sync + Write + Read<Cfg = ()> + EncodeSize,
{
    type Cfg = ();

    fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
        let round = u64::read(buf)?;
        let metadata = T::read(buf)?;
        let payload = Option::<aggregation::Payload>::read(buf)?;
        Ok(Self {
            round,
            metadata,
            payload,
        })
    }
}

impl<T> EncodeSize for Aggregation<T>
where
    T: Clone + Send + Sync + Write + Read<Cfg = ()> + EncodeSize,
{
    fn encode_size(&self) -> usize {
        self.round.encode_size() + self.metadata.encode_size() + self.payload.encode_size()
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
    use crate::usecases::counter::creators::CounterTaskData;
    use alloy::hex;

    const SAMPLE_SIGNATURE_HEX: &str =
        "4ffa4441848335dace97935d3c167d212fe5563c1ce9a626cc6d69b4fe06449c";

    #[test]
    fn test_aggregation_start_codec() {
        let metadata = CounterTaskData {
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
        };

        let original = Aggregation::new(1, metadata, Some(aggregation::Payload::Start));
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::<CounterTaskData>::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_aggregation_signature_codec() {
        let metadata = CounterTaskData {
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
        };

        let original = Aggregation::new(
            1,
            metadata,
            Some(aggregation::Payload::Signature(
                hex::decode(SAMPLE_SIGNATURE_HEX).expect("hex decode failed"),
            )),
        );
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::<CounterTaskData>::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }
}
