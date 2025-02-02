// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_cacher
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-initial-devnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// --chain
// cess-initial-devnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_cacher
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/cacher/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cacher.
pub trait WeightInfo {
	fn register() -> Weight;
	fn update() -> Weight;
	fn logout() -> Weight;
	fn pay(v: u32, ) -> Weight;
}

/// Weights for pallet_cacher using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Cacher Cachers (r:1 w:1)
	fn register() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Cacher Cachers (r:1 w:1)
	fn update() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Cacher Cachers (r:1 w:1)
	fn logout() -> Weight {
		Weight::from_parts(37_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:0)
	fn pay(v: u32, ) -> Weight {
		Weight::from_parts(43_773_000, 0)
			// Standard Error: 171_000
			.saturating_add(Weight::from_parts(26_133_000, 0).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Cacher Cachers (r:1 w:1)
	fn register() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Cacher Cachers (r:1 w:1)
	fn update() -> Weight {
		Weight::from_parts(40_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Cacher Cachers (r:1 w:1)
	fn logout() -> Weight {
		Weight::from_parts(37_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: System Account (r:1 w:0)
	fn pay(v: u32, ) -> Weight {
		Weight::from_parts(43_773_000, 0)
			// Standard Error: 171_000
			.saturating_add(Weight::from_parts(26_133_000, 0).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}