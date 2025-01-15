# Working with SBE

* [Docs](https://github.com/real-logic/simple-binary-encoding/wiki)
* [Repo](https://github.com/real-logic/simple-binary-encoding/wiki)
* [Design](https://github.com/real-logic/simple-binary-encoding/wiki/Design-Principles)

## General workflow

1) Write an XML schema
2) Run the SBE tool to translate the schema into language bindings i.e. C++/Rust/Java
3) Implement the de/serialization for each target type

### Write an XML schema

* XML reference: https://github.com/real-logic/simple-binary-encoding/wiki/FIX-SBE-XML-Primer
* Official sample: https://github.com/real-logic/simple-binary-encoding/tree/master/sbe-samples/src/main/resources

The XML schema is straightforward and first principled i.e. complex types compose of primitive types.
The SBE

### Run the SBE tool

The SBE tool itself us equally straightforward. However, for non-Java projects, the tool needs
to be build from source. To do so, ensure a recent Java version is installed, and follow these steps:

1) Clone the SBE repo:

`git clone https://github.com/real-logic/simple-binary-encoding.git`

2) Build SBE distribution:

`./gradlew`

3) Grab the SBE-all super-jar

The jar file is usually located in:

`sbe-all/build/libs`

Once the SBE tool has been build, copy the JAR into a project folder. While it
is rarely advisable to commit binaries into git, the SBE tool is one of those exceptions
that warrant being part of the git source tree to ensure the tool is always in the right spot.
In this project, the SBE tool is stored in:

[tools/sbe](../tools/sbe)

Working directly with the JAR is not particularly ergonomic as it requires quite a few default parameters.
The important ones are:

* Set jdk.internal.misc to ALL-UNNAMED
* Set the target language binding
* Set the target namespace. Note, this must match the namespace in the XML schema
* Set the output directory
* Set the error log to yes. There is no practical way to debug the schema without this flag.
* Set the path to the SBE tool
* Set the path to the xml schema

A sample call of the SBE tool looks like:

```bash
java
--add-opens java.base/jdk.internal.misc=ALL-UNNAMED
-Dsbe.xinclude.aware=true
-Dsbe.generate.ir=true
-Dsbe.target.language=Rust
-Dsbe.target.namespace=sbe
-Dsbe.output.dir=queng_sbe/
-Dsbe.errorLog=yes
-jar tools/sbe/sbe-all-1.35.0.jar
queng_sbe/sbe_schema/schema.xml
```

For all practical purpose, wrap the the SBE command into a shell script and just call it via
a makefile. For a sample script, see

[scripts/sbe.sh](../scripts/sbe.sh)

Notice, all paths in the script are relative to the project root folder because, when the make command
invokes the underlying script, it does so from the current working directory, which for the most part is the project
root.

Also, notice that the script first generates new bindings, then deletes all previous bindings, and lastly
renames the newly generated bindings to match the crate name of the previous bindings crate. This by purpose because
if the SBE tool encounters an error, the pipefail flag ensure that the script aborts immediately
with the implication that the previous bindings will not be deleted as to ensure the project continuous to build.

### Implement the de/serialization for each target type

Implementing the encoder / decoder follows an equally straightforward and first principle approach as seen before. It is
generally advised to implement the codec with functions rather than methods and then to bolt-on the codec with type
extensions to the actual Rust type. The wisdom here is twofold; for once, the underlying type will be used in multiple
crates and only few of them need the SBE codex. By using a type extension, you only introduce an additional dependency
to the SBE codec when absolutely needed thus keeping compile time in check. Second, decoupling the optional codec from
the actual Rust type simplifies code organization quite a bit since all SBE codecs can be stored in one place thus
easing code maintenance.

While the SBE format is zero copy and zero allocation, it achieves all this with a fair bit of SBE specific ceremony
code as we will see.

**SBE encoder**

The first step is always allocating a fixed sized buffer. The crux here is, while one could calculate the precise buffer
size from the fields,
this becomes cumbersome with variable byte sized types such as char or string.
Instead, I usually start with fixed buffer of sized 100, implement the encoder, and then check the actual buffer size
during the unit tests to find the exact number and then update the precise buffer size. It is just shortcut that makes
life easier. 100 is a good starting point but if you frequently deal with large messages, set it to 1024 to ensure you
have plenty of headroom.

Next, create a type specific encoder from the language bindings, and wrap the message buffer, header_codec, and get the
parent of the wrapped header.

```rust
pub fn encode_order_create_message(msg: OrderCreate) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 75];

    // create new type specific encoder 
    let mut csg = OrderCreateEncoder::default();

    // Wrap the header 
    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );
    // get the parent of the header.  
    csg = csg.header(0).parent().expect("Failed to encode header");
...
} 
```   

At this point, the bulk of ceremony has been completed and its time to encoded each field of the Rust type.

