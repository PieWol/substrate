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

//! Move contracts' _reserved_ balance from the `deposit_account` to be _held_ in the contract's
//! account instead. Since [`Currency`](frame_support::traits::Currency) has been [deprecated]
//! (https://github.com/paritytech/substrate/pull/12951), we need the deposits to be handled by the
//! [`frame_support::traits::fungible`] traits instead. For this transfer the balance in the
//! deposit account to the contract's account and hold it in there.
//! Then the deposit account is not needed anymore and we can get rid of it.

use crate::{
	migration::{IsFinished, MigrationStep},
	storage::DepositAccount,
	weights::WeightInfo,
	BalanceOf, CodeHash, Config, ContractInfoOf, HoldReason, Pallet, TrieId, Weight, LOG_TARGET,
};
#[cfg(feature = "try-runtime")]
use crate::{BalanceOf, ContractInfo};
#[cfg(feature = "try-runtime")]
use frame_support::{dispatch::Vec, traits::fungible::InspectHold};
use frame_support::{
	pallet_prelude::*,
	storage_alias,
	traits::{
		fungible::{Mutate, MutateHold},
		tokens::{fungible::Inspect, Fortitude, Preservation},
	},
	BoundedBTreeMap, DefaultNoBound,
};
use frame_system::Pallet as System;
use sp_core::hexdisplay::HexDisplay;
#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;
use sp_runtime::{traits::Zero, Saturating};

mod old {
	use super::*;

	#[derive(
		Encode, Decode, CloneNoBound, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct ContractInfo<T: Config> {
		pub trie_id: TrieId,
		pub deposit_account: DepositAccount<T>,
		pub code_hash: CodeHash<T>,
		pub storage_bytes: u32,
		pub storage_items: u32,
		pub storage_byte_deposit: BalanceOf<T>,
		pub storage_item_deposit: BalanceOf<T>,
		pub storage_base_deposit: BalanceOf<T>,
		pub delegate_dependencies:
			BoundedBTreeMap<CodeHash<T>, BalanceOf<T>, T::MaxDelegateDependencies>,
	}

	#[storage_alias]
	pub type ContractInfoOf<T: Config> = StorageMap<
		Pallet<T>,
		Twox64Concat,
		<T as frame_system::Config>::AccountId,
		ContractInfo<T>,
	>;
}

#[cfg(feature = "runtime-benchmarks")]
pub fn store_old_contract_info<T: Config>(account: T::AccountId, info: crate::ContractInfo<T>) {
	let info = old::ContractInfo {
		trie_id: info.trie_id.clone(),
		deposit_account: info.deposit_account().clone(),
		code_hash: info.code_hash,
		storage_bytes: Default::default(),
		storage_items: Default::default(),
		storage_byte_deposit: Default::default(),
		storage_item_deposit: Default::default(),
		storage_base_deposit: Default::default(),
		delegate_dependencies: Default::default(),
	};
	old::ContractInfoOf::<T>::insert(account, info);
}

#[derive(Encode, Decode, MaxEncodedLen, DefaultNoBound)]
pub struct Migration<T: Config> {
	last_account: Option<T::AccountId>,
}

impl<T: Config> MigrationStep for Migration<T> {
	const VERSION: u16 = 15;

	fn max_step_weight() -> Weight {
		T::WeightInfo::v15_migration_step()
	}

