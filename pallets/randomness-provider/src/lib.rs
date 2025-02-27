#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{decl_module, decl_storage, traits::Randomness as RandomnessT, Parameter};

use pallet_session::SessionManager;

use sp_consensus_vrf::schnorrkel;
use sp_runtime::traits::Member;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

#[derive(Debug, PartialEq, Encode)]
pub struct BabeRandomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness: schnorrkel::Randomness,
}

pub trait Config: pallet_babe::Config + frame_system::Config {
    /// A stable ID for a validator.
    type ValidatorId: Member + Parameter;
}

decl_storage! {
    trait Store for Module<T: Config> as RandomnessExt {
        /// Historical randomness for each epoch.
        HistoricalRandomness get(fn historical_randomness): map hasher(twox_64_concat) u64 => schnorrkel::Randomness;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {}
}

/// An `SessionManager` implementation that wraps an inner `I` and also
/// sets the historical randomness of the ending session.
pub struct NoteHistoricalRandomness<T, I>(sp_std::marker::PhantomData<(T, I)>);

impl<T: Config, I> SessionManager<T::ValidatorId> for NoteHistoricalRandomness<T, I>
where
    I: SessionManager<T::ValidatorId>,
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        <I as SessionManager<_>>::new_session(new_index)
    }

    fn end_session(end_index: SessionIndex) {
        <I as SessionManager<_>>::end_session(end_index);
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        <HistoricalRandomness>::insert(epoch.epoch_index, epoch.randomness);
    }

    fn start_session(start_index: SessionIndex) {
        <I as SessionManager<_>>::start_session(start_index)
    }
}

impl<T: Config> Module<T> {
    // Return babe randomness info for current epoch
    pub fn current_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    // Return babe randomness info for next epoch
    pub fn next_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::next_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    /// Return babe randomness for historical epoch
    pub fn randomness_of(epoch: u64) -> schnorrkel::Randomness {
        <HistoricalRandomness>::get(&epoch)
    }

    /// Return randomness with provider subject
    pub fn random(subject: &[u8]) -> T::Hash {
        <pallet_babe::Module<T>>::random(subject)
    }
}
