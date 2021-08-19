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

#![allow(clippy::from_over_into)] 

use crate::{evm::EvmAddress, *};
use bstringify::bstringify;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{
	convert::{Into, TryFrom},
	prelude::*,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

macro_rules! create_currency_id {
    ($(#[$meta:meta])*
	$vis:vis enum TokenSymbol {
        $($(#[$vmeta:meta])* $symbol:ident($name:expr, $deci:literal) = $val:literal,)*
    }) => {
		$(#[$meta])*
		$vis enum TokenSymbol {
			$($(#[$vmeta])* $symbol = $val,)*
		}

		impl TryFrom<u8> for TokenSymbol {
			type Error = ();

			fn try_from(v: u8) -> Result<Self, Self::Error> {
				match v {
					$($val => Ok(TokenSymbol::$symbol),)*
					_ => Err(()),
				}
			}
		}

		impl Into<u8> for TokenSymbol {
			fn into(self) -> u8 {
				match self {
					$(TokenSymbol::$symbol => ($val),)*
				}
			}
		}

		impl TryFrom<Vec<u8>> for CurrencyId {
			type Error = ();
			fn try_from(v: Vec<u8>) -> Result<CurrencyId, ()> {
				match v.as_slice() {
					$(bstringify!($symbol) => Ok(CurrencyId::Token(TokenSymbol::$symbol)),)*
					_ => Err(()),
				}
			}
		}

		impl TokenInfo for CurrencyId {
			fn currency_id(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($val),)*
					_ => None,
				}
			}
			fn name(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($name),)*
					_ => None,
				}
			}
			fn symbol(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some(stringify!($symbol)),)*
					_ => None,
				}
			}
			fn decimals(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($deci),)*
					_ => None,
				}
			}
		}

		$(pub const $symbol: CurrencyId = CurrencyId::Token(TokenSymbol::$symbol);)*

		impl TokenSymbol {
			pub fn get_info() -> Vec<(&'static str, u32)> {
				vec![
					$((stringify!($symbol), $deci),)*
				]
			}
		}

		#[test]
		#[ignore]
		fn generate_token_resources() {
			use crate::TokenSymbol::*;

			#[allow(non_snake_case)]
			#[derive(Serialize, Deserialize)]
			struct Token {
				symbol: String,
				address: EvmAddress,
			}

			let mut tokens = vec![
				$(
					Token {
						symbol: stringify!($symbol).to_string(),
						address: EvmAddress::try_from(CurrencyId::Token(TokenSymbol::$symbol)).unwrap(),
					},
				)*
			];

			let mut lp_tokens = vec![
				// DNAR paired LPs
				Token {
					symbol: "LP_DRAM_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(DRAM), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_SETUSD_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETUSD), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_SETEUR_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETEUR), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_SETGBP_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETGBP), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_SETCHF_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETCHF), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_SETSAR_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETSAR), DexShare::Token(DNAR))).unwrap(),
				},
				Token {
					symbol: "LP_RENBTC_DNAR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(RENBTC), DexShare::Token(DNAR))).unwrap(),
				},
				// SETR paired LPs
				Token {
					symbol: "LP_DNAR_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(DNAR), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_DRAM_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(DRAM), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_SETUSD_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETUSD), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_SETEUR_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETEUR), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_SETGBP_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETGBP), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_SETCHF_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETCHF), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_SETSAR_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETSAR), DexShare::Token(SETR))).unwrap(),
				},
				Token {
					symbol: "LP_RENBTC_SETR".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(RENBTC), DexShare::Token(SETR))).unwrap(),
				},
				// SETUSD paired LPs
				Token {
					symbol: "LP_DRAM_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(DRAM), DexShare::Token(SETUSD))).unwrap(),
				},
				Token {
					symbol: "LP_SETEUR_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETEUR), DexShare::Token(SETUSD))).unwrap(),
				},
				Token {
					symbol: "LP_SETGBP_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETGBP), DexShare::Token(SETUSD))).unwrap(),
				},
				Token {
					symbol: "LP_SETCHF_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETCHF), DexShare::Token(SETUSD))).unwrap(),
				},
				Token {
					symbol: "LP_SETSAR_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(SETSAR), DexShare::Token(SETUSD))).unwrap(),
				},
				Token {
					symbol: "LP_RENBTC_SETUSD".to_string(),
					address: EvmAddress::try_from(CurrencyId::DexShare(DexShare::Token(RENBTC), DexShare::Token(SETUSD))).unwrap(),
				},
			];
			tokens.append(&mut lp_tokens);

			frame_support::assert_ok!(std::fs::write("../predeploy-contracts/resources/tokens.json", serde_json::to_string_pretty(&tokens).unwrap()));
		}
    }
}

