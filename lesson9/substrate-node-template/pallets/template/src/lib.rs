#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use core::{convert::TryInto};
use parity_scale_codec::{Decode, Encode};

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{
	self as system, ensure_none, ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer, // SubmitTransaction,
	},
};

use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain as rt_offchain,
};
use sp_std::prelude::*;
use sp_std::str;

// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//type Result<T> = core::result::Result<T, &'static str>;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw8");

pub const MAX_VEC_LEN: usize = 1000;


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

pub const FETCHED_CRYPOTS: [(&[u8], &[u8], &[u8]); 2] = [
	(
		b"ETH",
		b"cryptocompare",
		b"https://min-api.cryptocompare.com/data/price?fsym=eth&tsyms=USD",
	),
	(
		b"ETHSymbol",
		b"cryptocurrency",
		b"https://api.coincap.io/v2/assets/ethereum",
	),
];


pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
	where
		D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}

pub fn de_string_to_integer<'de, D>(de: D) -> Result<u32, D::Error>
	where
		D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	let f: f32 =  s.parse::<f32>().unwrap();
	debug::info!("price02 is {}",f);
	Ok((f * 100.0) as u32)
}

pub fn de_float_to_integer<'de, D>(de: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
{
	let f: f32 = Deserialize::deserialize(de)?;
	Ok((f*100.0) as u32)
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct ETHPriceCompare {
	#[serde(rename(deserialize = "USD"), deserialize_with = "de_float_to_integer")]
	usd: u32,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct Status {
	#[serde(deserialize_with = "de_string_to_bytes")]
	timestamp: Vec<u8>,
	error_code: u32,
	//error_message: String,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct Data {
	#[serde(rename(deserialize = "priceUsd"), deserialize_with = "de_string_to_integer")]
	price_usd: u32,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EthInfo {
	data: Data,
}

//
// #[serde(crate = "alt_serde")]
// #[derive(Deserialize, Encode, Decode, Default)]
// struct ETH {
// 	quote: Quote,
// }
//
// #[serde(crate = "alt_serde")]
// #[derive(Deserialize, Encode, Decode, Default)]
// struct Quote {
// 	#[serde(rename = "USD")]
// 	usd: USD,
// }
//
// #[serde(crate = "alt_serde")]
// #[derive(Deserialize, Encode, Decode, Default)]
// struct USD {
// 	#[serde(deserialize_with = "de_float_to_integer")]
// 	price: u32,
// }
//
// #[serde(crate = "alt_serde")]
// #[derive(Deserialize, Encode, Decode, Default)]
// struct ETHPriceInfoMark {
// 	status: Status,
// 	data: Data,
// }


/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>>{
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The overarching event type.
	type Call: From<Call<Self>>;
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	//type BlockFetchDur: Get<T::BlockNumber>;
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
		//Something get(fn something): Option<u32>;

		Prices get(fn prices): Vec<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		//SomethingStored(u32, AccountId),
		NewPrice(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		//NoneValue,
		/// Value reached maximum and cannot be incremented further
		//StorageOverflow,
		HttpGetRequestError,
	    ConvertingRomoteUrlError,
	    HttpResponseError,
	    ParseResponseError,
		// HttpFetchingEthSybmolFailed,
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
		pub fn save_number(origin, number: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			Self::add_price(who, number);
			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			 match block_number.try_into().ok().unwrap() % 10 {
			     0 => {
							let res = Self::fetch_price_and_send_signed();
							if let Err(e) = res {
									debug::error!("Error: {}", e);
							}
			     }
			     _ => debug::info!("ignore..."),
			 }
		}

	}
}

impl <T: Trait> Module<T>{

	fn add_price(who: T::AccountId, price: u32) {
		Prices::mutate(|prices| {
			prices.push(price);
		});

		Self::deposit_event(RawEvent::NewPrice(price, who))
	}

	fn fetch_price_and_send_signed() -> Result<(), &'static str> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			return Err(
				"No local accounts available. Consider adding one via `author_insertKey` RPC."
			)?
		}
		let price1 = Self::fetch_eth_price01().map_err(|_| "Failed to fetch eth price ")?;
		let price2 = Self::fetch_eth_price02().map_err(|_| "Failed to fetch eth price ")?;
		let price = (price1 + price2)/2;
		debug::info!("average eth price in usd cents: {}", price);

		let results = signer.send_signed_transaction(
			|_account| {

				Call::save_number(price)
			}
		);

		for (acc, res) in &results {
			match res {
				Ok(()) => debug::info!("[{:?}] Submitted price of {} cents", acc.id, price),
				Err(e) => debug::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
			}
		}

		Ok(())
	}

    fn parese_response_eth01(res_bytes: Vec<u8>) -> Result<ETHPriceCompare, Error<T>> {
		let resp_str = str::from_utf8(&res_bytes)
			.map_err(|_| <Error<T>>::ParseResponseError)?;
		let eth_info: ETHPriceCompare = serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::ParseResponseError)?;
		let price = eth_info.usd;
		debug::info!("price is: {}", price);
		Ok(eth_info)
	}


	fn parese_response_eth02(res_bytes: Vec<u8>) -> Result<EthInfo, Error<T>> {
		let resp_str = str::from_utf8(&res_bytes)
			.map_err(|_| <Error<T>>::ParseResponseError)?;
		let eth_info: EthInfo = serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::ParseResponseError)?;
		Ok(eth_info)
	}

	fn fetch_json<'a>(remote_url: &'a[u8]) -> Result<Vec<u8>, Error<T>> {

		let remote_url_str = core::str::from_utf8(remote_url)
			.map_err(|_| <Error<T>>::ConvertingRomoteUrlError)?;
        let url_bug = remote_url_str.clone();
		debug::info!("url is : {}", &url_bug);
		let request = rt_offchain::http::Request::get(remote_url_str);

		let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(10000));
		// if headers.0.len() {
		//
		// }else {
		//
		// }
		let pending = request
			//.add_header(headers.0, headers.1)
			.deadline(timeout) // Setting the timeout time
			.send() // Sending the request out by the host
			.map_err(|_| <Error<T>>::HttpGetRequestError)?;


		let response = pending
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpResponseError)?
			.map_err(|_| <Error<T>>::HttpResponseError)?;

		if response.code != 200 {
			return Err(<Error<T>>::HttpResponseError)
		}

		Ok(response.body().collect::<Vec<u8>>())

	}

	fn fetch_eth_price01() -> Result<u32, Error<T>> {
		let config = FETCHED_CRYPOTS[0];
		let response =  Self::fetch_json(config.2).map_err(|e|{
			debug::error!("fetch remote url failed: {:?}" ,e);
			<Error<T>>::HttpGetRequestError
		})?;
        match Self::parese_response_eth01(response) {
			Ok(eth) => {
				let price = eth.usd;
				Ok(price)
			}
			Err(e) => {
				debug::error!("parse error: {:?}",e);
				Err(e)
			}
		}
	}


	fn fetch_eth_price02() -> Result<u32, Error<T>> {
		let config = FETCHED_CRYPOTS[1];
		let result = Self::fetch_json(config.2).map_err(|e| {
			debug::error!("fetch remote url failed: {:?}" ,e);
			<Error<T>>::HttpGetRequestError
		})?;

		match Self::parese_response_eth02(result) {
			Ok(eth) => {
				let price_usd = eth.data.price_usd;
				Ok(price_usd)
			}
			Err(e) => {
				debug::error!("parse error: {:?}", e);
				Err(e)
			}
		}
	}

}