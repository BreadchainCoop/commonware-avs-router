use bytes::{Buf, BufMut};
use commonware_codec::{EncodeSize, Error, Read, ReadExt, Write};

/// Represents a top-level message for the Aggregation protocol,
/// typically sent over a dedicated aggregation communication channel.
///
/// It encapsulates a specific round number, task variables, and a payload containing the actual
/// aggregation protocol message content.
#[derive(Clone, Debug, PartialEq)]
pub struct Aggregation {
    pub round: u64,
    pub var1: String,
    pub var2: String,
    pub var3: String,
    pub payload: Option<aggregation::Payload>,
}

impl Write for Aggregation {
    fn write(&self, buf: &mut impl BufMut) {
        self.round.write(buf);
        (self.var1.len() as u32).write(buf);
        buf.put_slice(self.var1.as_bytes());
        (self.var2.len() as u32).write(buf);
        buf.put_slice(self.var2.as_bytes());
        (self.var3.len() as u32).write(buf);
        buf.put_slice(self.var3.as_bytes());
        self.payload.write(buf);
    }
}

impl Read for Aggregation {
    type Cfg = ();

    fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
        let round = u64::read(buf)?;
        
        let var1_len = u32::read(buf)? as usize;
        if buf.remaining() < var1_len {
            return Err(Error::EndOfBuffer);
        }
        let mut var1_bytes = vec![0u8; var1_len];
        buf.copy_to_slice(&mut var1_bytes);
        let var1 = String::from_utf8(var1_bytes).map_err(|_| Error::Invalid("var1", "decoding from utf8 failed"))?;

        let var2_len = u32::read(buf)? as usize;
        if buf.remaining() < var2_len {
            return Err(Error::EndOfBuffer);
        }
        let mut var2_bytes = vec![0u8; var2_len];
        buf.copy_to_slice(&mut var2_bytes);
        let var2 = String::from_utf8(var2_bytes).map_err(|_| Error::Invalid("var2", "decoding from utf8 failed"))?;

        let var3_len = u32::read(buf)? as usize;
        if buf.remaining() < var3_len {
            return Err(Error::EndOfBuffer);
        }
        let mut var3_bytes = vec![0u8; var3_len];
        buf.copy_to_slice(&mut var3_bytes);
        let var3 = String::from_utf8(var3_bytes).map_err(|_| Error::Invalid("var3", "decoding from utf8 failed"))?;
        
        let payload = Option::<aggregation::Payload>::read(buf)?;
        Ok(Self { round, var1, var2, var3, payload })
    }
}

impl EncodeSize for Aggregation {
    fn encode_size(&self) -> usize {
        self.round.encode_size() + 
        4 + self.var1.len() + 
        4 + self.var2.len() + 
        4 + self.var3.len() + 
        self.payload.encode_size()
    }
}

pub mod aggregation {

    use bytes::{Buf, BufMut};
    use commonware_codec::{EncodeSize, Error, Read, ReadExt, Write};

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

    /// Sent by signer to all others
    #[derive(Clone, Debug, PartialEq)]
    pub struct Signature {
        pub signature: Vec<u8>,
    }

    impl Write for Signature {
        fn write(&self, buf: &mut impl BufMut) {
            (self.signature.len() as u32).write(buf);
            buf.put_slice(&self.signature);
        }
    }

    impl Read for Signature {
        type Cfg = ();

        fn read_cfg(buf: &mut impl Buf, _: &()) -> Result<Self, Error> {
            let len = u32::read(buf)? as usize;
            if buf.remaining() < len {
                return Err(Error::EndOfBuffer);
            }
            let mut signature = vec![0u8; len];
            buf.copy_to_slice(&mut signature);
            Ok(Self { signature })
        }
    }

    impl EncodeSize for Signature {
        fn encode_size(&self) -> usize {
            4 + self.signature.len() // u32 for length + bytes
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
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
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
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
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