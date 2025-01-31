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

use crate::{AccountId, CurrencyId, Runtime, System, TokenSymbol, TransactionPayment};
use frame_benchmarking::account;
use frame_support::traits::OnFinalize;
use frame_system::RawOrigin;
use orml_benchmarking::runtime_benchmarks;
use sp_std::prelude::*;

const SEED: u32 = 0;

runtime_benchmarks! {
	{ Runtime, setheum_transaction_payment }

	_ {}

	set_default_fee_token {
		let caller: AccountId = account("caller", 0, SEED);
		let currency_id = CurrencyId::Token(TokenSymbol::SETUSD);
	}: _(RawOrigin::Signed(caller.clone()), Some(currency_id))
	verify {
		assert_eq!(TransactionPayment::default_fee_currency_id(&caller), Some(currency_id));
	}

	on_finalize {
	}: {
		TransactionPayment::on_finalize(System::block_number());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::assert_ok;

	fn new_test_ext() -> sp_io::TestExternalities {
		frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into()
	}

	#[test]
	fn test_set_default_fee_token() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_set_default_fee_token());
		});
	}

	#[test]
	fn test_on_finalize() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_on_finalize());
		});
	}
}
