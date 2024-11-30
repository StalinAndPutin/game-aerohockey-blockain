#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use frame_support::{decl_module, decl_storage};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, TypeInfo)]
pub struct Era<H> {
	/// Genesis block hash of the era.
	pub genesis_block_hash: H,
	/// Final block hash.
	pub final_block_hash: H,
	/// Final state root.
	pub final_state_root: H,
}

pub trait Config: frame_system::Config {}

decl_storage! {
	trait Store for Module<T: Config> as Eras {
		/// Past eras.
		pub PastEras get(fn past_eras) config(past_eras): Vec<Era<T::Hash>>;
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin { }
}
