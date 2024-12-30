use std::borrow::Cow;

/// Extension trait for performing efficient shallow cloning operations on string types.
///
/// This trait provides functionality to create a borrowed view of a string without
/// performing deep copies, regardless of whether the original is borrowed or owned.
///
/// # Type Parameters
/// * `'b` - The lifetime parameter for the borrowed data
///
/// # Methods
/// * `shallow_clone` - Creates a borrowed `Cow<str>` from either borrowed or owned data
///
/// # Implementation Notes
/// - Always returns a `Cow::Borrowed` variant, avoiding unnecessary allocations
/// - Works with both `Cow::Borrowed` and `Cow::Owned` variants
/// - Preserves the content while potentially changing the variant
/// - Uses lifetime elision for cleaner method signatures
///
/// # Examples
/// ```
/// use std::borrow::Cow;
/// use cow_ext::CloneExt;
///
/// // With borrowed data
/// let borrowed = Cow::Borrowed("hello");
/// let cloned = borrowed.shallow_clone();
/// assert!(matches!(cloned, Cow::Borrowed(_)));
///
/// // With owned data
/// let owned: Cow<'_, str> = Cow::Owned(String::from("world"));
/// let cloned = owned.shallow_clone();
/// assert!(matches!(cloned, Cow::Borrowed(_)));
/// ```
///
/// # Safety
/// - The trait is safe to implement as it only provides borrowed views of existing data
/// - No unsafe code is used in the default implementation
/// - Memory safety is guaranteed through Rust's lifetime system
///
pub trait CloneExt<'b> {
    fn shallow_clone(&'b self) -> Cow<'b, str>;
}

impl<'b> CloneExt<'b> for Cow<'b, str> {
    fn shallow_clone(&'b self) -> Cow<'b, str> {
        Cow::Borrowed(&**self)
    }
}
