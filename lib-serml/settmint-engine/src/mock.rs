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

//! Mocks for the settmint engine module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, ord_parameter_types, parameter_types, PalletId};
use frame_system::EnsureSignedBy;
use orml_traits::parameter_type_with_key;
use primitives::{TokenSymbol, TradingPair};
use sp_core::H256;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::IdentityLookup,
};
use sp_std::cell::RefCell;

pub type AccountId = u128;
pub type BlockNumber = u64;
pub type AuctionId = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;

// Currencies constants - CurrencyId/TokenSymbol
pub const DNAR: CurrencyId = CurrencyId::Token(TokenSymbol::DNAR);
pub const SDEX: CurrencyId = CurrencyId::Token(TokenSymbol::SDEX); //  SettinDex
pub const SETT: CurrencyId = CurrencyId::Token(TokenSymbol::SETT); // Setter   -  The Defacto stablecoin & settmint reserve asset
pub const USDJ: CurrencyId = CurrencyId::Token(TokenSymbol::USDJ); // Setheum USD (US Dollar stablecoin)
pub const GBPJ: CurrencyId = CurrencyId::Token(TokenSymbol::GBPJ); // Setheum GBP (Pound Sterling stablecoin)
pub const EURJ: CurrencyId = CurrencyId::Token(TokenSymbol::EURJ); // Setheum EUR (Euro stablecoin)
pub const KWDJ: CurrencyId = CurrencyId::Token(TokenSymbol::KWDJ); // Setheum KWD (Kuwaiti Dinar stablecoin)
pub const JODJ: CurrencyId = CurrencyId::Token(TokenSymbol::JODJ); // Setheum JOD (Jordanian Dinar stablecoin)
pub const BHDJ: CurrencyId = CurrencyId::Token(TokenSymbol::BHDJ); // Setheum BHD (Bahraini Dirham stablecoin)
pub const KYDJ: CurrencyId = CurrencyId::Token(TokenSymbol::KYDJ); // Setheum KYD (Cayman Islands Dollar stablecoin)
pub const OMRJ: CurrencyId = CurrencyId::Token(TokenSymbol::OMRJ); // Setheum OMR (Omani Riyal stablecoin)
pub const CHFJ: CurrencyId = CurrencyId::Token(TokenSymbol::CHFJ); // Setheum CHF (Swiss Franc stablecoin)
pub const GIPJ: CurrencyId = CurrencyId::Token(TokenSymbol::GIPJ); // Setheum GIP (Gibraltar Pound stablecoin)

mod settmint_engine {
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
	type AccountStore = frame_system::Module<Runtime>;
	type MaxLocks = ();
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = DNAR;
}

impl orml_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const SettersPalletId: PalletId = PalletId(*b"set/setter");
}

impl setters::Config for Runtime {
	type Event = Event;
	type Convert = StandardExchangeRateConvertor<Runtime>;
	type Currency = Currencies;
	type StandardValidator = SettmintEngineModule;
	type SerpTreasury = SerpTreasuryModule;
	type PalletId = SettersPalletId;
	type OnUpdateSetter = ();
}

thread_local! {
	static RELATIVE_PRICE: RefCell<Option<Price>> = RefCell::new(Some(Price::one()));
}

pub struct MockPriceSource;
impl MockPriceSource {
	pub fn set_relative_price(price: Option<Price>) {
		RELATIVE_PRICE.with(|v| *v.borrow_mut() = price);
	}
}
impl PriceProvider<CurrencyId> for MockPriceSource {
	fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<Price> {
		match (base, quote) {
			(USDJ, SETT) => RELATIVE_PRICE.with(|v| *v.borrow_mut()),
			(SETT, USDJ) => RELATIVE_PRICE.with(|v| *v.borrow_mut()),
			_ => None,
		}
	}

	fn get_price(_currency_id: CurrencyId) -> Option<Price> {
		Some(Price::one())
	}

	fn lock_price(_currency_id: CurrencyId) {}

	fn unlock_price(_currency_id: CurrencyId) {}
}

pub struct MockSerpAuction;
impl SerpAuction<AccountId> for MockSerpAuction {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type AuctionId = AuctionId;

	fn new_setter_auction(
		_refund_recipient: &AccountId,
		_currency_id: Self::CurrencyId,
		_amount: Self::Balance,
		_target: Self::Balance,
	) -> DispatchResult {
		Ok(())
	}

