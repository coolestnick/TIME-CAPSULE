#[macro_use]
extern crate serde;
use candid::{Decode, Encode};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TimeCapsule {
    id: u64,
    content: String, // This can be extended to support multiple types like photos, videos, notes, etc.
    unlock_date: u64, // Timestamp for when the capsule should be unlocked
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TimeCapsulePayload {
    content: String,
    unlock_date: u64,
}

impl Storable for TimeCapsule {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TimeCapsule {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Existing thread-local variables

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static TIME_CAPSULE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for water usage items")
    );

    static TIME_CAPSULE_STORAGE: RefCell<StableBTreeMap<u64, TimeCapsule, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));


}

// Helper method to perform insert for TimeCapsule
fn do_insert_time_capsule(item: &TimeCapsule) {
    TIME_CAPSULE_STORAGE.with(|service| {
        service.borrow_mut().insert(item.id, item.clone());
    });
}

#[ic_cdk::query]
fn get_time_capsule(id: u64) -> Result<TimeCapsule, Error> {
    match _get_time_capsule(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("time capsule with id={} not found", id),
        }),
    }
}

fn _get_time_capsule(id: &u64) -> Option<TimeCapsule> {
    TIME_CAPSULE_STORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_time_capsule(item: TimeCapsulePayload) -> Option<TimeCapsule> {
    let id = TIME_CAPSULE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for time capsules");
    let time_capsule = TimeCapsule {
        id,
        content: item.content,
        unlock_date: item.unlock_date,
    };
    do_insert_time_capsule(&time_capsule);
    Some(time_capsule)
}

#[ic_cdk::update]
fn update_time_capsule(id: u64, item: TimeCapsulePayload) -> Result<TimeCapsule, Error> {
    match TIME_CAPSULE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut time_capsule) => {
            time_capsule.content = item.content;
            time_capsule.unlock_date = item.unlock_date;
            do_insert_time_capsule(&time_capsule);
            Ok(time_capsule)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update time capsule with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_time_capsule(id: u64) -> Result<TimeCapsule, Error> {
    match TIME_CAPSULE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(time_capsule) => Ok(time_capsule),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete time capsule with id={}. item not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

#[ic_cdk::query]
fn get_time_capsules_before_date(unlock_date: u64) -> Vec<TimeCapsule> {
    TIME_CAPSULE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, capsule)| capsule.unlock_date <= unlock_date)
            .map(|(_, capsule)| capsule.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_unlock_date(id: u64, new_unlock_date: u64) -> Result<TimeCapsule, Error> {
    match TIME_CAPSULE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut time_capsule) => {
            time_capsule.unlock_date = new_unlock_date;
            do_insert_time_capsule(&time_capsule);
            Ok(time_capsule)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update unlock date for time capsule with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_time_capsules() -> Vec<TimeCapsule> {
    TIME_CAPSULE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_total_time_capsules() -> u64 {
    TIME_CAPSULE_STORAGE.with(|service| service.borrow().len())
}

#[ic_cdk::query]
fn get_time_capsules_count_before_date(unlock_date: u64) -> usize {
    TIME_CAPSULE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, capsule)| capsule.unlock_date <= unlock_date)
            .count()
    })
}

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
