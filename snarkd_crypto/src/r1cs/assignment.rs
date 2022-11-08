use anyhow::{bail, Result};

pub trait Assignment<T> {
    fn get(self) -> Result<T>;

    fn get_ref(&self) -> Result<&T>;
}

impl<T> Assignment<T> for Option<T> {
    fn get(self) -> Result<T> {
        match self {
            Some(v) => Ok(v),
            None => bail!("AssignmentMissing"),
        }
    }

    fn get_ref(&self) -> Result<&T> {
        match *self {
            Some(ref v) => Ok(v),
            None => bail!("AssignmentMissing"),
        }
    }
}
