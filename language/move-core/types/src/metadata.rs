// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use sp_std::vec::Vec;
/// Representation of metadata,
#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
pub struct Metadata {
    /// The key identifying the type of metadata.
    pub key: Vec<u8>,
    /// The value of the metadata.
    pub value: Vec<u8>,
}
