// This file is part of Setheum.

// Copyright (C) 2019-2021 Setheum Labs.
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

//! Autogenerated weights for setheum_settway
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-01, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB
//! CACHE: 128

// Executed Command:
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/neom/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for setheum_settway.
pub struct WeightInfo<T>(_);
impl<T: frame_system::Config> setheum_settway::WeightInfo for WeightInfo<T> {
	fn authorize() -> Weight {
		(25_781_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize() -> Weight {
		(25_446_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unauthorize_all(c: u32) -> Weight {
		(26_379_000 as Weight)
			// Standard Error: 33_000
			.saturating_add((1_680_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	fn adjust_setter() -> Weight {
		(307_959_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(24 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn transfer_reserve_from() -> Weight {
		(220_884_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
