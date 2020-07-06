#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, StorageValue, StorageMap,dispatch, traits::{Randomness, Currency, ExistenceRequirement}, Parameter};
use sp_io::hashing::blake2_128;
// substrate的一个Bug 要求使用的时候frame_syste 名字必须是system
use frame_system::{self as system,ensure_signed};
use sp_runtime::{DispatchError, traits::{AtLeast32Bit, Bounded, Member}};
use crate::linked_item::LinkedItem;
use crate::linked_item::LinkedList;
//use crate::linked_item::{LinkedList, LinkedItem}; 组合引用
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

mod linked_item;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

// feature指定生效环境 测试环境才会用到定义在std下，derive约束打印、比较特征，链上资源宝贵减小wasm环境大小
// #[cfg_attr(feature = "std", derive(Debug, PartialEq,Eq))]
// #[derive(Encode, Decode)]
// pub struct KittyLinkedItem<T: Trait> {
//   pub prev: Option<T::KittyId>,
//   pub next: Option<T::KittyId>,
// }
pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	// 定义小猫ID 使用的时候指定类型;加上限定类型 Member代表该类型可以放到结构体或者枚举中使用
	type KittyId: Parameter + Member + AtLeast32Bit + Bounded + Default + Copy;
	type Currency: Currency<Self::AccountId>;  // 使用Currency接口方式更灵活;需要的时候使用Currency里面定义的balances 解耦优化
	type Randomness: Randomness<Self::Hash>; // 随机数解耦写法 定义接口，使用的时候再具体指定实现类型
}
// 注入Balances类型 定义T泛型必须是实现了特征Trait的，并且取该T（Trait）下的Currency类型 (as 类型转换【类似多态，上层类型可以有多个实现类型进行实例化，as 就是指定具体的实现类型】) 并且具体指定模块对象，
// 也就是指定同样实现了Trait的frame_system模块，并使用该模块的AccountId对象作为Currency（接口）类型的泛型参数；
// 这样就定义了一个Currency类型，并取该类型的Balance类型
// 然后赋值给BalanceOf<T>，这样就在该模块下定义了一个type BalanceOf<T>类型
// 注意T泛型的类型必须实现了Trait这个特征约束(接口)  这样解耦好处就是比如runtime模块就可以根据自己需要指定实现Currency接口的具体Balances
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type KittyLinkedItem<T> =  LinkedItem<<T as Trait>::KittyId>;// LinkedItem<<T as system::Trait>::KittyId>;
// type OwnedKittiesList<T> = LinkedList<OwnedKitties<T>, <T as system::Trait>::AccountId,<T as system::Trait>::KittyId>;
type OwnedKittiesList<T> = LinkedList<OwnedKitties<T>, <T as system::Trait>::AccountId,<T as Trait>::KittyId>;

