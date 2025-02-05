use client_error_ext::*;
use common_sbe_errors::ClientError;
#[test]
fn test_decode() {
    let client_error = ClientError::default();
    let encoded = client_error.clone().encode_to_sbe().unwrap().1;
    let decoded = ClientError::decode_from_sbe(&encoded).unwrap();
    assert_eq!(client_error, decoded);
}