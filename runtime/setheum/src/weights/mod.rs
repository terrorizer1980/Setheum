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

//! A list of the different weight modules for our runtime.
#![allow(clippy::unnecessary_cast)]

pub mod setmint_engine;
pub mod serp_treasury;
pub mod setheum_currencies;
pub mod dex;
pub mod setmint_gateway;
pub mod setheum_nft;
pub mod serp_prices;
pub mod setheum_transaction_payment;

pub mod orml_authority;
pub mod orml_oracle;
pub mod orml_tokens;
pub mod orml_vesting;
