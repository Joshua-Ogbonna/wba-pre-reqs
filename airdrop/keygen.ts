import { Keypair } from "@solana/web3.js";

// Generate new keypair
let kp = Keypair.generate()
console.log("New solana wallet generated: ", kp.publicKey.toBase58())
console.log("Wallet", kp.secretKey)

