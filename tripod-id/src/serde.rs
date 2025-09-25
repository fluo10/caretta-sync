use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Error};

use crate::{Double, Single, Triple};

impl Serialize for Single {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Single {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Single::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for Double {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Double {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Double::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

impl Serialize for Triple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Triple {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s  = String::deserialize(deserializer)?;
        Triple::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use crate::TripodId;

    
    #[test]
    fn single() {
        assert_tokens(&crate::Single::NIL, &[Token::Str("000")]);
    }

    #[test]
    fn double() {
        assert_tokens(&crate::Double::NIL, &[Token::Str("000-000")]);
    }
    #[test]
    fn triple() {
        assert_tokens(&crate::Triple::NIL, &[Token::Str("000-000-000")]);
    }
}