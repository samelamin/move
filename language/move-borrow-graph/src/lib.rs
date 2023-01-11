// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod graph;
mod paths;
pub mod references;
mod shared;
