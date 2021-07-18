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

//! Mocks for the prices module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types};
use frame_system::EnsureSignedBy;
use orml_traits::{parameter_type_with_key, DataFeeder};
use primitives::{Amount, TokenSymbol};
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{IdentityLookup, One as OneT, Zero},
	DispatchError, FixedPointNumber,
};
use support::{mocks::MockCurrencyIdMapping, ExchangeRate, Ratio};

pub type AccountId = u128;
pub type BlockNumber = u64;

mod serp_prices {
	pub use super::super::*;
}

// Currencies constants - CurrencyId/TokenSymbol
pub const DNAR: CurrencyId = CurrencyId::Token(TokenSymbol::DNAR);
pub const DRAM: CurrencyId = CurrencyId::Token(TokenSymbol::DRAM);
pub const SETT: CurrencyId = CurrencyId::Token(TokenSymbol::SETT);
pub const AEDJ: CurrencyId = CurrencyId::Token(TokenSymbol::AEDJ);
pub const AUDJ: CurrencyId = CurrencyId::Token(TokenSymbol::AUDJ);
pub const BRLJ: CurrencyId = CurrencyId::Token(TokenSymbol::BRLJ);
pub const CADJ: CurrencyId = CurrencyId::Token(TokenSymbol::CADJ);
pub const CHFJ: CurrencyId = CurrencyId::Token(TokenSymbol::CHFJ);
pub const CLPJ: CurrencyId = CurrencyId::Token(TokenSymbol::CLPJ);
pub const CNYJ: CurrencyId = CurrencyId::Token(TokenSymbol::CNYJ);
pub const COPJ: CurrencyId = CurrencyId::Token(TokenSymbol::COPJ);
pub const EURJ: CurrencyId = CurrencyId::Token(TokenSymbol::EURJ);
pub const GBPJ: CurrencyId = CurrencyId::Token(TokenSymbol::GBPJ);
pub const HKDJ: CurrencyId = CurrencyId::Token(TokenSymbol::HKDJ);
pub const HUFJ: CurrencyId = CurrencyId::Token(TokenSymbol::HUFJ);
pub const IDRJ: CurrencyId = CurrencyId::Token(TokenSymbol::IDRJ);
pub const JPYJ: CurrencyId = CurrencyId::Token(TokenSymbol::JPYJ);
pub const KESJ: CurrencyId = CurrencyId::Token(TokenSymbol::KESJ);
pub const KRWJ: CurrencyId = CurrencyId::Token(TokenSymbol::KRWJ);
pub const KZTJ: CurrencyId = CurrencyId::Token(TokenSymbol::KZTJ);
pub const MXNJ: CurrencyId = CurrencyId::Token(TokenSymbol::MXNJ);
pub const MYRJ: CurrencyId = CurrencyId::Token(TokenSymbol::MYRJ);
pub const NGNJ: CurrencyId = CurrencyId::Token(TokenSymbol::NGNJ);
pub const NOKJ: CurrencyId = CurrencyId::Token(TokenSymbol::NOKJ);
pub const NZDJ: CurrencyId = CurrencyId::Token(TokenSymbol::NZDJ);
pub const PENJ: CurrencyId = CurrencyId::Token(TokenSymbol::PENJ);
pub const PHPJ: CurrencyId = CurrencyId::Token(TokenSymbol::PHPJ);
pub const PKRJ: CurrencyId = CurrencyId::Token(TokenSymbol::PKRJ);
pub const PLNJ: CurrencyId = CurrencyId::Token(TokenSymbol::PLNJ);
pub const QARJ: CurrencyId = CurrencyId::Token(TokenSymbol::QARJ);
pub const RONJ: CurrencyId = CurrencyId::Token(TokenSymbol::RONJ);
pub const RUBJ: CurrencyId = CurrencyId::Token(TokenSymbol::RUBJ);
pub const SARJ: CurrencyId = CurrencyId::Token(TokenSymbol::SARJ);
pub const SEKJ: CurrencyId = CurrencyId::Token(TokenSymbol::SEKJ);
pub const SGDJ: CurrencyId = CurrencyId::Token(TokenSymbol::SGDJ);
pub const THBJ: CurrencyId = CurrencyId::Token(TokenSymbol::THBJ);
pub const TRYJ: CurrencyId = CurrencyId::Token(TokenSymbol::TRYJ);
pub const TWDJ: CurrencyId = CurrencyId::Token(TokenSymbol::TWDJ);
pub const TZSJ: CurrencyId = CurrencyId::Token(TokenSymbol::TZSJ);
pub const USDJ: CurrencyId = CurrencyId::Token(TokenSymbol::USDJ);
pub const ZARJ: CurrencyId = CurrencyId::Token(TokenSymbol::ZARJ);

