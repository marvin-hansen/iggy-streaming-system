use crate::*;

pub use decoder::DecimalQtyDecoder;
pub use encoder::DecimalQtyEncoder;

pub const ENCODED_LENGTH: usize = 9;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DecimalQtyEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for DecimalQtyEncoder<P>
    where
        P: Writer<'a> + Default,
    {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            match self.parent.as_mut() {
                Some(parent) => parent.get_buf_mut(),
                _ => {
                    panic!("parent was None")
                }
            }
        }
    }

    impl<'a, P> DecimalQtyEncoder<P>
    where
        P: Writer<'a> + Default,
    {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'num'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn num(&mut self, value: i64) {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'scale'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn scale(&mut self, value: u8) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value);
        }
    }
} // end encoder mod

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DecimalQtyDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for DecimalQtyDecoder<P>
    where
        P: Reader<'a> + ActingVersion + Default,
    {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for DecimalQtyDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> DecimalQtyDecoder<P>
    where
        P: Reader<'a> + Default,
    {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn num(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn scale(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 8)
        }
    }
} // end decoder mod
