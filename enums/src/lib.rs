use std::io;

pub enum AppError {
    Code { full: usize, short: u16 },
    Message(String),
    IOWrapper(io::Error),
    Unknown,
}

impl AppError {
    pub fn print_kind(&self, mut to: &mut impl io::Write) -> io::Result<()> {
        let kind = match self {
            AppError::Code { full: _, short: _ } => "Code",
            AppError::Message(_) => "Message",
            AppError::IOWrapper(_) => "IoWrapper",
            AppError::Unknown => "Unknown",
        };
        write!(&mut to, "{}", kind)?;
        Ok(())
    }
}

pub fn throw_err(num: i32) -> Result<(), AppError> {
    if num < -200 {
        return Err(AppError::IOWrapper(io::Error::from(
            io::ErrorKind::Other,
        )));
    } else if num == 42 {
        return Err(AppError::Code{ full: num as usize, short: (num % std::u16::MAX as i32) as u16});
    } else if num > 42 {
        return Err(AppError::Message(format!("{} was a bad choice", num)));
    } else {
        return Err(AppError::Unknown);
    }
}

#[cfg(test)]
mod tests {
    use super::{ AppError, throw_err };
    use std::io;

    #[test]
    fn throw_err_test() {
        let num = 42;
        if let Err(err) = throw_err(num) {
            match err {
                AppError::Code{full: f, short: _} =>
                assert_eq!(num as usize, f),
                AppError::Unknown |
                AppError::IOWrapper(_) =>
                    assert!(num < -200),
                AppError::Message(s) =>
                    assert_eq!(format!("{} was a bad choice", num), s)
            }
        }
    }

    #[test]
    fn test_application_error_get_kind() {
        let mut target = vec![];
        let _ = AppError::Code { full: 100, short: 100 
        }.print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), 
        "Code".to_string());
        
        let mut target = vec![];
        let _ = AppError::Message("0".to_string()).
        print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), 
        "Message".to_string());
        
        let mut target = vec![];
        let _ = AppError::Unknown.print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), 
        "Unknown".to_string());
        
        let mut target = vec![];
        let error = io::Error::from(io::ErrorKind::WriteZero);
        let _ = AppError::IOWrapper(error).print_kind(&mut 
        target);
        assert_eq!(String::from_utf8(target).unwrap(), 
        "IoWrapper".to_string());

    }
}