// LP tokens constants - CurrencyId/TokenSymbol : Dex Shares
pub const LP_CHFJ_USDJ: CurrencyId =
CurrencyId::DexShare(DexShare::Token(TokenSymbol::CHFJ), DexShare::Token(TokenSymbol::USDJ));
pub const LP_USDJ_DNAR: CurrencyId =
CurrencyId::DexShare(DexShare::Token(TokenSymbol::USDJ), DexShare::Token(TokenSymbol::DNAR));

// Currencies constants - FiatCurrencyIds (CurrencyId/TokenSymbol)
pub const AED: CurrencyId = CurrencyId::Token(TokenSymbol::AED);
pub const AUD: CurrencyId = CurrencyId::Token(TokenSymbol::AUD);
pub const BRL: CurrencyId = CurrencyId::Token(TokenSymbol::BRL);
pub const CAD: CurrencyId = CurrencyId::Token(TokenSymbol::CAD);
pub const CHF: CurrencyId = CurrencyId::Token(TokenSymbol::CHF);
pub const CLP: CurrencyId = CurrencyId::Token(TokenSymbol::CLP);
pub const CNY: CurrencyId = CurrencyId::Token(TokenSymbol::CNY);
pub const COP: CurrencyId = CurrencyId::Token(TokenSymbol::COP);
pub const EUR: CurrencyId = CurrencyId::Token(TokenSymbol::EUR);
pub const GBP: CurrencyId = CurrencyId::Token(TokenSymbol::GBP);
pub const HKD: CurrencyId = CurrencyId::Token(TokenSymbol::HKD);
pub const HUF: CurrencyId = CurrencyId::Token(TokenSymbol::HUF);
pub const IDR: CurrencyId = CurrencyId::Token(TokenSymbol::IDR);
pub const JPY: CurrencyId = CurrencyId::Token(TokenSymbol::JPY);
pub const KES: CurrencyId = CurrencyId::Token(TokenSymbol::KES);
pub const KRW: CurrencyId = CurrencyId::Token(TokenSymbol::KRW);
pub const KZT: CurrencyId = CurrencyId::Token(TokenSymbol::KZT);
pub const MXN: CurrencyId = CurrencyId::Token(TokenSymbol::MXN);
pub const MYR: CurrencyId = CurrencyId::Token(TokenSymbol::MYR);
pub const NGN: CurrencyId = CurrencyId::Token(TokenSymbol::NGN);
pub const NOK: CurrencyId = CurrencyId::Token(TokenSymbol::NOK);
pub const NZD: CurrencyId = CurrencyId::Token(TokenSymbol::NZD);
pub const PEN: CurrencyId = CurrencyId::Token(TokenSymbol::PEN);
pub const PHP: CurrencyId = CurrencyId::Token(TokenSymbol::PHP);
pub const PKR: CurrencyId = CurrencyId::Token(TokenSymbol::PKR);
pub const PLN: CurrencyId = CurrencyId::Token(TokenSymbol::PLN);
pub const QAR: CurrencyId = CurrencyId::Token(TokenSymbol::QAR);
pub const RON: CurrencyId = CurrencyId::Token(TokenSymbol::RON);
pub const RUB: CurrencyId = CurrencyId::Token(TokenSymbol::RUB);
pub const SAR: CurrencyId = CurrencyId::Token(TokenSymbol::SAR);
pub const SEK: CurrencyId = CurrencyId::Token(TokenSymbol::SEK);
pub const SGD: CurrencyId = CurrencyId::Token(TokenSymbol::SGD);
pub const THB: CurrencyId = CurrencyId::Token(TokenSymbol::THB);
pub const TRY: CurrencyId = CurrencyId::Token(TokenSymbol::TRY);
pub const TWD: CurrencyId = CurrencyId::Token(TokenSymbol::TWD);
pub const TZS: CurrencyId = CurrencyId::Token(TokenSymbol::TZS);
pub const USD: CurrencyId = CurrencyId::Token(TokenSymbol::USD);
pub const ZAR: CurrencyId = CurrencyId::Token(TokenSymbol::ZAR);
pub const KWD: CurrencyId = CurrencyId::Token(TokenSymbol::KWD);
pub const JOD: CurrencyId = CurrencyId::Token(TokenSymbol::JOD);
pub const BHD: CurrencyId = CurrencyId::Token(TokenSymbol::BHD);
pub const KYD: CurrencyId = CurrencyId::Token(TokenSymbol::KYD);
pub const OMR: CurrencyId = CurrencyId::Token(TokenSymbol::OMR);
pub const GIP: CurrencyId = CurrencyId::Token(TokenSymbol::GIP);

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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