In the example, the actual Rust type is defined as:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderCreate {
    exchange_id: ExchangeID,
    client_id: u16,
    client_order_id: ClientOrderID,
    symbol_id_exchange: OrderExchangeSymbol,
    order_type: OrderType,
    order_side: OrderSide,
    order_time_in_force: TimeInForce,
    // Expiration time. Conditionally required for orders with time_in_force = `GOOD_TILL_TIME_EXCHANGE`.
    time_expiry: Option<u64>,
    price: Decimal,
    quantity: Decimal,
}
```  

In general, simple message types without nested complex types are generally preferred as it keeps message size small.
The corresponding SBE XML schema definition is:

```xml 
    <!--   ORDER MESSAGE DEFINITIONS -->
    <sbe:message name="OrderCreate" id="401">
        <field name="messageType" id="1" type="MessageType"/>
        <field name="exchangeID" id="2" type="ExchangeID"/>
        <field name="clientID" id="3" type="ClientID"/>
        <field name="clientOrderID" id="4" type="ClientOrderID"/>
        <field name="exchangeSymbolID" id="5" type="OrderExchangeSymbolID"/>
        <field name="orderType" id="6" type="OrderType"/>
        <field name="orderSide" id="7" type="OrderSide"/>
        <field name="orderTimeInForce" id="8" type="OrderTimeInForce"/>
        <field name="orderTimeExpiry" id="9" type="OptionalTimestamp"/>
        <field name="orderQty" id="10" type="DecimalQty"/>
        <field name="orderPrice" id="11" type="DecimalPrice"/>
        <field name="orderStopPrice" id="12" type="DecimalOptional"/>
    </sbe:message>
