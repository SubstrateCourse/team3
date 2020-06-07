#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as PoeModule {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
//		Something get(fn something): Option<u32>;
            Proofs get(fn proofs): map hasher( blake2_128_concat) Vec<u8> =>(T::AccountId, T::BlockNumber);

    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        AccountIdDest = <T as system::Trait>::AccountId,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransfered(AccountId, AccountIdDest, Vec<u8>),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
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
        pub fn create_claim(origin, claim:Vec<u8>)->dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(&claim,(sender.clone(), system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimCreated(sender,claim));

            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin,claim:Vec<u8>)->dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let(owner, _block_number)=Proofs::<T>::get(&claim);

            ensure!(owner==sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));
            Ok(())
        }

        #[weight = 0]
        pub fn transfer_claim(origin, claim:Vec<u8>, dest: <T as system::Trait>::AccountId)->dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(&claim, (dest.clone(), system::Module::<T>::block_number()));

            Self::deposit_event(RawEvent::ClaimTransfered(sender,dest, claim));

            Ok(())
        }

    }
}
