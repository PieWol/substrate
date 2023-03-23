// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for frame_election_provider_support
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=frame-election-provider-support
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/election-provider-support/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for frame_election_provider_support.
pub trait WeightInfo {
	fn phragmen(v: u32, t: u32, e: u32, ) -> Weight;
	fn phragmms(v: u32, t: u32, e: u32, ) -> Weight;
	fn approval_voting(v: u32, t: u32, e: u32, ) -> Weight;
}

/// Weights for frame_election_provider_support using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn phragmen(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_149_617_000 picoseconds.
		Weight::from_parts(16_947_913_000, 0)
			// Standard Error: 176_584
			.saturating_add(Weight::from_parts(8_577_441, 0).saturating_mul(v.into()))
			// Standard Error: 10_861
			.saturating_add(Weight::from_parts(775_617, 0).saturating_mul(e.into()))
	}
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn phragmms(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_333_066_000 picoseconds.
		Weight::from_parts(9_421_750_000, 0)
			// Standard Error: 172_142
			.saturating_add(Weight::from_parts(8_792_110, 0).saturating_mul(v.into()))
			// Standard Error: 10_588
			.saturating_add(Weight::from_parts(801_984, 0).saturating_mul(e.into()))
	}
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn approval_voting(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_962_036_000 picoseconds.
		Weight::from_parts(3_989_759_000, 0)
			// Standard Error: 35_105
			.saturating_add(Weight::from_parts(2_035_777, 0).saturating_mul(v.into()))
			// Standard Error: 2_159
			.saturating_add(Weight::from_parts(124_045, 0).saturating_mul(e.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn phragmen(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_149_617_000 picoseconds.
		Weight::from_parts(16_947_913_000, 0)
			// Standard Error: 176_584
			.saturating_add(Weight::from_parts(8_577_441, 0).saturating_mul(v.into()))
			// Standard Error: 10_861
			.saturating_add(Weight::from_parts(775_617, 0).saturating_mul(e.into()))
	}
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn phragmms(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_333_066_000 picoseconds.
		Weight::from_parts(9_421_750_000, 0)
			// Standard Error: 172_142
			.saturating_add(Weight::from_parts(8_792_110, 0).saturating_mul(v.into()))
			// Standard Error: 10_588
			.saturating_add(Weight::from_parts(801_984, 0).saturating_mul(e.into()))
	}
	/// The range of component `v` is `[1000, 10000]`.
	/// The range of component `t` is `[17, 2000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn approval_voting(v: u32, _t: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_962_036_000 picoseconds.
		Weight::from_parts(3_989_759_000, 0)
			// Standard Error: 35_105
			.saturating_add(Weight::from_parts(2_035_777, 0).saturating_mul(v.into()))
			// Standard Error: 2_159
			.saturating_add(Weight::from_parts(124_045, 0).saturating_mul(e.into()))
	}
}