	fn new_diamond_auction(_amount: Self::Balance, _fix: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn new_serplus_auction(_amount: Self::Balance) -> DispatchResult {
		Ok(())
	}

	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		Ok(())
	}

	fn get_total_standard_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Default::default()
	}

	fn get_total_setter_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		Default::default()
	}

	fn get_total_diamond_in_auction() -> Self::Balance {
		Default::default()
	}
}

parameter_types! {
	pub StableCurrencyIds: Vec<CurrencyId> = vec![
		SETT, // Setter   -  The Defacto stablecoin & settmint reserve asset
		USDJ, // Setheum USD (US Dollar stablecoin)
		GBPJ, // Setheum GBP (Pound Sterling stablecoin)
		EURJ, // Setheum EUR (Euro stablecoin)
		KWDJ, // Setheum KWD (Kuwaiti Dinar stablecoin)
		JODJ, // Setheum JOD (Jordanian Dinar stablecoin)
		BHDJ, // Setheum BHD (Bahraini Dirham stablecoin)
		KYDJ, // Setheum KYD (Cayman Islands Dollar stablecoin)
		OMRJ, // Setheum OMR (Omani Riyal stablecoin)
		CHFJ, // Setheum CHF (Swiss Franc stablecoin)
		GIPJ, // Setheum GIP (Gibraltar Pound stablecoin)
	];
	pub const GetSetterCurrencyId: CurrencyId = SETT;  // Setter  currency ticker is SETT
	pub const GetSettUSDCurrencyId: CurrencyId = USDJ; // SettUSD currency ticker is USDJ
	pub const GetSettGBPCurrencyId: CurrencyId = GBPJ; // SettGBP currency ticker is GBPJ
	pub const GetSettEURCurrencyId: CurrencyId = EURJ; // SettEUR currency ticker is EURJ
	pub const GetSettKWDCurrencyId: CurrencyId = KWDJ; // SettKWD currency ticker is KWDJ
	pub const GetSettJODCurrencyId: CurrencyId = JODJ; // SettJOD currency ticker is JODJ
	pub const GetSettBHDCurrencyId: CurrencyId = BHDJ; // SettBHD currency ticker is BHDJ
	pub const GetSettKYDCurrencyId: CurrencyId = KYDJ; // SettKYD currency ticker is KYDJ
	pub const GetSettOMRCurrencyId: CurrencyId = OMRJ; // SettOMR currency ticker is OMRJ
	pub const GetSettCHFCurrencyId: CurrencyId = CHFJ; // SettCHF currency ticker is CHFJ
	pub const GetSettGIPCurrencyId: CurrencyId = GIPJ; // SettGIP currency ticker is GIPJ
	pub const GetDexerCurrencyId: CurrencyId = SDEX; // SettinDEX currency ticker is SDEX

	pub const SerpTreasuryPalletId: PalletId = PalletId(*b"set/settmintt");
	pub SerpTesSchedule: BlockNumber = 60; // Triggers SERP-TES for serping after Every 60 blocks
	pub SerplusSerpupRatio: Rate = Rate::saturating_from_rational(1 : 10); // 10% of SerpUp to buy back & burn NativeCurrency.
	pub SettPaySerpupRatio: Rate = Rate::saturating_from_rational(6 : 10); // 60% of SerpUp to SettPay as Cashdrops.
	pub SetheumTreasurySerpupRatio: Rate = Rate::saturating_from_rational(1 : 10); // 10% of SerpUp to network Treasury.
	pub CharityFundSerpupRatio: Rate = Rate::saturating_from_rational(1 : 10); // 10% of SerpUp to Setheum Foundation's Charity Fund.
	pub SIFSerpupRatio: Rate = Rate::saturating_from_rational(1 : 10); // 10% of SerpUp to Setheum Investment Fund (SIF) (NIF in Neom).
}