decl_storage! {
	trait Store for Module<T: Trait> as kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyId => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): T::KittyId;

		// /// Get kitty ID by account ID and user kitty index
		// pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) (T::AccountId, T::KittyId) => T::KittyId;
		// /// Get number of kitties by account ID
		// pub OwnedKittiesCount get(fn owned_kitties_count): map hasher(blake2_128_concat) T::AccountId => T::KittyId;
		 // 用户小猫相关数据使用自定义的链表数据结构保存
		 pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) (T::AccountId,Option<T::KittyId>) => Option<KittyLinkedItem<T>>;
		 // 小猫ID映射用户ID 55
		 pub KittyOwners get(fn kitty_owners): map hasher(blake2_128_concat) T::KittyId => Option<T::AccountId>;
		 // 小猫对应的价格
		 pub KittyPrices get(fn kitty_prices): map hasher(blake2_128_concat) T::KittyId => Option<BalanceOf<T>>;

	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where 
	 // 指定事件用到的参数以及事件T的约束 
	 AccountId = <T as frame_system::Trait>::AccountId, 
	 KittyId = <T as Trait>::KittyId,
	Balance = BalanceOf<T>,
	{
	   // 成功创建了一只小猫 (owner, kitty_id)	
	   Created(AccountId,KittyId),
	   // 小猫转移（交易）成功(from,to,kitty_id)
	   Transfered(AccountId, AccountId,KittyId),
	   // 小猫不能进行交易 （owner,kitty_id,price)
	   Ask(AccountId,KittyId,Option<Balance>),//Option<Balance>
	   // 小猫卖出成功 （from，to,kitty_id,price) 
	   Sold(AccountId,AccountId,KittyId,Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		RequireOwner,
		KittyNotForSale,
		PriceTooLow,
	}
}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		// decl_module宏展会自动将事件方法进行扩展处理，处理为符合rust语法的事件方法
		fn deposit_event() = default;
		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			// 生成新猫ID
			let new_kitty_id = Self::next_kitty_id()?;
          
			// Generate a random 128bit value 生成新猫DNA
			let dna = Self::random_value(&sender);
			
		 	// Create and store kitty 生成新猫
			let new_kitty = Kitty(dna);

            // 新猫信息存储
			Self::insert_kitty(&sender, new_kitty_id, new_kitty);
			// 发送事件
			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
			
			Ok(())
		}
  
		/// Breed kitties  繁殖小猫
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyId, kitty_id_2: T::KittyId) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

			// 发送事件
			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));

			Ok(())
		}
		// transfer kitty to another
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId,kitty_id: T::KittyId){
			// 小猫转移
			let sender = ensure_signed(origin)?; 
			// 判断操作人是否为猫主人
			ensure!(<OwnedKitties<T>>::contains_key((&sender,Some(kitty_id))),<Error<T>>::RequireOwner);
			// 调用转移方法
			Self::do_transfer(&sender,&to,kitty_id);
			// 发送事件
			Self::deposit_event(RawEvent::Transfered(sender,to, kitty_id));
		}

			// 设置猫卖价 
			#[weight = 0]
			pub fn set_price(origin,kitty_id: T::KittyId,sell_price: Option<BalanceOf<T>>){ 
				let sender = ensure_signed(origin)?; 
				// 判断操作人是否为猫主人
				 ensure!(<OwnedKitties<T>>::contains_key((&sender,Some(kitty_id))),<Error<T>>::RequireOwner);
				 // 设置价格 mutate_exists 的用法
				 <KittyPrices<T>>::mutate_exists(kitty_id,|price| *price=sell_price);
	
				// 发送事件
				Self::deposit_event(RawEvent::Ask(sender, kitty_id,sell_price)); 
			}
			// 买猫 
			#[weight = 0]
			pub fn buy_kitty(origin,kitty_id: T::KittyId,buy_price: BalanceOf<T>){
				// 购买人 
				let sender = ensure_signed(origin)?; 
				// 根据小猫ID获取对应的主人
				let owner= Self::kitty_owners(kitty_id).ok_or(<Error<T>>::InvalidKittyId)?;
				// 价格为None 则为非卖猫
				let sell_price = Self::kitty_prices(kitty_id).ok_or(<Error<T>>::KittyNotForSale);
				// 判断买价大于卖价
				ensure!(buy_price >= sell_price.unwrap_or_default(),<Error<T>>::PriceTooLow);
				// 买家转账给卖家 校验账号状态是否合规(不能将账号中的钱都转走导致账号的余额低于系统规定的某值，而被清除账户，不然判定为尘埃账户) ?:处理失败则返回
				// pallet_ballances的Module模块调用Curency接口的方法transfer接口泛型参数为T::AccountId
				// <pallet_balances::Module<T> as Currency<T::AccountId>>::tranfer(&sender,&owner,sell_price,ExistenceRequirement::KeepAlive)?;
				// 直接使用T下的currency并调用其transfer方法 .unwrap_or_default()
				T::Currency::transfer(&sender,&owner, buy_price,ExistenceRequirement::KeepAlive)?;
				
				// 转账流程通过 更新猫相关数据数据
			
				// 重置/移除该猫的价格	
				<KittyPrices<T>>::remove(kitty_id);
				// 调用转移方法
				Self::do_transfer(&owner,&sender,kitty_id);
				// 发送事件
				Self::deposit_event(RawEvent::Sold(owner,sender,kitty_id,buy_price));
	
			}
	}
}

// impl<T: Trait> OwnedKitties<T> {
// 	fn read_head(account: &T::AccountId) -> KittyLinkedItem<T> {
// 		Self::read(account, None)
// 	}

// 	fn write_head(account: &T::AccountId, item: KittyLinkedItem<T>) {
// 		Self::write(account, None, item);
// 	}

// 	fn read(account: &T::AccountId, key: Option<T::KittyId>) -> KittyLinkedItem<T> {
// 		<OwnedKitties<T>>::get((&account, key)).unwrap_or_else(|| KittyLinkedItem {
// 			prev: None,
// 			next: None,
// 		})
// 	}

// 	fn write(account: &T::AccountId, key: Option<T::KittyId>, item: KittyLinkedItem<T>) {
// 		<OwnedKitties<T>>::insert((&account, key), item);
// 	}

