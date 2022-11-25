#[cfg(not(test))]
pub trait Testable {}
#[cfg(test)]
pub trait Testable: serde::Serialize + serde::de::DeserializeOwned {}
