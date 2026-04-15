#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// =======================
// STRUCT
// =======================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Capsule {
    pub id: u64,
    pub message: String,
    pub unlock_time: u64,
    pub is_unlocked: bool,
}

// =======================
// STORAGE KEY
// =======================
const CAPSULE_DATA: Symbol = symbol_short!("CAPSULE");

// =======================
// CONTRACT
// =======================
#[contract]
pub struct MemoryCapsuleContract;

// =======================
// IMPLEMENTATION
// =======================
#[contractimpl]
impl MemoryCapsuleContract {

    // =======================
    // GET ALL CAPSULES
    // =======================
    pub fn get_capsules(env: Env) -> Vec<Capsule> {
        return env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // =======================
    // CREATE CAPSULE
    // =======================
    pub fn create_capsule(env: Env, message: String, unlock_time: u64) -> String {

        let mut capsules: Vec<Capsule> = env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));

        let capsule = Capsule {
            id: env.prng().gen::<u64>(),
            message: message,
            unlock_time: unlock_time,
            is_unlocked: false,
        };

        capsules.push_back(capsule);

        env.storage().instance().set(&CAPSULE_DATA, &capsules);

        return String::from_str(&env, "Capsule berhasil dibuat");
    }

    // =======================
    // UNLOCK CAPSULE
    // =======================
    pub fn unlock_capsule(env: Env, id: u64) -> String {

        let mut capsules: Vec<Capsule> = env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));

        let current_time = env.ledger().timestamp();

        for i in 0..capsules.len() {
            let mut capsule = capsules.get(i).unwrap();

            if capsule.id == id {

                if current_time < capsule.unlock_time {
                    return String::from_str(&env, "Capsule belum bisa dibuka");
                }

                if capsule.is_unlocked {
                    return String::from_str(&env, "Capsule sudah dibuka sebelumnya");
                }

                capsule.is_unlocked = true;

                capsules.set(i, capsule);
                env.storage().instance().set(&CAPSULE_DATA, &capsules);

                return String::from_str(&env, "Capsule berhasil dibuka");
            }
        }

        return String::from_str(&env, "Capsule tidak ditemukan");
    }

    // =======================
    // VIEW CAPSULE (SAFE VIEW)
    // =======================
    pub fn view_capsule(env: Env, id: u64) -> String {

        let capsules: Vec<Capsule> = env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));

        let current_time = env.ledger().timestamp();

        for i in 0..capsules.len() {
            let capsule = capsules.get(i).unwrap();

            if capsule.id == id {

                if current_time < capsule.unlock_time {
                    return String::from_str(&env, "Capsule masih terkunci");
                }

                return capsule.message;
            }
        }

        return String::from_str(&env, "Capsule tidak ditemukan");
    }

    // =======================
    // DELETE CAPSULE
    // =======================
    pub fn delete_capsule(env: Env, id: u64) -> String {

        let mut capsules: Vec<Capsule> = env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..capsules.len() {
            if capsules.get(i).unwrap().id == id {
                capsules.remove(i);

                env.storage().instance().set(&CAPSULE_DATA, &capsules);

                return String::from_str(&env, "Capsule berhasil dihapus");
            }
        }

        return String::from_str(&env, "Capsule tidak ditemukan");
    }

    // =======================
    // COUNT CAPSULES 
    // =======================
    pub fn count_capsules(env: Env) -> u32 {
        let capsules: Vec<Capsule> = env.storage()
            .instance()
            .get(&CAPSULE_DATA)
            .unwrap_or(Vec::new(&env));

        return capsules.len();
    }
}