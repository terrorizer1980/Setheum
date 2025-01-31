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

use super::*;
use primitives::{Balance, CurrencyId};
use sp_runtime::traits::Convert;
use sp_runtime::FixedPointNumber;

pub struct StandardExchangeRateConvertor<T>(sp_std::marker::PhantomData<T>);

impl<T> Convert<(CurrencyId, Balance), Balance> for StandardExchangeRateConvertor<T>
where
	T: Config,
{
	fn convert((currency_id, balance): (CurrencyId, Balance)) -> Balance {
		<Pallet<T>>::get_standard_exchange_rate(currency_id).saturating_mul_int(balance)
	}
}
