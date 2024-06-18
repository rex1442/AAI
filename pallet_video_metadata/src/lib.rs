#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MetadataStored(T::AccountId, Vec<u8>, u64, u64, Vec<u8>),
    }

    #[pallet::storage]
    #[pallet::getter(fn video_metadata)]
    pub type VideoMetadata<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, Vec<u8>, u64, u64, Vec<u8>), OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn store_metadata(origin: OriginFor<T>, video_url: Vec<u8>, latitude: u64, longitude: u64, timestamp: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let metadata_hash = T::Hashing::hash_of(&(video_url.clone(), latitude, longitude, timestamp.clone()));

            <VideoMetadata<T>>::insert(metadata_hash, (&who, video_url.clone(), latitude, longitude, timestamp.clone()));

            Self::deposit_event(Event::MetadataStored(who, video_url, latitude, longitude, timestamp));
            Ok(())
        }
    }
}
