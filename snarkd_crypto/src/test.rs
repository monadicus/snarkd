#[cfg(not(any(test, feature = "fuzz")))]
pub trait Testable {}
#[cfg(any(test, feature = "fuzz"))]
pub trait Testable: serde::Serialize + serde::de::DeserializeOwned {}
