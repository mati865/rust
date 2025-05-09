#![no_std]

//@ is "$.index[?(@.name=='example')].attrs" '["#[must_use]\n"]'
#[must_use]
pub fn example() -> impl Iterator<Item = i64> {}

//@ is "$.index[?(@.name=='explicit_message')].attrs" '["#[must_use = \"does nothing if you do not use it\"]\n"]'
#[must_use = "does nothing if you do not use it"]
pub fn explicit_message() -> impl Iterator<Item = i64> {}
