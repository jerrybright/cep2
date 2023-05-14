#![cfg_attr(not(feature = "std"), no_std)]

// we need to build a group of user
// Only sudo is able to add the user
// Only sudo is able to remove the user.
// Users can be able to add themselves in the group if the group size is less than 2.



// Clubmember -> alice, bob,
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubmember)]
	pub type ClubMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MemberAdded,
		MemberRemoved,
	}

	// Error inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyMember,
		NotMember,
		CannotRemoveOtherMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

			club_members.insert(location, who.clone());

			ClubMembers::<T>::put(&club_members);

			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;

			club_members.remove(location);

			ClubMembers::<T>::put(&club_members);

			Self::deposit_event(Event::MemberRemoved);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_member_by_yourself(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			let check_account = ensure_signed(origin.clone())?;

				ensure!(who == check_account, Error::<T>::CannotRemoveOtherMember);
				let mut club_members = ClubMembers::<T>::get();
				let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;

				club_members.remove(location);

				ClubMembers::<T>::put(&club_members);

				Self::deposit_event(Event::MemberRemoved);
				Ok(())
		}
		

    	// Add a member to the group if the group size is less than 2.
    	#[pallet::weight(10_000)]
    	pub fn add_self_to_group(origin: OriginFor<T>) -> DispatchResult {
        	let who = ensure_signed(origin.clone())?;

        // Get the current group members.
        	let mut club_members = ClubMembers::<T>::get();

        // Check if the user is already a member of the group.
        	let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

        // Check if the group size is less than 2.
        	ensure!(club_members.len() < 2, "Group is full");

        // Add the user to the group.
        	club_members.push(who.clone());

        	ClubMembers::<T>::put(&club_members);

        	Self::deposit_event(Event::MemberAdded);

        	Ok(())
    }

}



	}