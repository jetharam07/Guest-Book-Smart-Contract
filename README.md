# 📝 Solana Guestbook Smart Contract (Solana Anchor)

This repository contains the core smart contract logic for a **Guestbook dApp** built using the Anchor framework on **Solana**.  
The contract allows users to **write entries** with their name and message, which are stored on-chain.

---

## 📺 YouTube Video

🎥 I’ve made a full walkthrough of this Guestbook smart contract:  
👉 **Watch here:** *Guestbook Smart Contract on Solana Playground*  
🔗 *(Paste your YouTube video link here)*

---
## 🔍 Explorer Link

You can view the deployed Guestbook smart contract on **Solana Explorer**:

🔗 [Guestbook Program on Solana Explorer (Devnet)](https://explorer.solana.com/address/Cf2bnoyUYDxN5e6jAJzKXgvQpFrxhwSe8LLKjvPchPXR?ref=hub.despread.io&cluster=devnet)

> **Program ID:** `Cf2bnoyUYDxN5e6jAJzKXgvQpFrxhwSe8LLKjvPchPXR`
---

## 📌 Key Points

- ✅ Written in Rust using Anchor  
- ✅ Deployed & tested directly on **Solana Playground**  
- ❌ No frontend or full Anchor project structure included  
- 🧪 Testing done without integration or frontend (not deployed via local Anchor CLI)  

---

## 🛠️ Technologies Used

- Solana  
- Anchor framework  
- Solana Playground IDE  

---

## 📁 Repo Contents

- `lib.rs` – Contains only the smart contract logic (Guestbook program).  
  This is a minimal setup just to demonstrate and test the core functionality on Playground.

---

## 🔗 Related Project

If you want to see the **full Anchor project structure** (with workspace, program, etc.), check out my earlier project:

🔗 [Vote App Repo](https://github.com/jetharam07/Vote-app-smart-contract-solana)  
It includes complete setup even though frontend and testing weren’t done.

---

## 💡 Features

- Initialize a counter account per user (`InitCounter`)  
- Add multiple entries using a PDA per entry (`WriteEntry`)  
- Entries are uniquely stored using the author’s pubkey and counter  

---

## 🧠 Note

Feel free to **clone** this repo to explore the core logic.  
Since this repo only contains the `lib.rs` file (not the full Anchor project), if you need a complete project structure, check out my [Vote App Repo](https://github.com/jetharam07/Vote-app-smart-contract-solana) and use it as a base for your Solana learning projects!  

If you have any questions, drop a comment on the video 🚀
