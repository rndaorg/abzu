use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumberError {
    #[error("Invalid number format: '{0}'")]
    InvalidFormat(String),
    #[error("Empty number string")]
    EmptyNumber,
    #[error("Multiple decimal points in number")]
    MultipleDecimals,
    #[error("Invalid digit in base-60 number: '{0}'")]
    InvalidSexagesimalDigit(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Sexagesimal(SexagesimalNum),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SexagesimalNum {
    pub integer_part: i64,
    pub fractional_part: i64, // stored as sixtieths (0-59)
    pub has_fraction: bool,
}

impl SexagesimalNum {
    pub fn new(integer: i64, fractional: i64) -> Result<Self, NumberError> {
        if fractional < 0 || fractional >= 60 {
            return Err(NumberError::InvalidFormat(
                format!("Fractional part must be between 0 and 59, got {}", fractional)
            ));
        }
        
        Ok(SexagesimalNum {
            integer_part: integer,
            fractional_part: fractional,
            has_fraction: fractional != 0,
        })
    }
    
    pub fn to_f64(&self) -> f64 {
        self.integer_part as f64 + (self.fractional_part as f64 / 60.0)
    }
    
    pub fn from_f64(value: f64) -> Self {
        let integer_part = value.floor() as i64;
        let fractional = ((value - integer_part as f64) * 60.0).round() as i64;
        
        SexagesimalNum {
            integer_part,
            fractional_part: fractional,
            has_fraction: fractional != 0,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{}", n),
            Value::Sexagesimal(sex) => write!(f, "{}", sex),
        }
    }
}

impl fmt::Display for SexagesimalNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_fraction {
            write!(f, "{};{:02}", self.integer_part, self.fractional_part)
        } else {
            write!(f, "{}", self.integer_part)
        }
    }
}

/// Parses a number string, detecting base-10 and sexagesimal formats
pub fn parse_number(s: &str) -> Result<Value, NumberError> {
    if s.is_empty() {
        return Err(NumberError::EmptyNumber);
    }
    
    // Check for sexagesimal notation (using ; as separator)
    if s.contains(';') {
        return parse_sexagesimal(s);
    }
    
    // Check for potential base-60 notation with commas (future use)
    if s.contains(',') {
        return parse_sexagesimal_comma(s);
    }
    
    // Regular base-10 number
    parse_base10(s)
}

fn parse_base10(s: &str) -> Result<Value, NumberError> {
    // Count decimal points to catch errors like "123.45.67"
    let decimal_count = s.chars().filter(|&c| c == '.').count();
    if decimal_count > 1 {
        return Err(NumberError::MultipleDecimals);
    }
    
    if decimal_count == 1 {
        // Parse as float
        match s.parse::<f64>() {
            Ok(f) => Ok(Value::Float(f)),
            Err(_) => Err(NumberError::InvalidFormat(s.to_string())),
        }
    } else {
        // Parse as integer
        match s.parse::<i64>() {
            Ok(i) => Ok(Value::Integer(i)),
            Err(_) => Err(NumberError::InvalidFormat(s.to_string())),
        }
    }
}

fn parse_sexagesimal(s: &str) -> Result<Value, NumberError> {
    let parts: Vec<&str> = s.split(';').collect();
    
    if parts.len() != 2 {
        return Err(NumberError::InvalidFormat(
            format!("Sexagesimal numbers must have exactly one ';' separator, got: {}", s)
        ));
    }
    
    let integer_part = parts[0].parse::<i64>()
        .map_err(|_| NumberError::InvalidFormat(parts[0].to_string()))?;
    
    let fractional_part = parts[1].parse::<i64>()
        .map_err(|_| NumberError::InvalidFormat(parts[1].to_string()))?;
    
    if fractional_part < 0 || fractional_part >= 60 {
        return Err(NumberError::InvalidFormat(
            format!("Fractional part must be between 0 and 59, got {}", fractional_part)
        ));
    }
    
    Ok(Value::Sexagesimal(SexagesimalNum::new(integer_part, fractional_part)?))
}

fn parse_sexagesimal_comma(s: &str) -> Result<Value, NumberError> {
    let parts: Vec<&str> = s.split(',').collect();
    
    // For now, we'll handle simple cases. Full base-60 support comes later.
    if parts.len() == 1 {
        // Single number, treat as base-10 for now
        parse_base10(parts[0])
    } else if parts.len() == 2 {
        // Two parts: integer and fractional (base-60)
        let integer_part = parts[0].parse::<i64>()
            .map_err(|_| NumberError::InvalidFormat(parts[0].to_string()))?;
        
        let fractional_part = parts[1].parse::<i64>()
            .map_err(|_| NumberError::InvalidFormat(parts[1].to_string()))?;
        
        if fractional_part < 0 || fractional_part >= 60 {
            return Err(NumberError::InvalidFormat(
                format!("Fractional part must be between 0 and 59, got {}", fractional_part)
            ));
        }
        
        let value = integer_part as f64 + (fractional_part as f64 / 60.0);
        Ok(Value::Float(value)) // Store as float for now, will convert to Sexagesimal later
    } else {
        // Multiple parts - full base-60 positional notation (for future)
        Err(NumberError::InvalidFormat(
            "Multi-position base-60 numbers not yet supported".to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_base10_integer() {
        assert_eq!(parse_number("123").unwrap(), Value::Integer(123));
        assert_eq!(parse_number("0").unwrap(), Value::Integer(0));
        assert_eq!(parse_number("-42").unwrap(), Value::Integer(-42));
    }

    #[test]
    fn test_parse_base10_float() {
        assert_eq!(parse_number("123.45").unwrap(), Value::Float(123.45));
        assert_eq!(parse_number("0.5").unwrap(), Value::Float(0.5));
        assert_eq!(parse_number("-3.14").unwrap(), Value::Float(-3.14));
    }

    #[test]
    fn test_parse_sexagesimal() {
        // Test semicolon notation (integer;fractional)
        let result = parse_number("1;30").unwrap();
        if let Value::Sexagesimal(sex) = result {
            assert_eq!(sex.integer_part, 1);
            assert_eq!(sex.fractional_part, 30);
            assert_eq!(sex.to_f64(), 1.5);
        } else {
            panic!("Expected Sexagesimal value");
        }
    }

    #[test]
    fn test_parse_sexagesimal_comma() {
        // Test comma notation for future base-60 support
        let result = parse_number("1,30").unwrap();
        assert!(matches!(result, Value::Float(1.5)));
    }

    #[test]
    fn test_sexagesimal_display() {
        let num = SexagesimalNum::new(2, 15).unwrap();
        assert_eq!(format!("{}", num), "2;15");
        
        let num = SexagesimalNum::new(5, 0).unwrap();
        assert_eq!(format!("{}", num), "5");
    }

    #[test]
    fn test_sexagesimal_from_float() {
        let sex = SexagesimalNum::from_f64(2.25);
        assert_eq!(sex.integer_part, 2);
        assert_eq!(sex.fractional_part, 15); // 0.25 * 60 = 15
        
        let sex = SexagesimalNum::from_f64(3.5);
        assert_eq!(sex.integer_part, 3);
        assert_eq!(sex.fractional_part, 30); // 0.5 * 60 = 30
    }

    #[test]
    fn test_invalid_numbers() {
        assert!(parse_number("").is_err());
        assert!(parse_number("123.45.67").is_err());
        assert!(parse_number("1;60").is_err()); // Fractional part too large
        assert!(parse_number("abc").is_err());
    }
}