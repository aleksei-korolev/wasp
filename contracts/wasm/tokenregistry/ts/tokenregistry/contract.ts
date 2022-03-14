// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

import * as wasmlib from "wasmlib";
import * as sc from "./index";

export class MintSupplyCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncMintSupply);
	params: sc.MutableMintSupplyParams = new sc.MutableMintSupplyParams(wasmlib.ScView.nilProxy);
}

export class MintSupplyContext {
	params: sc.ImmutableMintSupplyParams = new sc.ImmutableMintSupplyParams(wasmlib.paramsProxy());
	state: sc.MutableTokenRegistryState = new sc.MutableTokenRegistryState(wasmlib.ScState.proxy());
}

export class TransferOwnershipCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncTransferOwnership);
	params: sc.MutableTransferOwnershipParams = new sc.MutableTransferOwnershipParams(wasmlib.ScView.nilProxy);
}

export class TransferOwnershipContext {
	params: sc.ImmutableTransferOwnershipParams = new sc.ImmutableTransferOwnershipParams(wasmlib.paramsProxy());
	state: sc.MutableTokenRegistryState = new sc.MutableTokenRegistryState(wasmlib.ScState.proxy());
}

export class UpdateMetadataCall {
	func: wasmlib.ScFunc = new wasmlib.ScFunc(sc.HScName, sc.HFuncUpdateMetadata);
	params: sc.MutableUpdateMetadataParams = new sc.MutableUpdateMetadataParams(wasmlib.ScView.nilProxy);
}

export class UpdateMetadataContext {
	params: sc.ImmutableUpdateMetadataParams = new sc.ImmutableUpdateMetadataParams(wasmlib.paramsProxy());
	state: sc.MutableTokenRegistryState = new sc.MutableTokenRegistryState(wasmlib.ScState.proxy());
}

export class GetInfoCall {
	func: wasmlib.ScView = new wasmlib.ScView(sc.HScName, sc.HViewGetInfo);
	params: sc.MutableGetInfoParams = new sc.MutableGetInfoParams(wasmlib.ScView.nilProxy);
}

export class GetInfoContext {
	params: sc.ImmutableGetInfoParams = new sc.ImmutableGetInfoParams(wasmlib.paramsProxy());
	state: sc.ImmutableTokenRegistryState = new sc.ImmutableTokenRegistryState(wasmlib.ScState.proxy());
}

export class ScFuncs {
	static mintSupply(_ctx: wasmlib.ScFuncCallContext): MintSupplyCall {
		const f = new MintSupplyCall();
		f.params = new sc.MutableMintSupplyParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static transferOwnership(_ctx: wasmlib.ScFuncCallContext): TransferOwnershipCall {
		const f = new TransferOwnershipCall();
		f.params = new sc.MutableTransferOwnershipParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static updateMetadata(_ctx: wasmlib.ScFuncCallContext): UpdateMetadataCall {
		const f = new UpdateMetadataCall();
		f.params = new sc.MutableUpdateMetadataParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}

	static getInfo(_ctx: wasmlib.ScViewCallContext): GetInfoCall {
		const f = new GetInfoCall();
		f.params = new sc.MutableGetInfoParams(wasmlib.newCallParamsProxy(f.func));
		return f;
	}
}