// 	pub fn append(account: &T::AccountId, kitty_id: T::KittyId) {
// 		let head = Self::read_head(account);
// 		let new_head = KittyLinkedItem {
// 			prev: Some(kitty_id),
// 			next: head.next,
// 		};

// 		Self::write_head(account, new_head);

// 		let prev = Self::read(account, head.prev);
// 		let new_prev = KittyLinkedItem {
// 			prev: prev.prev,
// 			next: Some(kitty_id),
// 		};
// 		Self::write(account, head.prev, new_prev);

// 		let item = KittyLinkedItem {
// 			prev: head.prev,
// 			next: None,
// 		};
// 		Self::write(account, Some(kitty_id), item);
// 	}

// 	pub fn remove(account: &T::AccountId, kitty_id: T::KittyId) {
// 		if let Some(item) = <OwnedKitties<T>>::take((&account, Some(kitty_id))) {
// 			let prev = Self::read(account, item.prev);
// 			let new_prev = KittyLinkedItem {
// 				prev: prev.prev,
// 				next: item.next,
// 			};

// 			Self::write(account, item.prev, new_prev);

// 			let next = Self::read(account, item.next);
// 			let new_next = KittyLinkedItem {
// 				prev: item.prev,
// 				next: next.next,
// 			};

// 			 Self::write(account, item.next, new_next);
// 		}
// 	}
// }

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (
			// <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			// 解耦写法
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> sp_std::result::Result<T::KittyId, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyId::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}
	// 转移方法提取
	fn do_transfer(source: &T::AccountId,target: &T::AccountId,kitty_id: T::KittyId){
		// 从原主人数据中移除掉
		<OwnedKittiesList<T>>::remove(source, kitty_id);
		// 将数据添加到新主人数据中 
	    Self::insert_owned_kitty(target, kitty_id);
	
	}
	// 保存小猫信息
	fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyId) {
		// 用户的小猫
		<OwnedKittiesList<T>>::append(&owner,kitty_id);
		// 小猫的主人
		<KittyOwners<T>>::insert(kitty_id,&owner);
	}

	fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyId, kitty: Kitty) {
		// Create and store kitty
		Kitties::<T>::insert(kitty_id, kitty);
		KittiesCount::<T>::put(kitty_id + 1.into());
		Self::insert_owned_kitty(owner, kitty_id);
	}

	fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyId, kitty_id_2: T::KittyId) ->  sp_std::result::Result<T::KittyId,DispatchError> {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		
		ensure!(<OwnedKitties<T>>::contains_key((&sender,Some(kitty_id_1))), Error::<T>::RequireOwner);
		ensure!(<OwnedKitties<T>>::contains_key((&sender,Some(kitty_id_2))), Error::<T>::RequireOwner);
		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(kitty_id)
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};
	use frame_system as system;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
		// 新增测试常量
		pub const ExistentialDeposit: u64 = 1;
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type MaximumExtrinsicWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
	}
    // 为测试实现balances相关类型
	impl balances::Trait for Test {
		type Balance = u64;
		type DustRemoval = ();
		type Event = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
	}
	// 实现测试中需要用到的类型
	impl Trait for Test {
		type Event = ();
		type KittyId = u32;
		type Currency = Balances;
		type Randomness = pallet_randomness_collective_flip::Module<Test>;
	}
	type OwnedKittiesTest = OwnedKitties<Test>;
    // 测试中引入类型
	pub type System = system::Module<Test>;
	pub type Balances = balances::Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn owned_kitties_can_append_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesList::<Test>::append(&0, 1);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: Some(1),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: None,
			}));

			OwnedKittiesList::<Test>::append(&0, 2);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: Some(2),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem::<Test> {
				prev: Some(1),
				next: None,
			}));

			OwnedKittiesList::<Test>::append(&0, 3);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: Some(3),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem::<Test> {
				prev: Some(1),
				next: Some(3),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem::<Test> {
				prev: Some(2),
				next: None,
			}));
		});
	}

	#[test]
	fn owned_kitties_can_remove_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesList::<Test>::append(&0, 1);
			OwnedKittiesList::<Test>::append(&0, 2);
			OwnedKittiesList::<Test>::append(&0, 3);

			OwnedKittiesList::<Test>::remove(&0, 2);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: Some(3),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: Some(3),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem::<Test> {
				prev: Some(1),
				next: None,
			}));

			OwnedKittiesList::<Test>::remove(&0, 1);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: Some(3),
				next: Some(3),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: None,
			}));

			OwnedKittiesList::<Test>::remove(&0, 3);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem::<Test> {
				prev: None,
				next: None,
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), None);
		});
	}
}