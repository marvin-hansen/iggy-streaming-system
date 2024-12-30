use crate::*;

pub use decoder::DataBarDecoder;
pub use encoder::DataBarEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 75;
pub const SBE_TEMPLATE_ID: u16 = 204;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct DataBarEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for DataBarEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for DataBarEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> DataBarEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_type(&mut self, value: message_type::MessageType) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value as u16)
        }

        /// primitive array field 'symbolID'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: String
        /// - encodedOffset: 2
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn symbol_id(&mut self, value: &[u8; 20]) {
            let offset = self.offset + 2;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'dateTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 22
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn date_time(&mut self, value: i64) {
            let offset = self.offset + 22;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn open_price_encoder(self) -> decimal_price_codec::DecimalPriceEncoder<Self> {
            let offset = self.offset + 30;
            decimal_price_codec::DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn high_price_encoder(self) -> decimal_price_codec::DecimalPriceEncoder<Self> {
            let offset = self.offset + 39;
            decimal_price_codec::DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn low_price_encoder(self) -> decimal_price_codec::DecimalPriceEncoder<Self> {
            let offset = self.offset + 48;
            decimal_price_codec::DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn close_price_encoder(self) -> decimal_price_codec::DecimalPriceEncoder<Self> {
            let offset = self.offset + 57;
            decimal_price_codec::DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn volume_encoder(self) -> decimal_volume_codec::DecimalVolumeEncoder<Self> {
            let offset = self.offset + 66;
            decimal_volume_codec::DecimalVolumeEncoder::default().wrap(self, offset)
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DataBarDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for DataBarDecoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for DataBarDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for DataBarDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> DataBarDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>, offset: usize) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                offset + message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_type(&self) -> message_type::MessageType {
            self.get_buf().get_u16_at(self.offset).into()
        }

        #[inline]
        pub fn symbol_id(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 2)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn date_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 22)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn open_price_decoder(self) -> decimal_price_codec::DecimalPriceDecoder<Self> {
            let offset = self.offset + 30;
            decimal_price_codec::DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn high_price_decoder(self) -> decimal_price_codec::DecimalPriceDecoder<Self> {
            let offset = self.offset + 39;
            decimal_price_codec::DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn low_price_decoder(self) -> decimal_price_codec::DecimalPriceDecoder<Self> {
            let offset = self.offset + 48;
            decimal_price_codec::DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn close_price_decoder(self) -> decimal_price_codec::DecimalPriceDecoder<Self> {
            let offset = self.offset + 57;
            decimal_price_codec::DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn volume_decoder(self) -> decimal_volume_codec::DecimalVolumeDecoder<Self> {
            let offset = self.offset + 66;
            decimal_volume_codec::DecimalVolumeDecoder::default().wrap(self, offset)
        }
    }
} // end decoder
