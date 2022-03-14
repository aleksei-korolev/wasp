// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

#![allow(dead_code)]
#![allow(unused_mut)]

use wasmlib::*;

pub struct Erc721Events {
}

impl Erc721Events {

	pub fn approval(&self, approved: &ScAgentID, owner: &ScAgentID, token_id: &ScHash) {
		let mut evt = EventEncoder::new("erc721.approval");
		evt.encode(&agent_id_to_string(&approved));
		evt.encode(&agent_id_to_string(&owner));
		evt.encode(&hash_to_string(&token_id));
		evt.emit();
	}

	pub fn approval_for_all(&self, approval: bool, operator: &ScAgentID, owner: &ScAgentID) {
		let mut evt = EventEncoder::new("erc721.approvalForAll");
		evt.encode(&bool_to_string(approval));
		evt.encode(&agent_id_to_string(&operator));
		evt.encode(&agent_id_to_string(&owner));
		evt.emit();
	}

	pub fn init(&self, name: &str, symbol: &str) {
		let mut evt = EventEncoder::new("erc721.init");
		evt.encode(&string_to_string(&name));
		evt.encode(&string_to_string(&symbol));
		evt.emit();
	}

	pub fn mint(&self, balance: u64, owner: &ScAgentID, token_id: &ScHash) {
		let mut evt = EventEncoder::new("erc721.mint");
		evt.encode(&uint64_to_string(balance));
		evt.encode(&agent_id_to_string(&owner));
		evt.encode(&hash_to_string(&token_id));
		evt.emit();
	}

	pub fn transfer(&self, from: &ScAgentID, to: &ScAgentID, token_id: &ScHash) {
		let mut evt = EventEncoder::new("erc721.transfer");
		evt.encode(&agent_id_to_string(&from));
		evt.encode(&agent_id_to_string(&to));
		evt.encode(&hash_to_string(&token_id));
		evt.emit();
	}
}
