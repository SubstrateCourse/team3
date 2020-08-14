#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version="0.1.0")]
mod erc20{
    use ink_core::storage;

    #[ink(storage)]
    struct Erc20{
        total_supply:storage::Value<Balance>,
        balance:storage::HashMap<AccountId,Balance>,
        allowances: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transfer{
        #[ink(topic)]
        from:Option<AccountId>,
        #[ink(topic)]
        to:Option<AccountId>,
        value:Balance,
    }

    #[ink(event)]
    struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Erc20{
        #[ink(constructor)]
        fn new(&mut self,initial_supply:Balance){
            let caller=self.env().caller();
            self.total_supply.set(initial_supply);
            self.balance.insert(caller,initial_supply);
            self.env().emit_event(Transfer{
                from:None,
                to:Some(caller),
                value:initial_supply,
            });
        }

        #[ink(message)]
        fn balance_of(&self,owner:AccountId)->Balance{
            self.balance_of_or_zero(&owner)
        }

        fn balance_of_or_zero(&self,owner:&AccountId)->Balance{
            //判断有没有初始化，没有则设置0
            *self.balance.get(owner).unwrap_or(&0)
        }

        #[ink(message)]
        fn transfer(&mut self,to:AccountId,value:Balance)->bool{
            let from=self.env().caller();
            let from_balance=self.balance_of_or_zero(&from);
            if from_balance < value{
                return false;
            }
            let to_balance=self.balance_of_or_zero(&to);
            self.balance.insert(from,from_balance-value);
            self.balance.insert(to,to_balance+value);
            self.env().emit_event(Transfer{
                from:Some(from),
                to:Some(to),
                value,
            });
            true
        }


        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            true
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner, &spender)
        }



        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn new_works() {
            let erc20 = Erc20::new(666);
            assert_eq!(erc20.total_supply,666);
        }

        #[test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply, 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

    }


}
