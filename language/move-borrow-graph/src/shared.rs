// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
use sp_std::collections::{btree_map::BTreeMap,btree_set::BTreeSet};

pub fn remap_set<T: Copy + Ord>(set: &mut BTreeSet<T>, id_map: &BTreeMap<T, T>) {
    let _before = set.len();
    *set = sp_std::mem::take(set)
        .into_iter()
        .map(|x| id_map.get(&x).copied().unwrap_or(x))
        .collect();
    let _after = set.len();
    debug_assert!(_before == _after);
}
