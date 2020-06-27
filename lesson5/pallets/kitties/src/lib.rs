#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_error, ensure, StorageValue, StorageMap, traits::Randomness};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError, DispatchResult};

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) (T::AccountId, u32) => u32;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(fn owned_kitties_count): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			// 生成新猫ID
			let new_kitty_id = Self::next_kitty_id()?;

			// Generate a random 128bit value 生成新猫DNA
			let dna = Self::random_value(&sender);

		 	// Create and store kitty 生成新猫
			let new_kitty = Kitty(dna);

			// 作业：补完剩下的部分
            // 新猫信息存储
            Self::insert_kitty(sender, new_kitty_id, new_kitty);

		}

		/// Breed kitties  繁殖小猫
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: u32, kitty_id_2: u32) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}
	}
}
// 父系 母系 DNA 和选择参数生成新的DNA
fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		//	作业：完成方法
		let payload =(
			<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);

		let dna = payload.using_encoded(blake2_128);

		return dna;
	}

	fn next_kitty_id() -> sp_std::result::Result<u32, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == u32::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		// 溢出校验
		// ensure!(count != u32::max_value(),<ErrorM<T>>::KittiesCountOverflow);
		Ok(kitty_id)
	}

	fn insert_kitty(sender: T::AccountId, kitty_id: u32, kitty: Kitty) {

		// 用户拥有历史猫总数
		let user_total_kitties = Self::owned_kitties_count(&sender);
		// 用户-猫总数：猫ID  猫ID对应了用户的第几只猫
		<OwnedKitties<T>>::insert((sender.clone(), user_total_kitties),kitty_id);
		// 更新当前用户拥有的猫数量
		<OwnedKittiesCount<T>>::insert(sender, user_total_kitties+1);
		// kittes  所有猫 maps cat_id : cat
		Kitties::insert(kitty_id,kitty);
		// 历史猫总数
		let count = KittiesCount::get();
		// 更新猫总数量
		KittiesCount::put(count+1);
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: u32, kitty_id_2: u32) -> DispatchResult {
		// 判断公母猫是否存在
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;
		// 两只不同的猫 性别 ？
		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);

		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty  繁殖生成新DNA
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}
}