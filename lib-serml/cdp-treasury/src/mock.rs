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

//! Mocks for the cdp treasury module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types};
use frame_system::{EnsureOneOf, EnsureRoot, EnsureSignedBy};
use orml_traits::parameter_type_with_key;
use primitives::{TokenSymbol, TradingPair};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};
use sp_std::cell::RefCell;

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type Amount = i64;
pub type AuctionId = u32;

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;
pub const SETHEUM: CurrencyId = CurrencyId::Token(TokenSymbol::SETHEUM);
pub const SETUSD: CurrencyId = CurrencyId::Token(TokenSymbol::SETUSD);
pub const BTC: CurrencyId = CurrencyId::Token(TokenSymbol::RENBTC);
pub const DNAR: CurrencyId = CurrencyId::Token(TokenSymbol::DNAR);

mod cdp_treasury {
	pub use super::super::*;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SETHEUM;
}

impl orml_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const GetStableCurrencyId: CurrencyId = SETUSD;
	pub const GetExchangeFee: (u32, u32) = (0, 100);
	pub const TradingPathLimit: u32 = 3;
	pub EnabledTradingPairs: Vec<TradingPair> = vec![
		TradingPair::from_currency_ids(SETUSD, BTC).unwrap(),
		TradingPair::from_currency_ids(SETUSD, DNAR).unwrap(),
		TradingPair::from_currency_ids(BTC, DNAR).unwrap(),
	];
	pub const DEXModuleId: ModuleId = ModuleId(*b"set/dexm");
}

impl module_dex::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = TradingPathLimit;
	type ModuleId = DEXModuleId;
	type CurrencyIdMapping = ();
	type DEXIncentives = ();
	type WeightInfo = ();
	type ListingOrigin = EnsureSignedBy<One, AccountId>;
}

thread_local! {
	pub static TOTAL_COLLATERAL_AUCTION: RefCell<u32> = RefCell::new(0);
	pub static TOTAL_COLLATERAL_IN_AUCTION: RefCell<Balance> = RefCell::new(0);
}

pub struct MockAuctionManager;
impl AuctionManager<AccountId> for MockAuctionManager {
	type CurrencyId = CurrencyId;
	type Balance = Balance;
	type AuctionId = AuctionId;

	fn new_collateral_auction(
		_refund_recipient: &AccountId,
		_currency_id: Self::CurrencyId,
		amount: Self::Balance,
		_target: Self::Balance,
	) -> DispatchResult {
		TOTAL_COLLATERAL_AUCTION.with(|v| *v.borrow_mut() += 1);
		TOTAL_COLLATERAL_IN_AUCTION.with(|v| *v.borrow_mut() += amount);
		Ok(())
	}

	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		unimplemented!()
	}

	fn get_total_collateral_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		TOTAL_COLLATERAL_IN_AUCTION.with(|v| *v.borrow_mut())
	}

	fn get_total_target_in_auction() -> Self::Balance {
		unimplemented!()
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
	pub const MaxAuctionsCount: u32 = 5;
}

parameter_types! {
	pub const CDPTreasuryModuleId: ModuleId = ModuleId(*b"set/cdpt");
	pub const TreasuryAccount: AccountId = 10;
}

thread_local! {
	static IS_SHUTDOWN: RefCell<bool> = RefCell::new(false);
}

impl Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = MockAuctionManager;
	type UpdateOrigin = EnsureOneOf<AccountId, EnsureRoot<AccountId>, EnsureSignedBy<One, AccountId>>;
	type DEX = DEXModule;
	type MaxAuctionsCount = MaxAuctionsCount;
	type ModuleId = CDPTreasuryModuleId;
	type TreasuryAccount = TreasuryAccount;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		CDPTreasuryModule: cdp_treasury::{Pallet, Storage, Call, Config, Event<T>},
		Currencies: orml_currencies::{Pallet, Call, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		PalletBalances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		DEXModule: module_dex::{Pallet, Storage, Call, Event<T>, Config<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balances: vec![
				(ALICE, DNAR, 1000),
				(ALICE, SETUSD, 1000),
				(ALICE, BTC, 1000),
				(BOB, DNAR, 1000),
				(BOB, SETUSD, 1000),
				(BOB, BTC, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			balances: self.balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		module_dex::GenesisConfig::<Runtime> {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: EnabledTradingPairs::get(),
			initial_added_liquidity_pools: vec![],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
