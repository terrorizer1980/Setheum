// This file is part of Setheum.

// Copyright (C) 2020-2021 Setheum Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for setheum_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-25, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB
//! CACHE: 128

// Executed Command:
// target/release/setheum
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=setheum_dex
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/newrome/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for setheum_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> setheum_dex::WeightInfo for WeightInfo<T> {
	fn enable_trading_pair() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn disable_trading_pair() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn list_trading_pair() -> Weight {
		(25_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn add_liquidity() -> Weight {
		(134_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn add_liquidity_and_deposit() -> Weight {
		(187_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn remove_liquidity() -> Weight {
		(131_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn remove_liquidity_by_withdraw() -> Weight {
		(221_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn swap_with_exact_supply(u: u32) -> Weight {
		(98_000_000 as Weight)
			// Standard Error: 360_000
			.saturating_add((2_100_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	fn swap_with_exact_target(u: u32) -> Weight {
		(99_600_000 as Weight)
			// Standard Error: 268_000
			.saturating_add((1_100_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
}
