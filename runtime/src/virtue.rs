use support::{decl_storage, decl_module, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_primitives::traits::{Hash};

use parity_codec::{Encode, Decode};

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Transaction<AccountId> {
    from: AccountId,
    to: AccountId,
}

decl_storage! {
    trait Store for Module<T: Trait> as VirtueStorage {
        // Declare storage and getter functions here
        Transactions get(get_transaction): map T::Hash => Transaction<T::AccountId>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here

        fn send_transaction(origin, to: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;

            let tx = Transaction { from: sender, to: to };
            let hash = <T as system::Trait>::Hashing::hash_of(&tx);
            <Transactions<T>>::insert(hash, tx);

            Ok(())
        }
    }
}