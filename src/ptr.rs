use core::ptr::NonNull;

#[repr(transparent)]
pub struct Ptr<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Ptr<T> {
	#[cfg(feature = "const")]
	pub const unsafe fn new(ptr: *const T) -> Self {
		Self(NonNull::new(ptr as *mut T).unwrap())
	}

	#[cfg(not(feature = "const"))]
	pub unsafe fn new(ptr: *const T) -> Self {
		Self(NonNull::new(ptr as *mut T).unwrap())
	}

	#[cfg(feature = "const")]
	pub const unsafe fn as_ref<'a>(&self) -> &'a T { self.0.as_ref() }

	#[cfg(not(feature = "const"))]
	pub unsafe fn as_ref<'a>(&self) -> &'a T { self.0.as_ref() }

	#[cfg(feature = "const")]
	pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T { self.0.as_mut() }

	#[cfg(not(feature = "const"))]
	pub unsafe fn as_mut<'a>(&mut self) -> &'a mut T { self.0.as_mut() }
}