	fn step(&mut self) -> (IsFinished, Weight) {
		let mut iter = if let Some(last_account) = self.last_account.take() {
			ContractInfoOf::<T>::iter_from(ContractInfoOf::<T>::hashed_key_for(last_account))
		} else {
			ContractInfoOf::<T>::iter()
		};

		if let Some((account, contract)) = iter.next() {
			let deposit_account = &contract.deposit_account();
			if System::<T>::consumers(deposit_account) > Zero::zero() {
				System::<T>::dec_consumers(deposit_account);
			}

			// Get the deposit balance to transfer.
			let total_deposit_balance = T::Currency::total_balance(deposit_account);
			let reducible_deposit_balance = T::Currency::reducible_balance(
				deposit_account,
				Preservation::Expendable,
				Fortitude::Force,
			);

			if total_deposit_balance > reducible_deposit_balance {
				// This should never happen, as by design all balance in the deposit account is
				// storage deposit and therefore reducible after decrementing the consumer
				// reference.
				log::warn!(
					target: LOG_TARGET,
					"Deposit account 0x{:?} for contract 0x{:?} has some non-reducible balance {:?} from a total of {:?} that will remain in there.",
					HexDisplay::from(&deposit_account.encode()),
					HexDisplay::from(&account.encode()),
					total_deposit_balance.saturating_sub(reducible_deposit_balance),
					total_deposit_balance
				);
			}

			// Move balance reserved from the deposit account back to the contract account.
			// Let the deposit account die.
			let transferred_deposit_balance = T::Currency::transfer(
				deposit_account,
				&account,
				reducible_deposit_balance,
				Preservation::Expendable,
			)
			.map(|_| {
				log::debug!(
					target: LOG_TARGET,
					"{:?} transferred from the deposit account 0x{:?} to the contract 0x{:?}.",
					reducible_deposit_balance,
					HexDisplay::from(&deposit_account.encode()),
					HexDisplay::from(&account.encode())
				);
				reducible_deposit_balance
			})
			.unwrap_or_else(|err| {
				log::error!(
					target: LOG_TARGET,
					"Failed to transfer {:?} from the deposit account 0x{:?} to the contract 0x{:?}, reason: {:?}.",
					reducible_deposit_balance,
					HexDisplay::from(&deposit_account.encode()),
					HexDisplay::from(&account.encode()),
					err
				);
				Zero::zero()
			});

			// Hold the reserved balance.
			if transferred_deposit_balance == Zero::zero() {
				log::warn!(
					target: LOG_TARGET,
					"No balance to hold as storage deposit on the contract 0x{:?}.",
					HexDisplay::from(&account.encode())
				);
			} else {
				T::Currency::hold(
					&HoldReason::StorageDepositReserve.into(),
					&account,
					transferred_deposit_balance,
				)
				.map(|_| {
					log::debug!(
						target: LOG_TARGET,
						"Successfully held {:?} as storage deposit on the contract 0x{:?}.",
						transferred_deposit_balance,
						HexDisplay::from(&account.encode())
					);
				})
				.unwrap_or_else(|err| {
					log::error!(
						target: LOG_TARGET,
						"Failed to hold {:?} as storage deposit on the contract 0x{:?}, reason: {:?}.",
						transferred_deposit_balance,
						HexDisplay::from(&account.encode()),
						err
					);
				});
			}

			log::debug!(target: LOG_TARGET, "===");

			// Store last key for next migration step
			self.last_account = Some(account);

			(IsFinished::No, T::WeightInfo::v15_migration_step())
		} else {
			log::info!(target: LOG_TARGET, "Done Migrating Storage Deposits.");
			(IsFinished::Yes, T::WeightInfo::v15_migration_step())
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade_step() -> Result<Vec<u8>, TryRuntimeError> {
		let sample: Vec<_> = ContractInfoOf::<T>::iter().take(100).collect();

		log::debug!(target: LOG_TARGET, "Taking sample of {} contracts", sample.len());

		let state: Vec<(T::AccountId, ContractInfo<T>, BalanceOf<T>, BalanceOf<T>)> = sample
			.iter()
			.map(|(account, contract)| {
				let deposit_balance = T::Currency::total_balance(&contract.deposit_account());
				let account_balance = T::Currency::total_balance(&account);
				(
					account.clone(),
					contract.clone(),
					account_balance.saturating_add(deposit_balance),
					deposit_balance,
				)
			})
			.collect();

		Ok(state.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade_step(state: Vec<u8>) -> Result<(), TryRuntimeError> {
		let sample =
			<Vec<(T::AccountId, ContractInfo<T>, BalanceOf<T>, BalanceOf<T>)> as Decode>::decode(
				&mut &state[..],
			)
			.expect("pre_upgrade_step provides a valid state; qed");

		log::debug!(target: LOG_TARGET, "Validating sample of {} contracts", sample.len());
		for (account, contract, old_total_balance, deposit_balance) in sample {
			log::debug!(target: LOG_TARGET, "===");
			log::debug!(target: LOG_TARGET, "Account: 0x{} ", HexDisplay::from(&account.encode()));

			ensure!(
				old_total_balance == T::Currency::total_balance(&account),
				"total balance mismatch"
			);
			ensure!(
				deposit_balance ==
					T::Currency::balance_on_hold(
						&HoldReason::StorageDepositReserve.into(),
						&account
					),
				"deposit mismatch"
			);
			ensure!(
				!System::<T>::account_exists(&contract.deposit_account()),
				"deposit account still exists"
			);
		}

		Ok(())
	}
}