create_currency_id! {
	// Represent a Token symbol with 8 bit
	// Bit 8 : 0 for Setheum Network
	// Bit 7 : Reserved
	// Bit 6 - 1 : The token ID
	#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		/// Setheum Network
		DNAR("Setheum Dinar", 12) = 0,
		DRAM("Setheum Dirham", 12) = 1,
		SETR("Setter", 12) = 2,
		// SettCurrencies
		SETUSD("SetDollar", 12) = 3,
		SETEUR("SetEuro", 12) = 4,
		SETGBP("SetPound", 12) = 5,
		SETCHF("SetFranc", 12) = 6,
 		SETSAR("SetRiyal", 12) = 7,
		// Foreign Currencies
		RENBTC("Ren Bitcoin", 8) = 8,

		// TODO: Remove these fiat references once the `serp-ocw` module has been implemented
		/// Fiat Currencies as Pegs - only for price feed
		USD("Fiat US Dollar", 12) = 181,
		EUR("Fiat Euro", 12) =182,
		GBP("Fiat Pound Sterling", 12) = 184,
		CHF("Fiat Swiss Franc", 12) = 187,
 		SAR("Fiat Saudi Riyal", 12) = 190,
		KWD("Fiat Kuwaiti Dinar", 12) = 191,		
		JOD("Fiat Jordanian Dinar", 12) = 192,		
		BHD("Fiat Bahraini Dirham", 12) = 193,		
		KYD("Fiat Cayman Islands Dollar", 12) = 194,
		OMR("Fiat Omani Riyal", 12) = 195,			
		GIP("Fiat Gibraltar Pound", 12) = 196,		
	}
}

pub trait TokenInfo {
	fn currency_id(&self) -> Option<u8>;
	fn name(&self) -> Option<&str>;
	fn symbol(&self) -> Option<&str>;
	fn decimals(&self) -> Option<u8>;
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DexShare {
	Token(TokenSymbol),
	Erc20(EvmAddress),
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
	Erc20(EvmAddress),
	ChainSafe(chainbridge::ResourceId),
}

impl CurrencyId {
	pub fn is_token_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Token(_))
	}

	pub fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, CurrencyId::DexShare(_, _))
	}

	pub fn is_erc20_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Erc20(_))
	}

	pub fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			CurrencyId::DexShare(dex_share_0, dex_share_1) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();
				let currency_id_1: CurrencyId = (*dex_share_1).into();
				Some((currency_id_0, currency_id_1))
			}
			_ => None,
		}
	}

	pub fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
		let dex_share_0 = match currency_id_0 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			CurrencyId::Erc20(address) => DexShare::Erc20(address),
			_ => return None,
		};
		let dex_share_1 = match currency_id_1 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			CurrencyId::Erc20(address) => DexShare::Erc20(address),
			_ => return None,
		};
		Some(CurrencyId::DexShare(dex_share_0, dex_share_1))
	}
}

impl From<DexShare> for u32 {
	fn from(val: DexShare) -> u32 {
		let mut bytes = [0u8; 4];
		match val {
			DexShare::Token(token) => {
				bytes[3] = token.into();
			}
			DexShare::Erc20(address) => {
				let is_zero = |&&d: &&u8| -> bool { d == 0 };
				let leading_zeros = address.as_bytes().iter().take_while(is_zero).count();
				let index = if leading_zeros > 16 { 16 } else { leading_zeros };
				bytes[..].copy_from_slice(&address[index..index + 4][..]);
			}
		}
		u32::from_be_bytes(bytes)
	}
}

/// Generate the EvmAddress from CurrencyId so that evm contracts can call the erc20 contract.
impl TryFrom<CurrencyId> for EvmAddress {
	type Error = ();

	fn try_from(val: CurrencyId) -> Result<Self, Self::Error> {
		match val {
			CurrencyId::Token(_) => Ok(EvmAddress::from_low_u64_be(
				MIRRORED_TOKENS_ADDRESS_START | u64::from(val.currency_id().unwrap()),
			)),
			CurrencyId::DexShare(token_symbol_0, token_symbol_1) => {
				let symbol_0 = match token_symbol_0 {
					DexShare::Token(token) => CurrencyId::Token(token).currency_id().ok_or(()),
					DexShare::Erc20(_) => Err(()),
				}?;
				let symbol_1 = match token_symbol_1 {
					DexShare::Token(token) => CurrencyId::Token(token).currency_id().ok_or(()),
					DexShare::Erc20(_) => Err(()),
				}?;

				let mut prefix = EvmAddress::default();
				prefix[0..H160_PREFIX_DEXSHARE.len()].copy_from_slice(&H160_PREFIX_DEXSHARE);
				Ok(prefix | EvmAddress::from_low_u64_be(u64::from(symbol_0) << 32 | u64::from(symbol_1)))
			}
			CurrencyId::Erc20(address) => Ok(address),
			CurrencyId::ChainSafe(_) => Err(()),
		}
	}
}

impl Into<CurrencyId> for DexShare {
	fn into(self) -> CurrencyId {
		match self {
			DexShare::Token(token) => CurrencyId::Token(token),
			DexShare::Erc20(address) => CurrencyId::Erc20(address),
		}
	}
}
