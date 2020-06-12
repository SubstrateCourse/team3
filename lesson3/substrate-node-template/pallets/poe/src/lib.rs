#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
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
    type MaxClaimLength: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, u32);
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
        ClaimTransfered(AccountId, Vec<u8>, AccountId),
        ClaimSelled(Vec<u8>, AccountId, AccountId, u32),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        ProofTooLong,
        NotEnoughPrice,
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

        /// Just a dummy entry point.
        /// function that can be called by the external world as an extrinsics call
        /// takes a parameter of the type `AccountId`, stores it, and emits an event

        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>, price: u32) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            ensure!(claim.len() as u32 <= T::MaxClaimLength::get(), Error::<T>::ProofTooLong);

            Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number(), price));

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            let (owner, _block_number, _price) = Proofs::<T>::get(&claim);
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
            Ok(())
        }


        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            let (owner, _block_number, price) = Proofs::<T>::get(&claim);
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            let dest = T::Lookup::lookup(dest)?;

            Proofs::<T>::insert(&claim, (dest.clone(), system::Module::<T>::block_number(), price));

            Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, dest));
            Ok(())
        }


        #[weight = 0]
        pub fn buy_claim(origin, claim: Vec<u8>, offer_price: u32) -> dispatch::DispatchResult {

            let sender = ensure_signed(origin)?;
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            let (owner, _block_number, price) = Proofs::<T>::get(&claim);
            ensure!(offer_price >= price, Error::<T>::NotEnoughPrice);

            Self::deposit_event(RawEvent::ClaimSelled(claim, owner, sender, offer_price));
            Ok(())
        }
    }
}
