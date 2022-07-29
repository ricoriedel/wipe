use std::fmt::{Debug, Formatter};

pub struct Error(String);

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error(err.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_std_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "123");
        let msg = io_err.to_string();
        let err: Error = io_err.into();

        assert_eq!(msg, format!("{:?}", err));
    }
}
