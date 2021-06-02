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

//! # SERP Treasury Module
//!
//! ## Overview
//!
//! SERP Treasury manages the accumulated fees and bad standards generated by
//! Settmint, and handle excess serplus or standards timely in order to keep the
//! system healthy. It's the only entry for issuing/burning stable
//! coins for the entire system.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{pallet_prelude::*, transactional};
use frame_system::pallet_prelude::*;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use primitives::{Balance, CurrencyId};
use sp_runtime::{
	traits::{AccountIdConversion, One, Zero},
	DispatchError, DispatchResult, FixedPointNumber, ModuleId,
};
use support::{SerpAuction, SerpTreasury, SerpTreasuryExtended, DEXManager, Ratio};

mod benchmarking;
mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The origin which may update parameters and handle
		/// serplus/standard/reserve. Root can always do this.
		type UpdateOrigin: EnsureOrigin<Self::Origin>;

		/// The Currency for managing assets related to Settmint
		type Currency: MultiCurrencyExtended<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

		#[pallet::constant]
		/// Stablecoin currency id
		type GetStableCurrencyId: Get<CurrencyId>;

		/// Auction manager creates different types of auction to handle system serplus and standard.
		type SerpAuctionHandler: SerpAuction<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

		/// Dex manager is used to swap reserve asset (Setter) for propper (SettCurrency).
		type DEX: DEXManager<Self::AccountId, CurrencyId, Balance>;

		#[pallet::constant]
		/// The cap of lots when an auction is created
		type MaxAuctionsCount: Get<u32>;

		#[pallet::constant]
		/// The SERP Treasury's module id, keeps serplus and reserve asset.
		type ModuleId: Get<ModuleId>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The reserve amount of SERP Treasury is not enough
		ReserveNotEnough,
		/// The serplus pool of SERP Treasury is not enough
		SerplusPoolNotEnough,
		/// standard pool overflow
		StandardPoolOverflow,
		/// The standard pool of SERP Treasury is not enough
		StandardPoolNotEnough,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// The expected amount size for per lot collateral auction of the
		/// reserve type updated. \[reserve_type, new_size\]
		ExpectedSetterAuctionSizeUpdated(CurrencyId, Balance),
	}

	/// The maximum amount of reserve amount for sale per setter auction
	#[pallet::storage]
	#[pallet::getter(fn expected_setter_auction_size)]
	pub type ExpectedSetterAuctionSize<T: Config> = StorageMap<_, Twox64Concat, CurrencyId, Balance, ValueQuery>;

	/// Current total standard value of system. It's not same as standard in Settmint
	/// engine, it is the bad standard of the system.
	#[pallet::storage]
	#[pallet::getter(fn standard_pool)]
	pub type StandardPool<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub expected_setter_auction_size: Vec<(CurrencyId, Balance)>,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			GenesisConfig {
				expected_setter_auction_size: vec![],
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			self.expected_setter_auction_size
				.iter()
				.for_each(|(currency_id, size)| {
					ExpectedSetterAuctionSize::<T>::insert(currency_id, size);
				});
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		/// Handle excessive serplus or standards of system when block end
		fn on_finalize(_now: T::BlockNumber) {
			// offset the same amount between standard pool and serplus pool
			Self::offset_serplusand_standard();
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::auction_serplus())]
		#[transactional]
		pub fn auction_serplus(origin: OriginFor<T>, amount: Balance) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			ensure!(
				Self::serpluspool().saturating_sub(T::SerpAuctionHandler::get_total_serplusin_auction()) >= amount,
				Error::<T>::SerplusPoolNotEnough,
			);
			T::SerpAuctionHandler::new_serplus_auction(amount)?;
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::auction_standard())]
		#[transactional]
		pub fn auction_standard(
			origin: OriginFor<T>,
			standard_amount: Balance,
			initial_price: Balance,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			ensure!(
				Self::standard_pool().saturating_sub(T::SerpAuctionHandler::get_total_standard_in_auction())
					>= standard_amount,
				Error::<T>::StandardPoolNotEnough,
			);
			T::SerpAuctionHandler::new_diamond_auction(initial_price, standard_amount)?;
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::auction_reserve())]
		#[transactional]
		pub fn auction_reserve(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			amount: Balance,
			target: Balance,
			splited: bool,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			<Self as SerpTreasuryExtended<T::AccountId>>::create_setter_auctions(
				currency_id,
				amount,
				target,
				Self::account_id(),
				splited,
			)?;
			Ok(().into())
		}

		/// Update parameters related to setter auction under specific
		/// reserve type
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		///
		/// - `currency_id`: reserve type
		/// - `serplusbuffer_size`: setter auction maximum size
		#[pallet::weight((T::WeightInfo::set_expected_setter_auction_size(), DispatchClass::Operational))]
		#[transactional]
		pub fn set_expected_setter_auction_size(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			size: Balance,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			ExpectedSetterAuctionSize::<T>::insert(currency_id, size);
			Self::deposit_event(Event::ExpectedSetterAuctionSizeUpdated(currency_id, size));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Get account of SERP Treasury module.
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}

	/// Get current total serplus of system.
	pub fn serpluspool() -> Balance {
		T::Currency::free_balance(T::GetStableCurrencyId::get(), &Self::account_id())
	}

	/// Get total reserve amount of SERP Treasury module.
	pub fn total_reserves(currency_id: CurrencyId) -> Balance {
		T::Currency::free_balance(currency_id, &Self::account_id())
	}

	/// Get reserve amount not in auction
	pub fn total_reserves_not_in_auction(currency_id: CurrencyId) -> Balance {
		T::Currency::free_balance(currency_id, &Self::account_id())
			.saturating_sub(T::SerpAuctionHandler::get_total_reserve_in_auction(currency_id))
	}

	fn offset_serplusand_standard() {
		let offset_amount = sp_std::cmp::min(Self::standard_pool(), Self::serpluspool());

		// Burn the amount that is equal to offset_amount of settcurrency.
		if !offset_amount.is_zero()
			&& T::Currency::withdraw(T::GetStableCurrencyId::get(), &Self::account_id(), offset_amount).is_ok()
		{
			StandardPool::<T>::mutate(|standard| {
				*standard = standard
					.checked_sub(offset_amount)
					.expect("offset = min(standard, serplus); qed")
			});
		}
	}
}

