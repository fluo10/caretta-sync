use chrono::naive::serde::ts_microseconds::deserialize;
use iroh_tickets::{ParseError, Ticket};
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, de::Error as _, Serialize};

use crate::util::{decode_base32, encode_base32};

#[derive(Clone, Debug)]

pub struct DocTicket(iroh_docs::DocTicket);

impl From<iroh_docs::DocTicket> for DocTicket {
    fn from(value: iroh_docs::DocTicket) -> Self {
        Self(value)
    }
}
impl From<DocTicket> for iroh_docs::DocTicket {
    fn from(value: DocTicket) -> Self {
        value.0
    }
}

impl PartialEq for DocTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0.capability.raw() == other.0.capability.raw() &&
        self.0.nodes == other.0.nodes
    }
}

impl Serialize for DocTicket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let bytes =self.0.to_bytes();
        if serializer.is_human_readable() {
            serializer.serialize_str(&encode_base32(&bytes))
        } else {
            serializer.serialize_bytes(&bytes)
        }
    
    }
}
impl<'de> Deserialize<'de> for DocTicket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let bytes = if deserializer.is_human_readable() {
            decode_base32(&String::deserialize(deserializer)?).map_err(|e|D::Error::custom(e))?
        } else {
            Vec::<u8>::deserialize(deserializer)?
        };

        match iroh_docs::DocTicket::from_bytes(&bytes){
            Ok(x) => Ok(Self(x)),
            Err(e) => Err(D::Error::custom(e))
        }
    }
}

impl JsonSchema for DocTicket {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        stringify!(caretta_id::types::DocTicket).into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "pattern": "^[a-zA-Z0-9]+$"
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use iroh::{EndpointAddr, SecretKey};
    use iroh_docs::NamespaceSecret;
    use schemars::{SchemaGenerator, generate::SchemaSettings};

    use super::*;
    static DOC_TICKET: LazyLock<DocTicket> = LazyLock::new(|| DocTicket::from(
        iroh_docs::DocTicket::new(
            iroh_docs::Capability::Write(NamespaceSecret::new(&mut rand::rng())), 
            vec![EndpointAddr::new(SecretKey::generate(&mut rand::rng()).public())])
        ));

    #[test]
    fn json_conversion() {
        let s = serde_json::to_string(&*DOC_TICKET).unwrap();
        let t: DocTicket = serde_json::from_str(&s).unwrap();
        assert_eq!(t, *DOC_TICKET);
    }

    #[test]
    fn cbor_conversion() {
        let mut v: Vec<u8> = Vec::new();
        ciborium::into_writer(&*DOC_TICKET, &mut v);
        let t: DocTicket = ciborium::from_reader(v.as_slice()).unwrap();
        assert_eq!(t, *DOC_TICKET);
    }
    #[test]
    fn json_schema() {
        jsonschema::validate(
            &serde_json::to_value(DocTicket::json_schema(&mut SchemaGenerator::new(SchemaSettings::openapi3()))).unwrap(),
            &serde_json::to_value(&*DOC_TICKET).unwrap()
        ).unwrap();
    }
}