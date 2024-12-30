use cow_ext::CloneExt;
use std::borrow::Cow;

#[test]
fn test_clone_ext_borrowed() {
    // Test with borrowed data
    let original_str = "test string";
    let cow_borrowed = Cow::Borrowed(original_str);

    // Perform shallow clone
    let cloned = cow_borrowed.shallow_clone();

    // Verify the content matches
    assert_eq!(cloned, "test string");

    // Verify we got a borrowed variant
    match cloned {
        Cow::Borrowed(_) => (),
        Cow::Owned(_) => panic!("Expected Borrowed variant, got Owned"),
    }
}

#[test]
fn test_clone_ext_owned() {
    // Test with owned data
    let owned_string = String::from("test string");
    let cow_owned: Cow<'_, str> = Cow::Owned(owned_string);

    // Perform shallow clone
    let cloned = cow_owned.shallow_clone();

    // Verify the content matches
    assert_eq!(cloned, "test string");

    // Verify we got a borrowed variant (even though original was owned)
    match cloned {
        Cow::Borrowed(_) => (),
        Cow::Owned(_) => panic!("Expected Borrowed variant, got Owned"),
    }
}

#[test]
fn test_clone_ext_lifetime() {
    // Create a string with explicit lifetime
    let original_string = String::from("test string");

    // Create a new scope to test lifetime behavior
    {
        let cow = Cow::Borrowed(original_string.as_str());
        let cloned = cow.shallow_clone();

        // Verify content and variant
        assert_eq!(cloned, "test string");
        match cloned {
            Cow::Borrowed(_) => (),
            Cow::Owned(_) => panic!("Expected Borrowed variant, got Owned"),
        }
    } // cloned is dropped here, but original_string lives on

    // Original string should still be valid
    assert_eq!(original_string, "test string");
}