impl<T: Config> SerpTreasury<T::AccountId> for Pallet<T> {
	type Balance = Balance;
	type CurrencyId = CurrencyId;

	fn get_serpluspool() -> Self::Balance {
		Self::serpluspool()
	}

	fn get_standard_pool() -> Self::Balance {
		Self::standard_pool()
	}

	fn get_total_reserves(id: Self::CurrencyId) -> Self::Balance {
		Self::total_reserves(id)
	}

	fn get_standard_proportion(amount: Self::Balance) -> Ratio {
		let stable_total_supply = T::Currency::total_issuance(T::GetStableCurrencyId::get());
		Ratio::checked_from_rational(amount, stable_total_supply).unwrap_or_default()
	}

	fn on_system_standard(amount: Self::Balance) -> DispatchResult {
		StandardPool::<T>::try_mutate(|standard_pool| -> DispatchResult {
			*standard_pool = standard_pool.checked_add(amount).ok_or(Error::<T>::StandardPoolOverflow)?;
			Ok(())
		})
	}

	fn on_system_serplus(amount: Self::Balance) -> DispatchResult {
		Self::issue_standard(&Self::account_id(), amount)
	}

	/// TODO: update to `currency_id` which is any `SettCurrency`.
	fn issue_standard(who: &T::AccountId, standard: Self::Balance) -> DispatchResult {
		T::Currency::deposit(T::GetStableCurrencyId::get(), who, standard)?;
		Ok(())
	}

	/// TODO: update to `currency_id` which is any `SettCurrency`.
	fn burn_standard(who: &T::AccountId, standard: Self::Balance) -> DispatchResult {
		T::Currency::withdraw(T::GetStableCurrencyId::get(), who, standard)
	}

	/// Issue Dexer (`SDEX` in Setheum or `HALAL` in Neom). `dexer` here just referring to the DEX token balance.
	/// TODO: update to `T::GetDexCurrencyId::get()` which is any `SettinDex` coin.
	fn issue_dexer(who: &T::AccountId, dexer: Self::Balance) -> DispatchResult {
		T::Currency::deposit(T::GetStableCurrencyId::get(), who, dexer)?;
		Ok(())
	}

	/// Burn Dexer (`SDEX` in Setheum or `HALAL` in Neom). `dexer` here just referring to the DEX token balance.
	/// TODO: update to `T::GetDexCurrencyId::get()` which is any `SettinDex` coin.
	fn burn_dexer(who: &T::AccountId, dexer: Self::Balance) -> DispatchResult {
		T::Currency::withdraw(T::GetStableCurrencyId::get(), who, dexer)
	}

	/// TODO: update to `currency_id` which is either `SETT` or any `SettCurrency`.
	fn deposit_serplus(from: &T::AccountId, serplus: Self::Balance) -> DispatchResult {
		T::Currency::transfer(T::GetStableCurrencyId::get(), from, &Self::account_id(), serplus)
	}

