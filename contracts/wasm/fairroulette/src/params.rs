// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;
use wasmlib::host::*;

use crate::*;
use crate::keys::*;
use crate::structs::*;

#[derive(Clone, Copy)]
pub struct ImmutablePlaceBetParams {
    pub(crate) id: i32,
}

impl ImmutablePlaceBetParams {
    pub fn number(&self) -> ScImmutableInt64 {
		ScImmutableInt64::new(self.id, idx_map(IDX_PARAM_NUMBER))
	}
}

#[derive(Clone, Copy)]
pub struct MutablePlaceBetParams {
    pub(crate) id: i32,
}

impl MutablePlaceBetParams {
    pub fn number(&self) -> ScMutableInt64 {
		ScMutableInt64::new(self.id, idx_map(IDX_PARAM_NUMBER))
	}
}

#[derive(Clone, Copy)]
pub struct ImmutablePlayPeriodParams {
    pub(crate) id: i32,
}

impl ImmutablePlayPeriodParams {
    pub fn play_period(&self) -> ScImmutableInt32 {
		ScImmutableInt32::new(self.id, idx_map(IDX_PARAM_PLAY_PERIOD))
	}
}

#[derive(Clone, Copy)]
pub struct MutablePlayPeriodParams {
    pub(crate) id: i32,
}

impl MutablePlayPeriodParams {
    pub fn play_period(&self) -> ScMutableInt32 {
		ScMutableInt32::new(self.id, idx_map(IDX_PARAM_PLAY_PERIOD))
	}
}