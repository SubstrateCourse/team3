#![cfg_attr(not(feature = "std"), no_std)]

use alt_serde::{Deserialize, Deserializer};
/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{debug, decl_error, decl_event, decl_module, decl_storage};
use frame_system::{
    self as system,
    offchain::{AppCrypto, CreateSignedTransaction},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{offchain as rt_offchain, offchain::storage::StorageValueRef};
use sp_std::prelude::*;
use sp_std::str;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const ETH_PRICE_COINCAP_API: &[u8] = b"https://api.coincap.io/v2/assets/ethereum";
pub const ETH_PRICE_BITFINEX_API: &[u8] = b"https://api.bitfinex.com/v1/pubticker/ethusd";

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
pub struct CoinCap {
    data: CoinCapData,
}

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinCapData {
    #[serde(deserialize_with = "de_string_to_f64")]
    price_usd: f64,
}

#[serde(crate = "alt_serde")]
#[derive(Debug, Deserialize)]
pub struct Bitfinex {
    #[serde(deserialize_with = "de_string_to_f64")]
    last_price: f64,
}

fn de_string_to_f64<'der, D>(der: D) -> Result<f64, D::Error>
where
    D: Deserializer<'der>,
{
    let s: &str = Deserialize::deserialize(der)?;
    Ok(s.parse::<f64>().unwrap())
}

pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;

    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };

    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;
    // implemented for ocw-runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

/// This is the pallet's configuration trait
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        /// To emit this event, we call the deposit function, from our runtime functions
        SomethingStored(u32, AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        ResponseParseError,
        ResponseCharsetError,
        HttpError,
        AlreadyFetched
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

        fn offchain_worker(_block_number: T::BlockNumber) {
            debug::info!("Entering off-chain workers");

            /*******
             * 学员们在这里追加逻辑
             *******/
             let price_result = Self::fetch_eth_price();
             let result = Self::store_eth_price(price_result);
             if let Err(e) = result {
                debug::error!("Error: {:?}", e);
            }
        }
    }
}

impl<T: Trait> Module<T> {
    fn store_eth_price(price_result: Result<u64, Error<T>>) -> Result<(), Error<T>> {
        let eth_prices = StorageValueRef::persistent(b"offchain-worker::eth-prices");
        let eth_prices_lock = StorageValueRef::persistent(b"offchain-worker::eth-prices-lock");
        let res: Result<Result<bool, bool>, Error<T>> =
            eth_prices_lock.mutate(|s: Option<Option<bool>>| {
                match s {
                    // lock is never set or set to false
                    None | Some(Some(false)) => Ok(true),
                    // lock is set to true
                    _ => Err(<Error<T>>::AlreadyFetched),
                }
            });

        if let Ok(Ok(true)) = res {
            let mut prices: Vec<u64> = Vec::<u64>::new();
            if let Some(Some(fetched_prices)) = eth_prices.get::<Vec<u64>>() {
                prices = fetched_prices;
            }
            match price_result {
                Ok(price) => {
                    prices.push(price);
                    debug::info!("current price list: {:?}", prices);
                    eth_prices.set(&prices);
                    eth_prices_lock.set(&false);
                }
                Err(err) => {
                    eth_prices_lock.set(&false);
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn fetch_eth_price() -> Result<u64, Error<T>> {
        let coincap_result = Self::fetch_data_from_api(&ETH_PRICE_COINCAP_API);
        let bitfinex_result = Self::fetch_data_from_api(&ETH_PRICE_BITFINEX_API);
        let coincap_price;
        let bitfinex_price;
        match coincap_result {
            Ok(coincap_bytes) => {
                let coincap_str =
                    str::from_utf8(&coincap_bytes).map_err(|_| Error::<T>::ResponseCharsetError)?;
                let coincap = serde_json::from_str::<CoinCap>(coincap_str)
                    .map_err(|_| Error::<T>::ResponseParseError)?;
                debug::info!("coincap object: {:?}", coincap);
                coincap_price = coincap.data.price_usd as u64;
            }
            Err(err) => {
                return Err(err);
            }
        }
        match bitfinex_result {
            Ok(bitfinex_bytes) => {
                let bitfinex_str = str::from_utf8(&bitfinex_bytes)
                    .map_err(|_| Error::<T>::ResponseCharsetError)?;
                let bitfinex = serde_json::from_str::<Bitfinex>(bitfinex_str)
                    .map_err(|_| Error::<T>::ResponseParseError)?;
                debug::info!("bitfinex object: {:?}", bitfinex);
                bitfinex_price = bitfinex.last_price as u64;
            }
            Err(err) => {
                return Err(err);
            }
        }
        let price = (coincap_price + bitfinex_price) / 2;
        Ok(price)
    }

    fn fetch_data_from_api(url_bytes: &[u8]) -> Result<Vec<u8>, Error<T>> {
        let remote_url = str::from_utf8(url_bytes).map_err(|_| <Error<T>>::HttpError)?;
        let request = rt_offchain::http::Request::get(remote_url);

        let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

        let pending = request
            // .add_header("Content-Type", str::from_utf8(b"application/json;charset=utf-8")
            .deadline(timeout)
            .send()
            .map_err(|_| <Error<T>>::HttpError)?;

        let response = pending
            .try_wait(timeout)
            .map_err(|_| Error::<T>::HttpError)?
            .map_err(|_| Error::<T>::HttpError)?;

        if response.code != 200 {
            debug::error!("Unexpected http status code: {}", response.code);
            return Err(<Error<T>>::HttpError);
        }

        Ok(response.body().collect::<Vec<u8>>())
    }
}
