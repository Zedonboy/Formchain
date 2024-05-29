use candid::{CandidType, Principal};
use ic_ledger_types::BlockIndex;

pub type NotifyTopUpResult = Result<u128, NotifyError>;
#[derive(CandidType)]
pub enum RegistryError {
    UserIdExists,
    AmountBelowMin
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NotifyError {
    Refunded {
        reason: String,
        block_index: Option<BlockIndex>,
    },
    Processing,
    TransactionTooOld(BlockIndex),
    InvalidTransaction(String),
    Other {
        error_code: u64,
        error_message: String,
    },
}

#[derive(CandidType)]
pub struct NotifyTopUpArg {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}


use std::{
    borrow::Borrow, cell::RefCell, cmp, collections::{HashMap, HashSet}, default, str::FromStr, sync::Arc
};

use candid::{types::TypeInner};
use ic_cdk::{api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId}, caller};
use serde::{de::Visitor, Deserialize, Serialize};
use serde_bytes::ByteBuf;

pub type EMAIL_ADDRESS = String;
pub type MAIL_ID = String;
pub type NEWSLETTER_ID = String;

struct RcbytesVisitor;

impl<'de> Visitor<'de> for RcbytesVisitor {
    type Value = Rcbytes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rcbytes(Arc::new(ByteBuf::from(v))))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Rcbytes(Arc::new(ByteBuf::from(v))))
    }

    

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {

        let len = cmp::min(seq.size_hint().unwrap_or(0), 4096);
        let mut bytes = Vec::with_capacity(len);

        while let Some(b) = seq.next_element()? {
            bytes.push(b)
        };

        Ok(Rcbytes(Arc::new(ByteBuf::from(bytes))))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Rcbytes(Arc::new(ByteBuf::from(v))))
    }

   
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(v)
    }

}
pub struct Rcbytes(pub Arc<serde_bytes::ByteBuf>);

impl Rcbytes {
    pub fn new(arc : Arc<serde_bytes::ByteBuf>) -> Self {
        Rcbytes(arc)
    }
}

impl CandidType for Rcbytes {
    fn _ty() -> candid::types::Type {
        TypeInner::Vec(TypeInner::Nat8.into()).into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer {
       serializer.serialize_blob(&self.0)
    }
}

impl<'de> Deserialize<'de> for Rcbytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_bytes(RcbytesVisitor)
    }
}

impl Serialize for Rcbytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_bytes(&self.0)
    }
}

impl Clone for Rcbytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}