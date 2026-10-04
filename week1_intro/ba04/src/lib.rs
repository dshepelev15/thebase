pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    if (u8::MAX - a) >= b {
        Some(a + b)
    }
    else {
        None
    }
}

pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    let left = u8::MAX - a;
    if left >= b {
        a + b
    } else {
        b - left - 1
    }
}

pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    let left = u8::MAX - a;
    if left >= b {
        a + b
    } else {
        u8::MAX
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);

        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);
    }

    #[test]
    fn test_add_u8_checked_good_case() {
        let a = 0;
        let b = 128;

        assert_eq!(add_u8_checked(a, b), Some(128));
    }

    #[test]
    fn test_add_u8_checked_overflow_case() {
        let a = 192;
        let b = 128;

        assert_eq!(add_u8_checked(a, b), None);
    }
    
}
