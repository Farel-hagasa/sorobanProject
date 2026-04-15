# ⏳ Memory Capsule DApp

**Memory Capsule DApp** – Time-Locked & Recipient-Based Messaging on Stellar Blockchain

---

## 🚀 Project Description

Memory Capsule DApp is a decentralized smart contract application built on the Stellar blockchain using the Soroban SDK. It enables users to create messages that are **locked in time** and can only be accessed after a specified future timestamp.

Unlike traditional note-taking applications, this system introduces a **time-based and recipient-based messaging mechanism**, where users can send messages to themselves or others that can only be opened in the future.

This ensures that messages remain secure, tamper-proof, and inaccessible until the exact moment defined by the creator.

---

## 🌌 Project Vision

Our vision is to transform how people interact with digital memories and communication by introducing **time-delayed messaging on blockchain**.

We aim to:

* ⏳ Introduce **time-based data access control**
* 👤 Enable **future communication between users**
* 🔐 Ensure **trustless and secure message delivery**
* 📦 Preserve digital messages in an immutable environment
* 💡 Create meaningful experiences beyond simple data storage

We believe blockchain can be used not only for transactions, but also for **preserving moments, intentions, and messages across time**.

---

## 🔥 Key Features

### 1. ⏳ Time-Locked Capsule Creation

* Create a capsule with a custom unlock timestamp
* Message is locked until the specified time
* Securely stored on-chain

---

### 2. 👤 Recipient-Based Messaging (NEW 🔥)

* Send capsules to another wallet address
* Only the **recipient** can unlock the message
* Enables “future messaging” between users

---

### 3. 🔒 Locked Message Protection

* Capsules cannot be accessed before unlock time
* Fully enforced by smart contract logic
* Prevents early access or manipulation

---

### 4. 🔓 Secure Capsule Unlocking

* Capsule can only be unlocked:

  * After the unlock time ⏳
  * By the intended recipient 👤
* Guarantees fairness and security

---

### 5. 👁️ Safe Capsule Viewing

* If accessed before unlock time → returns “locked”
* If unlocked → returns actual message
* Ensures controlled data visibility

---

### 6. 📊 Capsule Status Tracking (NEW 🔥)

* Check capsule status:

  * `"Locked"` 🔒
  * `"Unlocked"` 🔓
* Real-time status based on blockchain timestamp

---

### 7. 🗑️ Capsule Management

* Delete capsules securely
* Efficient storage handling

---

### 8. 📈 Capsule Statistics

* Count total capsules stored
* Lightweight analytics directly from contract

---

## ⚙️ Contract Details

* Contract Address: *(Isi setelah deploy)*
* Network: Stellar Testnet (Soroban)

---

## 🧪 How It Works

1. User creates a capsule with:

   * message
   * unlock time
   * recipient address

2. Capsule is stored on blockchain in **locked state** 🔒

3. Attempt to open before time → ❌ rejected

4. Only the **recipient** can unlock the capsule

5. After time is reached → ✅ capsule can be opened

---

## 🛠️ Technology Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## ▶️ Getting Started

### Deploy Contract

```id="g3j2dk"
cargo build --target wasm32-unknown-unknown --release
```

```id="q0a9k2"
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/<nama_file>.wasm \
--source alice \
--network testnet
```

---

## 🔧 Contract Functions

* `create_capsule(message, unlock_time, recipient)`
* `get_capsules()`
* `unlock_capsule(id)`
* `view_capsule(id)`
* `get_capsule_status(id)`
* `delete_capsule(id)`
* `count_capsules()`

---


## 🚀 Future Scope

### Short-Term

* 🔐 Message encryption
* 👤 Multi-user capsule ownership
* ⏱️ Countdown timer UI

---

### Medium-Term

* 🔔 Notification when unlocked
* 🤝 Shared capsules (multi-recipient)
* 🏷️ Tag & category system

---

### Long-Term

* 🌐 Cross-chain capsule support
* 📦 IPFS decentralized frontend
* 🤖 AI-powered reflection on messages
* 🔐 Zero-knowledge privacy layer

---

## 🎯 Unique Value Proposition

* ⏳ Time-based access control
* 👤 Recipient-based message delivery
* 🔐 Trustless & immutable system
* 💡 Emotional + functional use case
* 🚀 Beyond CRUD (real blockchain logic)

---


## 🏆 Project Goal

This project demonstrates how blockchain can be used to create **time-controlled and recipient-based communication systems**, enabling new ways to store and deliver messages securely across time.

---

**Memory Capsule DApp** – Send Messages Through Time & Space 🚀
