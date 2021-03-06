use std::{fmt, ptr::NonNull};
use swift_rt::metadata::{Metadata, MetadataKind};

/// The metatype for [`Any`](crate::Any), also known as `Any.Type`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct AnyType(
    // TODO: Use pointer type that takes advantage of
    // `_swift_abi_LeastValidPointerValue`.
    NonNull<Metadata>,
);

// SAFETY: The data referenced by the pointer is globally accessible.
unsafe impl Send for AnyType {}
unsafe impl Sync for AnyType {}

impl AnyType {
    #[inline]
    pub(crate) unsafe fn from_metadata(ty: NonNull<Metadata>) -> Self {
        Self(ty)
    }

    #[inline]
    pub(crate) fn metadata(&self) -> &'static Metadata {
        unsafe { &*self.0.as_ptr() }
    }

    /// Return the name of a Swift type represented by a metadata object.
    #[inline]
    #[doc(alias = "swift_getTypeName")]
    pub fn name(&self, qualified: bool) -> &'static str {
        self.metadata().name(qualified)
    }

    /// Returns the mangled name of a Swift type represented by a metadata
    /// object.
    ///
    /// # Availability
    ///
    /// **Swift:** 5.3
    #[inline]
    #[doc(alias = "swift_getMangledTypeName")]
    pub fn mangled_name(&self) -> &'static str {
        // TODO: Dynamically load the symbol at runtime and return `Result` with
        // missing symbol error type.
        self.metadata().mangled_name()
    }

    /// Returns this type as a class if it is one.
    #[inline]
    pub fn to_class(self) -> Option<AnyClass> {
        if self.is_class() {
            Some(AnyClass(self))
        } else {
            None
        }
    }

    /// Returns `true` if this type refers to any kind of class.
    #[inline]
    pub fn is_class(self) -> bool {
        self.metadata().kind().is_any_kind_of_class()
    }

    /// Returns `true` if this type refers to any kind of `Optional<T>`.
    #[inline]
    pub fn is_optional(self) -> bool {
        // TODO: Create `Metadata::is_optional`.
        self.metadata().raw_kind() == MetadataKind::OPTIONAL.value() as usize
    }
}

/// The protocol to which all class types implicitly conform.
///
/// See [documentation](https://developer.apple.com/documentation/swift/anyclass).
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AnyClass(AnyType);

impl AsRef<AnyType> for AnyClass {
    #[inline]
    fn as_ref(&self) -> &AnyType {
        &self.0
    }
}

impl From<AnyClass> for AnyType {
    #[inline]
    fn from(class: AnyClass) -> Self {
        class.0
    }
}

impl fmt::Debug for AnyClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("AnyClass").field(&(self.0).0).finish()
    }
}

impl AnyClass {
    #[inline]
    pub(crate) unsafe fn from_metadata(ty: NonNull<Metadata>) -> Self {
        Self(AnyType::from_metadata(ty))
    }

    /// Return the name of a Swift type represented by a metadata object.
    #[inline]
    #[doc(alias = "swift_getTypeName")]
    pub fn name(&self, qualified: bool) -> &'static str {
        self.0.name(qualified)
    }

    /// Returns the mangled name of a Swift type represented by a metadata
    /// object.
    ///
    /// # Availability
    ///
    /// **Swift:** 5.3
    #[inline]
    #[doc(alias = "swift_getMangledTypeName")]
    pub fn mangled_name(&self) -> &'static str {
        self.0.mangled_name()
    }
}
