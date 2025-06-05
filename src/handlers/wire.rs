use bytes::{Buf, BufMut};
use commonware_codec::{EncodeSize, Error, Read, ReadExt, Write};

/// Represents a top-level message for the Aggregation protocol,
/// typically sent over a dedicated aggregation communication channel.
///
/// It encapsulates a specific round number and a payload containing the actual
/// aggregation protocol message content.
#[derive(Clone, Debug, PartialEq)]
pub struct Aggregation {
    pub round: u64,
    pub payload: Option<aggregation::Payload>,
}

impl Write for Aggregation {
    fn write(&self, buf: &mut impl BufMut) {
        self.round.write(buf);
        self.payload.write(buf);
    }
}

impl Read for Aggregation {
    type Cfg = ();

    fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
        let round = u64::read(buf)?;
        let payload = Option::<aggregation::Payload>::read(buf)?;
        Ok(Self { round, payload })
    }
}

impl EncodeSize for Aggregation {
    fn encode_size(&self) -> usize {
        self.round.encode_size() + self.payload.encode_size()
    }
}

pub mod aggregation {

    use bytes::{Buf, BufMut};

    /// Message sent by orchestrator to start aggregation
    #[derive(Clone, Debug, PartialEq)]
    pub struct Start {}

    impl Write for Start {
        fn write(&self, _buf: &mut impl BufMut) {}
    }

    impl Read for Start {
        type Cfg = ();

        fn read_cfg(_buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
            Ok(Self {})
        }
    }

    impl EncodeSize for Start {
        fn encode_size(&self) -> usize {
            0
        }
    }
    use commonware_codec::{EncodeSize, Error, Read, ReadExt, ReadRangeExt, Write};

    /// Sent by signer to all others
    #[derive(Clone, Debug, PartialEq)]
    pub struct Signature {
        pub signature: Vec<u8>,
    }

    impl Write for Signature {
        fn write(&self, buf: &mut impl BufMut) {
            self.signature.write(buf);
        }
    }

    impl Read for Signature {
        type Cfg = ();

        fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
            let signature = Vec::<u8>::read_range(buf, ..)?;
            Ok(Self { signature })
        }
    }

    impl EncodeSize for Signature {
        fn encode_size(&self) -> usize {
            self.signature.encode_size()
        }
    }

    /// Defines the different types of messages exchanged during the aggregation protocol.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Payload {
        /// Message sent by orchestrator to start aggregation
        Start(Start),
        /// Sent by signer to all others
        Signature(Signature),
    }

    impl Write for Payload {
        fn write(&self, buf: &mut impl BufMut) {
            match self {
                Payload::Start(start) => {
                    buf.put_u8(0);
                    start.write(buf);
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
                0 => Payload::Start(Start::read(buf)?),
                1 => Payload::Signature(Signature::read(buf)?),
                _ => return Err(Error::InvalidEnum(tag)),
            };
            Ok(result)
        }
    }

    impl EncodeSize for Payload {
        fn encode_size(&self) -> usize {
            1 + match self {
                Payload::Start(start) => start.encode_size(),
                Payload::Signature(signature) => signature.encode_size(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregation_start_codec() {
        let original = Aggregation {
            round: 1,
            payload: Some(aggregation::Payload::Start(aggregation::Start {})),
        };
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_aggregation_signature_codec() {
        let original = Aggregation {
            round: 1,
            payload: Some(aggregation::Payload::Signature(aggregation::Signature {
                signature: vec![1, 2, 3],
            })),
        };
        let mut buf = Vec::with_capacity(original.encode_size());
        original.write(&mut buf);
        let decoded = Aggregation::read(&mut std::io::Cursor::new(buf)).unwrap();
        assert_eq!(original, decoded);
    }
}