pub struct MockDataProvider;
impl DataProvider<CurrencyId, Price> for MockDataProvider {
	fn get(currency_id: &CurrencyId) -> Option<Price> {
		match *currency_id {
			USDJ => Some(Price::saturating_from_rational(99, 100)),
			CHFJ => Some(Price::saturating_from_integer(50000)),
			DNAR => Some(Price::saturating_from_integer(100)),
			DNAR => Some(Price::zero()),
			_ => None,
		}
	}
}

impl DataFeeder<CurrencyId, Price, AccountId> for MockDataProvider {
	fn feed_value(_: AccountId, _: CurrencyId, _: Price) -> sp_runtime::DispatchResult {
		Ok(())
	}
}

pub struct MockDex;
impl DEXManager<AccountId, CurrencyId, Balance> for MockDex {
	fn get_liquidity_pool(currency_id_a: CurrencyId, currency_id_b: CurrencyId) -> (Balance, Balance) {
		match (currency_id_a, currency_id_b) {
			(USDJ, DNAR) => (10000, 200),
			_ => (0, 0),
		}
	}

	fn get_liquidity_token_address(_currency_id_a: CurrencyId, _currency_id_b: CurrencyId) -> Option<H160> {
		unimplemented!()
	}

	fn get_swap_target_amount(
		_path: &[CurrencyId],
		_supply_amount: Balance,
		_price_impact_limit: Option<Ratio>,
	) -> Option<Balance> {
		unimplemented!()
	}

	fn get_swap_supply_amount(
		_path: &[CurrencyId],
		_target_amount: Balance,
		_price_impact_limit: Option<Ratio>,
	) -> Option<Balance> {
		unimplemented!()
	}

	fn swap_with_exact_supply(
		_who: &AccountId,
		_path: &[CurrencyId],
		_supply_amount: Balance,
		_min_target_amount: Balance,
		_price_impact_limit: Option<Ratio>,
	) -> sp_std::result::Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn swap_with_exact_target(
		_who: &AccountId,
		_path: &[CurrencyId],
		_target_amount: Balance,
		_max_supply_amount: Balance,
		_price_impact_limit: Option<Ratio>,
	) -> sp_std::result::Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn add_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_max_amount_a: Balance,
		_max_amount_b: Balance,
		_min_share_increment: Balance,
		_deposit_increment_share: bool,
	) -> DispatchResult {
		unimplemented!()
	}

	fn remove_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_remove_share: Balance,
		_min_withdrawn_a: Balance,
		_min_withdrawn_b: Balance,
		_by_withdraw: bool,
	) -> DispatchResult {
		unimplemented!()
	}
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

