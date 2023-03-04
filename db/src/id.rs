use rand::prelude::*;

pub fn new_id() -> String {
    let value: u64 = random();
    let hex_value = format!("{:x}", value);
    hex_value
}

// use rand::prelude::*;
// use sqlx::encode::IsNull;
// use sqlx::error::BoxDynError;
// use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef};
// use sqlx::{Decode, Encode, Sqlite, Type};
// use std::ops::Deref;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Id(pub String);

// impl Id {
//     pub fn new() -> Self {
//         let value: u64 = random();
//         let hex_value = format!("{:x}", value);
//         Self(hex_value)
//     }
// }

// impl From<&str> for Id {
//     fn from(value: &str) -> Self {
//         Self(value.to_owned())
//     }
// }

// impl Into<String> for Id {
//     fn into(self) -> String {
//         self.0
//     }
// }

// impl Deref for Id {
//     type Target = String;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl Type<Sqlite> for Id {
//     fn type_info() -> SqliteTypeInfo {
//         <String as Type<Sqlite>>::type_info()
//     }
// }

// impl<'q> Encode<'q, Sqlite> for Id {
//     fn encode(self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
//         <String as Encode<'q, Sqlite>>::encode(self.0, args)
//     }

//     fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
//         <String as Encode<'q, Sqlite>>::encode_by_ref(&self.0, args)
//     }
// }

// impl<'r> Decode<'r, Sqlite> for Id {
//     fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
//         <String as Decode<'r, Sqlite>>::decode(value).map(|s| Self(s))
//     }
// }
