use crate::*;

pub use decoder::OrderCreateDecoder;
pub use encoder::OrderCreateEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 67;
pub const SBE_TEMPLATE_ID: u16 = 401;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct OrderCreateEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderCreateEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderCreateEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCreateEncoder<'a> {
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

        /// primitive field 'exchangeID'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn exchange_id(&mut self, value: u8) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'clientID'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 3
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn client_id(&mut self, value: u16) {
            let offset = self.offset + 3;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive field 'clientOrderID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 5
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn client_order_id(&mut self, value: u64) {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn exchange_symbol_id_encoder(
            self,
        ) -> order_exchange_symbol_id_codec::OrderExchangeSymbolIDEncoder<Self> {
            let offset = self.offset + 13;
            order_exchange_symbol_id_codec::OrderExchangeSymbolIDEncoder::default()
                .wrap(self, offset)
        }

        /// primitive field 'orderType'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn order_type(&mut self, value: u8) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'orderSide'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 30
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn order_side(&mut self, value: u8) {
            let offset = self.offset + 30;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'orderTimeInForce'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 31
        /// - encodedLength: 1
        /// - version: 0
        #[inline]
        pub fn order_time_in_force(&mut self, value: u8) {
            let offset = self.offset + 31;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'orderTimeExpiry'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 32
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_time_expiry(&mut self, value: u64) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_qty_encoder(self) -> decimal_qty_codec::DecimalQtyEncoder<Self> {
            let offset = self.offset + 40;
            decimal_qty_codec::DecimalQtyEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_price_encoder(self) -> decimal_price_codec::DecimalPriceEncoder<Self> {
            let offset = self.offset + 49;
            decimal_price_codec::DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_stop_price_encoder(
            self,
        ) -> decimal_optional_codec::DecimalOptionalEncoder<Self> {
            let offset = self.offset + 58;
            decimal_optional_codec::DecimalOptionalEncoder::default().wrap(self, offset)
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderCreateDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for OrderCreateDecoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for OrderCreateDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderCreateDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCreateDecoder<'a> {
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn exchange_id(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 2)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn client_id(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 3)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn client_order_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 5)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn exchange_symbol_id_decoder(
            self,
        ) -> order_exchange_symbol_id_codec::OrderExchangeSymbolIDDecoder<Self> {
            let offset = self.offset + 13;
            order_exchange_symbol_id_codec::OrderExchangeSymbolIDDecoder::default()
                .wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_type(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 29)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_side(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 30)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_time_in_force(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 31)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-1' }
        #[inline]
        pub fn order_time_expiry(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 32);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_qty_decoder(self) -> decimal_qty_codec::DecimalQtyDecoder<Self> {
            let offset = self.offset + 40;
            decimal_qty_codec::DecimalQtyDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_price_decoder(self) -> decimal_price_codec::DecimalPriceDecoder<Self> {
            let offset = self.offset + 49;
            decimal_price_codec::DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_stop_price_decoder(
            self,
        ) -> decimal_optional_codec::DecimalOptionalDecoder<Self> {
            let offset = self.offset + 58;
            decimal_optional_codec::DecimalOptionalDecoder::default().wrap(self, offset)
        }
    }
} // end decoder