impl serp_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type StableCurrencyIds = StableCurrencyIds;
	type GetSetterCurrencyId = GetSetterCurrencyId;
	type GetSettUSDCurrencyId = GetSettUSDCurrencyId;
	type GetSettGBPCurrencyId = GetSettGBPCurrencyId;
	type GetSettEURCurrencyId = GetSettEURCurrencyId;
	type GetSettKWDCurrencyId = GetSettKWDCurrencyId;
	type GetSettJODCurrencyId = GetSettJODCurrencyId;
	type GetSettBHDCurrencyId = GetSettBHDCurrencyId;
	type GetSettKYDCurrencyId = GetSettKYDCurrencyId;
	type GetSettOMRCurrencyId = GetSettOMRCurrencyId;
	type GetSettCHFCurrencyId = GetSettCHFCurrencyId;
	type GetSettGIPCurrencyId = GetSettGIPCurrencyId;
	type GetDexerCurrencyId = GetDexerCurrencyId;
	type SerpTesSchedule = SerpTesSchedule;
	type SerplusSerpupRatio = SerplusSerpupRatio;
	type SettPaySerpupRatio = SettPaySerpupRatio;
	type SetheumTreasurySerpupRatio = SetheumTreasurySerpupRatio;
	type CharityFundSerpupRatio = CharityFundSerpupRatio;
	type SIFSerpupRatio = SIFSerpupRatio;
	type SerpAuctionHandler = MockSerpAuction;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type Dex = ();
	type MaxAuctionsCount = MaxAuctionsCount;
	type PalletId = SerpTreasuryPalletId;
	type WeightInfo = ();
}

parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"set/dexm");
	pub const GetExchangeFee: (u32, u32) = (0, 100);
	pub const TradingPathLimit: u32 = 3;
	pub EnabledTradingPairs : Vec<TradingPair> = vec![TradingPair::new(USDJ, SETT), TradingPair::new(USDJ, EURJ)];
}

impl setheum_dex::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = TradingPathLimit;
	type PalletId = DexPalletId;
	type DexIncentives = ();
	type WeightInfo = ();
	type ListingOrigin = EnsureSignedBy<One, AccountId>;
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub StandardCurrencyIds: Vec<CurrencyId> = vec![
		USDJ, // Setheum USD (US Dollar stablecoin)
		GBPJ, // Setheum GBP (Pound Sterling stablecoin)
		EURJ, // Setheum EUR (Euro stablecoin)
		KWDJ, // Setheum KWD (Kuwaiti Dinar stablecoin)
		JODJ, // Setheum JOD (Jordanian Dinar stablecoin)
		BHDJ, // Setheum BHD (Bahraini Dirham stablecoin)
		KYDJ, // Setheum KYD (Cayman Islands Dollar stablecoin)
		OMRJ, // Setheum OMR (Omani Riyal stablecoin)
		CHFJ, // Setheum CHF (Swiss Franc stablecoin)
		GIPJ, // Setheum GIP (Gibraltar Pound stablecoin)
	];
	pub const GetReserveCurrencyId: CurrencyId = SETT;
	pub DefaultStandardExchangeRate: ExchangeRate = ExchangeRate::one();
	pub const MinimumStandardValue: Balance = 2;
	pub MaxSlippageSwapWithDex: Ratio = Ratio::saturating_from_rational(50, 100);
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl Config for Runtime {
	type Event = Event;
	type PriceSource = MockPriceSource;
	type StandardCurrencyIds = StandardCurrencyIds;
	type GetReserveCurrencyId = GetReserveCurrencyId;
	type DefaultStandardExchangeRate = DefaultStandardExchangeRate;
	type MinimumStandardValue = MinimumStandardValue;
	type GetStableCurrencyId = GetStableCurrencyId;
	type SerpTreasury = SerpTreasuryModule;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type MaxSlippageSwapWithDex = MaxSlippageSwapWithDex;
	type Dex = DexModule;
	type UnsignedPriority = UnsignedPriority;
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
		System: frame_system::{Module, Call, Storage, Config, Event<T>},
		SettmintEngineModule: settmint_engine::{Module, Storage, Call, Event<T>, Config, ValidateUnsigned},
		SerpTreasuryModule: serp_treasury::{Module, Storage, Call, Config, Event<T>},
		Currencies: orml_currencies::{Module, Call, Event<T>},
		Tokens: orml_tokens::{Module, Storage, Event<T>, Config<T>},
		SettersModule: setters::{Module, Storage, Call, Event<T>},
		PalletBalances: pallet_balances::{Module, Call, Storage, Event<T>},
		DexModule: setheum_dex::{Module, Storage, Call, Event<T>, Config<T>},
	}
);

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> SendTransactionTypes<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, SETT, 1000),
				(BOB, SETT, 1000),
				(CAROL, SETT, 100),
				(ALICE, EURJ, 1000),
				(BOB, EURJ, 1000),
				(CAROL, USDJ, 1000),
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
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		setheum_dex::GenesisConfig::<Runtime> {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: EnabledTradingPairs::get(),
			initial_added_liquidity_pools: vec![],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
