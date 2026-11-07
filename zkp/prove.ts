// zkp/prove.ts — EIN SOF empatia-ZKP (snarkjs / Groth16)
// Copyright © 2026 Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X)
import { groth16 } from "snarkjs";

async function proveEmpathy(
  empathyScore: number,       // 0–1
  threshold: number = 0.5,    // vaadittu minimi
  salt: bigint,
): Promise<{ proof: any; publicSignals: any }> {
  const input = {
    empathy: Math.round(empathyScore * 10000),
    threshold: Math.round(threshold * 10000),
    salt: salt.toString(),
  };
  return await groth16.fullProve(
    input,
    "empathy.wasm",
    "empathy_final.zkey"
  );
}

// Lohkoketjuun menee vain: proof + publicSignals
// Ei koskaan: empathy, salt, HRV, ihokonduktanssi
