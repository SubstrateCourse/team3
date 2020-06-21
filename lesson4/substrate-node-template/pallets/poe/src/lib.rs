#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath};

// use frame_support::{
// 	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,storage::{StorageDoubleMap,StorageMap},
// 	traits::{Currency, ExistenceRequirement, Get}
// };
// use frame_system::{self as system, ensure_signed};
// use sp_std::prelude::*;
// use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait{
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
	// 附加题答案
	type MaxClaimLength: Get<u32>;

	type MaxNoteLength: Get<u32>;
}

//不太了解 js怎么支持Vec<struct>结构

// #[derive(Encode, Decode, Default, Clone, PartialEq)]
// #[cfg_attr(feature = "std",derive(Debug))]
// pub struct ClaimNote<BlockNumber> {
// 	claim: Vec<u8>,
// 	note: Vec<u8>,
// 	block_number: BlockNumber,
// }


type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	// PoeModule
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		Prices get(fn price):  map hasher(blake2_128_concat) Vec<u8> => (BalanceOf<T>, T::BlockNumber);
	    //pub Claims: double_map hasher(blake2_128_concat) Vec<u8>, Vec<u8> => (T::AccountId, T::BlockNumber);
		//Claims get(fn claims): map hasher(blake2_128_concat)
		//ClaimNoteDoubleMap get(fn claimkey): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) Vec<u8> => (T::AccountId, Vec<u8>, Vec<u8>, T::BlockNumber);
	    //Claims get(fn claims): Vec<ClaimNote<T::BlockNumber>>;
	    ProofInfo get(fn pio): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Moment, Option<Vec<u8>>);
	    AccountClaims get(fn acs): map hasher(identity) T::AccountId => Vec<Vec<u8>>;

	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfered(AccountId , Vec<u8>, AccountId),
		//ClaimsCreated(AccountId, Vec<u8>, Vec<u8>),

	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		ClaimsTooLong,
		NoteTooLong,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

       #[weight = 0]
       pub fn create_claims_note(origin, claim: Vec<u8> , note: Option<Vec<u8>>) -> dispatch::DispatchResult {

        	let sender = ensure_signed(origin)?;

        	ensure!(!ProofInfo::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

        	ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

             match note.clone() {
                None => (),
                Some(text) => ensure!(T::MaxNoteLength::get() >= text.len() as u32, Error::<T>::NoteTooLong),
            }
        	//ensure!(T::MaxNoteLength::get() >= note.len() as u32, Error::<T>::ClaimsTooLong);

            ProofInfo::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number(), <timestamp::Module<T>>::now(), note.clone()));

            if AccountClaims::<T>::contains_key(&sender) {
                let mut vec = AccountClaims::<T>::get(&sender);
                match vec.binary_search(&claim) {
                    Ok(_) => (),
                    Err(index) => vec.insert(index, claim.clone()),
                };
                AccountClaims::<T>::insert(&sender, vec);
            }else {
                let mut vec = Vec::<Vec<u8>>::new();
                vec.push(claim.clone());
                AccountClaims::<T>::insert(&sender, vec);
            }
        	Self::deposit_event(RawEvent::ClaimCreated(sender,claim));
        	Ok(())

       }

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

       /* #[weight = 0]
        pub fn query_claims(origin) -> dispatch::DispatchResult {

			let sender = ensure_signed(origin)?;

            let results: Vec<(T::AccountId, Vec<u8>, Vec<u8>, T::BlockNumber)> = ClaimNoteDoubleMap::<T>::iter_prefix_values(sender).collect::<Vec<(T::AccountId,Vec<u8>,Vec<u8>,T::BlockNumber)>>();

            for  (_, claim, note, block_number) in results {
                 let claim_item = ClaimNote{ claim, note, block_number };
                 <Claims<T>>::append(&claim_item);
            }

            Ok(())

        }*/

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		// 第二题答案
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			let dest = T::Lookup::lookup(dest)?;

			Proofs::<T>::insert(&claim, (&dest, system::Module::<T>::block_number()));
            Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, dest));

			Ok(())
		}
	}
}
