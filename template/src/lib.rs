#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, ensure};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type MaxProofLength: Get<u8>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Claims get(fn get_claim): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId  {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		
		NewClaim(AccountId, Vec<u8>),
		ClaimRevoked(AccountId,Vec<u8>),
		CliamTransfer(AccountId, AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		ClaimNotExisted,
		ClaimAlreadyExisted,
		NotTheOwner,
		ClaimTooLang,
		
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 0]
		pub fn new_claim(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let Owner = ensure_signed(origin)?;
			ensure!(proof.len()<=T::MaxProofLength::get() as usize, Error::<T>::ClaimTooLang);
			ensure!(! Claims::<T>::contains_key(&proof), Error::<T>::ClaimAlreadyExisted);


			Claims::<T>::insert(&proof,(Owner.clone(), frame_system::Module::<T>::block_number()));

			// Emit an event.
			Self::deposit_event(RawEvent::NewClaim(Owner, proof));
			// Return a successful DispatchResult
			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, proof:Vec<u8>) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(proof.len()<=T::MaxProofLength::get() as usize, Error::<T>::ClaimTooLang);
			ensure!(Claims::<T>::contains_key(&proof), Error::<T>::ClaimNotExisted);

			let (owner, _) = Self::get_claim(&proof);
			
			ensure!(owner==sender.clone(), Error::<T>::NotTheOwner);

			Claims::<T>::remove(&proof);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));

			Ok(())
		}

		#[weight = 0]
		pub fn transfer_claim(origin, proof:Vec<u8>, dest: <T as frame_system::Trait>::AccountId) -> dispatch::DispatchResult{
			let sender = ensure_signed(origin)?;
			ensure!(proof.len()<=T::MaxProofLength::get() as usize, Error::<T>::ClaimTooLang);
			ensure!(Claims::<T>::contains_key(&proof), Error::<T>::ClaimNotExisted);

			let (owner, _) = Self::get_claim(&proof);
			
			ensure!(owner==sender.clone(), Error::<T>::NotTheOwner);

			Claims::<T>::insert(&proof,(dest.clone(), frame_system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::NewClaim(dest, proof));
			Ok(())
		}

		
	}
}
