
//! Autogenerated weights for pallet_storage_handler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ubuntu`, CPU: `Intel(R) Core(TM) i5-10400 CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-initial-devnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// pallet
// --chain
// cess-initial-devnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_storage_handler
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/storage-handler/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_storage_handler.
pub trait WeightInfo {
	fn buy_space() -> Weight;
	fn expansion_space() -> Weight;
	fn renewal_space() -> Weight;
}

/// Weights for pallet_storage_handler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: StorageHandler PurchasedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:0)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn buy_space() -> Weight {
		// Minimum execution time: 92_464 nanoseconds.
		Weight::from_parts(93_249_000, 0)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: StorageHandler PurchasedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:0)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:0)
	fn expansion_space() -> Weight {
		// Minimum execution time: 84_456 nanoseconds.
		Weight::from_parts(87_497_000, 0)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn renewal_space() -> Weight {
		// Minimum execution time: 75_841 nanoseconds.
		Weight::from_parts(78_035_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: StorageHandler PurchasedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:0)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn buy_space() -> Weight {
		// Minimum execution time: 92_464 nanoseconds.
		Weight::from_parts(93_249_000, 0)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: StorageHandler PurchasedSpace (r:1 w:1)
	// Storage: StorageHandler TotalIdleSpace (r:1 w:0)
	// Storage: StorageHandler TotalServiceSpace (r:1 w:0)
	fn expansion_space() -> Weight {
		// Minimum execution time: 84_456 nanoseconds.
		Weight::from_parts(87_497_000, 0)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: StorageHandler UserOwnedSpace (r:1 w:1)
	// Storage: StorageHandler UnitPrice (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn renewal_space() -> Weight {
		// Minimum execution time: 75_841 nanoseconds.
		Weight::from_parts(78_035_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}