```  

The only differences to the Rust type are:

1) An extra field messageType required to dispatch messages
2) OrderTimeInForce, OrderType, ClientOrderID, and similar are actual aliases to primitive types i.e. u8
3) Rust decimal is mapped to a custom Decimal type i.e. DecimalQty

The message type field is always required to correctly identify an incoming
message and dispatch it to the correct event handler. Because the SBE stanfard header is of constant size, you can
actually extract the message type id straight out of the byte array because the type ID is always at array index 2.

That means, in your event handler, you can immediately determine the message type without deciding the actual message
and then just pattern match and lazy decode the message when needed. For example:

```rust
     let message_type = MessageType::from(u16::from(raw_message[2]));
     
       match message_type {
            MessageType::ClientLogin => {
            ...   
```  

Consequently, that also means when encoding the message, the fist field that must be set is always the message type:

```rust
 csg.message_type(MessageType::OrderCreate);

    csg.exchange_id(msg.exchange_id() as u8);

    csg.client_id(msg.client_id());

    csg.client_order_id(msg.client_order_id().client_order_id_binary());
```  

The bulk of primitive types are trivial to encoded. Notice, though, exchange_id
is actually a Rust enum. See the section below about Rust specific best practices for details why Rust enums can safely
be encoded and decoded as U8.

All custom or complex types require a separate encoded / decoder, but because these encoders consume self as a value,
you have to move and un-move self to get around the Rust borrow checker. Specifically, when encoding the String
symbol_id as a fixed sized binary representation, you need to do 3 steps:

1) Create a sub-encoder that consumes self
2) Set all sub-fields
3) Return self by calling parent of the sub-encoder

```rust
     // Self (csg) moves into symbol_id_encoder
    let mut symbol_id_encoder = csg.exchange_symbol_id_encoder();
    let (first, second) = msg.symbol_id_exchange().exchange_order_id_binary();
    symbol_id_encoder.first(first);
    symbol_id_encoder.second(second);

    // Move Self (csg) from symbol_id_encoder back into csg.
    csg = symbol_id_encoder
        .parent()
        .expect("Failed to encode symbol id");
```  

The generated language bindings contain a sub-encoder for every complex type and they all follow the same pattern shown
above.

Rust Decimal is the only remaining important aspect that needs some elaboration. Fundamentally, you create a sub-encoder
as shown above to store the value and mantissa of your Decimal. However, the FIX 5 standard only comes with fixed
mantissa types for SBE Decimal. In a conventional setting, the 7 and 9 digits after the dot seem sufficient to cover all
cases. However, when dealing with crypto currencies and scientific data, you may want either a higher level of precision
or variable precision that replicates the exact number of digits after the dot without arbitrary truncation.

To do just that, you can define custom Decimal types with u8 encoded mantissa (scale in schema) to ensure you can
express arbitrary precision without truncation. For extreme cases, an u16 encoded mantissa gives quite a bit of extra
millage, but that is reserved for rather exotic scientific corner cases.

```xml 
      <!--        Variable sized Decimal types with exponent in range [0, 255] (u8) for internal use-->
        <composite name="DecimalPrice" description="A number representing price as Rust decimal">
            <type name="num" description="the number" primitiveType="int64"/>
            <type name="scale" description="the scale" primitiveType="uint8"/>
        </composite>

        <composite name="DecimalQty" description="A number representing quantity as Rust decimal">
            <type name="num" description="the number" primitiveType="int64"/>
            <type name="scale" description="the scale" primitiveType="uint8"/>
        </composite>

        <composite name="DecimalOptional" description="Optional Rust decimal">
            <type name="num" presence="optional" primitiveType="int64"/>
            <type name="scale" presence="optional" primitiveType="uint8"/>
        </composite>
```  

With these custom Decimal types in place, you can then encode the Rust Decimal seamlessly as following:

```rust
   // self moves into qty_encoder
    let mut qty_encoder = csg.order_qty_encoder();
    qty_encoder.num(
        msg.quantity()
            .mantissa()
            .to_i64()
            .expect("Failed to convert quantity decimal to i64"),
    );
    qty_encoder.scale(msg.quantity().scale() as u8);
    // Move Self (csg) from qty_encoder back into csg.
    csg = qty_encoder.parent().expect("Failed to encode order qty");
```  

When all fields are encoded, you have to extract the actual buffer size
by calling limit and then return a pair of the limit and actual buffer.
You may want to checksum here as well, but the SBE standard only prescribes to return limit and the byte buffer.

```rust
    let limit = csg.get_limit();
    Ok((limit, buffer))
```  

When the encoder is fully implemented, ensure to write and run unit tests.
For once you want to ensure encoding works, but equally important you wan to extract the actual buffer size from the
limit field and then update the fixed buffer allocated at the very top of the encoder to match exactly the actual limit.

**SBE decoder**

The decoder is usually simpler, faster, and easier to write. The usual SBE ceremony us straightforward, create a
decoder, create a buffer, decoder headers and you are good to go.

```rust
  pub fn decode_order_create_message(buffer: &[u8]) -> Result<OrderCreate, SbeDecodeError> {
    let mut csg = OrderCreateDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);
```  

Decoding the fields is equally straightforward although special attention is needed for Option<T> and complex types i.e.
Decimal.

```rust
    let exchange_id = ExchangeID::from(csg.exchange_id());

    let client_id = csg.client_id();

    let client_order_id = ClientOrderID::from(csg.client_order_id());

    let decoder = csg.exchange_symbol_id_decoder();
    let exchange_symbol_id = OrderExchangeSymbol::from((decoder.first(), decoder.second()));

    let order_type = OrderType::from(csg.order_type());

    let order_side = OrderSide::from(csg.order_side());

    let order_time_in_force = TimeInForce::from(csg.order_time_in_force())
```  

It really is that simple. Now, for optional types you have to do some checks
to return the correct option value i.e.

```rust
    let order_time_expiry = if csg.order_time_expiry().is_some() {
        let val = csg.order_time_expiry().unwrap();

        if val == 0 {
            None
        } else {
            Some(val)
        }
    } else {
        None
    };
```  

While this is a bit of a boilerplate, I urge you to follow this convention to make the returned option value as explicit
as possible. This helps enormously when debugging complex message encoding and decoding.

Lastly, complex types such as Decimal are actually easy to decode once
you get used to calling the custom decoder first.

```rust
    let qty_decoder = csg.order_qty_decoder();
    let order_quantity = Decimal::new(qty_decoder.num(), qty_decoder.scale() as u32);

    let price_decoder = csg.order_price_decoder();
    let order_price = Decimal::new(price_decoder.num(), price_decoder.scale() as u32);
```  

When all fields are decoded, just return a new instance of the Rust type i.e.

```rust
    Ok(OrderCreate::new(
        exchange_id,
        client_id,
        client_order_id,
        exchange_symbol_id,
        order_type,
        order_side,
        order_time_in_force,
        order_time_expiry,
        order_quantity,
        order_price,
    ))
```  

And that is all there is to know about writing an SBE decoder.

## Best practices

In practice a few best practices emerged specifically for working in a Rust only environment.

**Encode Rust Enum as the underlying type representation i.e. u8**

You can define SBE ENUM's but that implies mapping each Enum variant from
an SBE Enum from/to a Rust Enum. This was designed for a polyglot environment
where SBE Enums translates to many different programming languages. However,
in a Rust only environment you can just implement From<u8> and cast the SBE u8 straight into
a Rust Enum.

Code example:

* [Rust Enum](https://github.com/marvin-hansen/cgp-exploration/blob/main/queng_common/common_exchange/src/exchange_id.rs
  )
* [SBE Codec](https://github.com/marvin-hansen/cgp-exploration/tree/main/queng_sbe/sbe_messages_control/src/data_start)

In the example, the Rust Enum is encoded straight as a u8 and decoded by casting the u8 back into the Rust Enum thus
bypassing the entire boilerplate of SBE Enums altogether.

### Direct byte access to message fields

As mentioned earlier, the message type field is always at array index 2 therefore you can access it directly without
decoding the message. As stated above, it is sensible to encode / decoder Rust enums as the underlying primitive type
i.e. u8. That also means, when you know the index position in the byte array, you can directly access any of those Rust
enum fields without decoding the message.

For example, when you processing an inbound stream that is prone to some noise in form to unrelated messages, which are
usually identified by having some field other messages don't have, you can just extract exactly one byte, test that
field, and decided instantly whether to process that message at all or just drop it.

Another example is early error propagation. When your message type is a error message that contains a field that encodes
the exact error type, you can also extract that field without decoding the message, test if its a critical error, and if
so propagate the error; If not, decode the full message and handle the reported error accordingly.

There are many hidden gems in direct byte access to SBE message fields, but by far the most common one is extracting the
message type.

