use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

pub struct Transaction<AccountId> {
    from: AccountId,
    to: AccountId,
}

decl_storage! {
    trait Store for Module<T: Trait> as VirtueStorage {
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here

        fn send_transaction(origin, to: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;

            let tx = Transaction { from: sender, to: to };

            Ok(())
        }
    }
}