	/// TODO: update to `T::GetSetterCurrencyId::get()` which is `SETT`. Rename `reserve` to `setter`.
	fn deposit_reserve(from: &T::AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult {
		T::Currency::transfer(currency_id, from, &Self::account_id(), amount)
	}

	/// TODO: update to `T::GetSetterCurrencyId::get()` which is `SETT`. Rename `reserve` to `setter`.
	fn withdraw_reserve(to: &T::AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult {
		T::Currency::transfer(currency_id, &Self::account_id(), to, amount)
	}
}

impl<T: Config> SerpTreasuryExtended<T::AccountId> for Pallet<T> {
	/// Swap exact amount of reserve in auction to stable,
	/// return actual target stable amount
	fn swap_exact_reserve_in_auction_to_stable(
		currency_id: CurrencyId,
		supply_amount: Balance,
		min_target_amount: Balance,
		price_impact_limit: Option<Ratio>,
	) -> sp_std::result::Result<Balance, DispatchError> {
		ensure!(
			Self::total_reserves(currency_id) >= supply_amount
				&& T::SerpAuctionHandler::get_total_reserve_in_auction(currency_id) >= supply_amount,
			Error::<T>::ReserveNotEnough,
		);

		T::DEX::swap_with_exact_supply(
			&Self::account_id(),
			&[currency_id, T::GetStableCurrencyId::get()],
			supply_amount,
			min_target_amount,
			price_impact_limit,
		)
	}

	/// swap reserve which not in auction to get exact stable,
	/// return actual supply reserve amount
	fn swap_reserve_not_in_auction_with_exact_stable(
		currency_id: CurrencyId,
		target_amount: Balance,
		max_supply_amount: Balance,
		price_impact_limit: Option<Ratio>,
	) -> sp_std::result::Result<Balance, DispatchError> {
		ensure!(
			Self::total_reserves_not_in_auction(currency_id) >= max_supply_amount,
			Error::<T>::ReserveNotEnough,
		);

		T::DEX::swap_with_exact_target(
			&Self::account_id(),
			&[currency_id, T::GetStableCurrencyId::get()],
			target_amount,
			max_supply_amount,
			price_impact_limit,
		)
	}

	fn create_setter_auctions(
		currency_id: CurrencyId,
		amount: Balance,
		target: Balance,
		refund_receiver: T::AccountId,
		splited: bool,
	) -> DispatchResult {
		ensure!(
			Self::total_reserves_not_in_auction(currency_id) >= amount,
			Error::<T>::ReserveNotEnough,
		);

		let mut unhandled_reserve_amount = amount;
		let mut unhandled_target = target;
		let expected_setter_auction_size = Self::expected_setter_auction_size(currency_id);
		let max_auctions_count: Balance = T::MaxAuctionsCount::get().into();
		let lots_count = if !splited
			|| max_auctions_count.is_zero()
			|| expected_setter_auction_size.is_zero()
			|| amount <= expected_setter_auction_size
		{
			One::one()
		} else {
			let mut count = amount
				.checked_div(expected_setter_auction_size)
				.expect("setter auction maximum size is not zero; qed");

			let remainder = amount
				.checked_rem(expected_setter_auction_size)
				.expect("setter auction maximum size is not zero; qed");
			if !remainder.is_zero() {
				count = count.saturating_add(One::one());
			}
			sp_std::cmp::min(count, max_auctions_count)
		};
		let average_amount_per_lot = amount.checked_div(lots_count).expect("lots count is at least 1; qed");
		let average_target_per_lot = target.checked_div(lots_count).expect("lots count is at least 1; qed");
		let mut created_lots: Balance = Zero::zero();

		while !unhandled_reserve_amount.is_zero() {
			created_lots = created_lots.saturating_add(One::one());
			let (lot_reserve_amount, lot_target) = if created_lots == lots_count {
				// the last lot may be have some remnant than average
				(unhandled_reserve_amount, unhandled_target)
			} else {
				(average_amount_per_lot, average_target_per_lot)
			};

			T::SerpAuctionHandler::new_setter_auction(
				&refund_receiver,
				currency_id,
				lot_reserve_amount,
				lot_target,
			)?;

			unhandled_reserve_amount = unhandled_reserve_amount.saturating_sub(lot_reserve_amount);
			unhandled_target = unhandled_target.saturating_sub(lot_target);
		}
		Ok(())
	}
}

#[cfg(feature = "std")]
impl GenesisConfig {
	/// Direct implementation of `GenesisBuild::build_storage`.
	///
	/// Kept in order not to break dependency.
	pub fn build_storage<T: Config>(&self) -> Result<sp_runtime::Storage, String> {
		<Self as GenesisBuild<T>>::build_storage(self)
	}

	/// Direct implementation of `GenesisBuild::assimilate_storage`.
	///
	/// Kept in order not to break dependency.
	pub fn assimilate_storage<T: Config>(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
		<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
	}
}