parameter_type_with_key! {
	pub PegCurrencyIds: |_currency_id: CurrencyId| -> CurrencyId {
		match currency_id {
			&AEDJ => &AED,
			&AUDJ => &AUD,
			&BRLJ => &BRL,
			&CADJ => &CAD,
			&CHFJ => &CHF,
			&CLPJ => &CLP,
			&CNYJ => &CNY,
			&COPJ => &COP,
			&EURJ => &EUR,
			&GBPJ => &GBP,
			&HKDJ => &HKD,
			&HUFJ => &HUF,
			&IDRJ => &IDR,
			&JPYJ => &JPY,
			&KESJ => &KES,
			&KRWJ => &KRW,
			&KZTJ => &KZT,
			&MXNJ => &MXN,
			&MYRJ => &MYR,
			&NGNJ => &NGN,
			&NOKJ => &NOK,
			&NZDJ => &NZD,
			&PENJ => &PEN,
			&PHPJ => &PHP,
			&PKRJ => &PKR,
			&PLNJ => &PLN,
			&QARJ => &QAR,
			&RONJ => &RON,
			&RUBJ => &RUB,
			&SARJ => &SAR,
			&SEKJ => &SEK,
			&SGDJ => &SGD,
			&THBJ => &THB,
			&TRYJ => &TRY,
			&TWDJ => &TWD,
			&TZSJ => &TZS,
			&USDJ => &USD,
			&ZARJ => &ZAR,
			_ => None,
		}
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

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub const GetSetterCurrencyId: CurrencyId = SETT; // Setter currency ticker is SETT.
	pub const GetSettUSDCurrencyId: CurrencyId = USDJ; // SettUSD currency ticker is USDJ.
	pub const GetFiatUSDCurrencyId: CurrencyId = USD; // The USD Fiat currency denomination.
	pub FiatUsdFixedPrice: Price = Price::one(); // Fixed 1 USD Fiat denomination for pricing.

	pub const GetSetterPegOneCurrencyId: CurrencyId = GBP; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegTwoCurrencyId: CurrencyId = EUR; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegThreeCurrencyId: CurrencyId = KWD; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegFourCurrencyId: CurrencyId = JOD; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegFiveCurrencyId: CurrencyId = BHD; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegSixCurrencyId: CurrencyId = KYD; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegSevenCurrencyId: CurrencyId = OMR; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegEightCurrencyId: CurrencyId = CHF; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegNineCurrencyId: CurrencyId = GIP; // Fiat pegs of the Setter (SETT).
	pub const GetSetterPegTenCurrencyId: CurrencyId = USD; // Fiat pegs of the Setter (SETT).
	
	pub StableCurrencyIds: Vec<CurrencyId> = vec![
		SETT, AEDJ, AUDJ, BRLJ, CADJ, CHFJ, CLPJ, CNYJ, COPJ, EURJ,
		GBPJ, HKDJ, HUFJ, IDRJ, JPYJ, KESJ, KRWJ, KZTJ, MXNJ, MYRJ,
		NGNJ, NOKJ, NZDJ, PENJ, PHPJ, PKRJ, PLNJ, QARJ, RONJ, RUBJ,
		SARJ, SEKJ, SGDJ, THBJ, TRYJ, TWDJ, TZSJ, USDJ, ZARJ,
	];
	pub FiatCurrencyIds: Vec<CurrencyId> = vec![
		AED, AUD, BRL, CAD, CHF, CLP, CNY, COP, EUR, GBP,
		HKD, HUF, IDR, JPY, KES, KRW, KZT, MXN, MYR, NGN,
		NOK, NZD, PEN, PHP, PKR, PLN, QAR, RON, RUB, SAR,
		SEK, SGD, THB, TRY, TWD, TZS, USD, ZAR, KWD, JOD,
		BHD, KYD, OMR, GIP
	];
}

impl Config for Runtime {
	type Event = Event;
	type Source = MockDataProvider;
	type GetSetterCurrencyId = GetSetterCurrencyId;
	type GetSettUSDCurrencyId = GetSettUSDCurrencyId;
	type GetFiatUSDCurrencyId = GetFiatUSDCurrencyId;
	type FiatUsdFixedPrice = FiatUsdFixedPrice;
	type GetSetterPegOneCurrencyId = GetSetterPegOneCurrencyId;
	type GetSetterPegTwoCurrencyId = GetSetterPegTwoCurrencyId;
	type GetSetterPegThreeCurrencyId = GetSetterPegThreeCurrencyId;
	type GetSetterPegFourCurrencyId = GetSetterPegFourCurrencyId;
	type GetSetterPegFiveCurrencyId = GetSetterPegFiveCurrencyId;
	type GetSetterPegSixCurrencyId = GetSetterPegSixCurrencyId;
	type GetSetterPegSevenCurrencyId = GetSetterPegSevenCurrencyId;
	type GetSetterPegEightCurrencyId = GetSetterPegEightCurrencyId;
	type GetSetterPegNineCurrencyId = GetSetterPegNineCurrencyId;
	type GetSetterPegTenCurrencyId = GetSetterPegTenCurrencyId;
	type StableCurrencyIds = StableCurrencyIds;
	type PegCurrencyIds = PegCurrencyIds;
	type FiatCurrencyIds = FiatCurrencyIds;
	type LockOrigin = EnsureSignedBy<One, AccountId>;
	type DEX = MockDEX;
	type Currency = Tokens;
	type CurrencyIdMapping = MockCurrencyIdMapping;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		SerpPrices: prices::{Pallet, Storage, Call, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Storage, Event<T>},
	}
);

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		t.into()
	}
}
