#!/usr/bin/env bash
set -euo pipefail

solana airdrop --keypair keypairs/admin.json 100

anchor build

for prog in "snowflake"; do
  solana program deploy \
    --program-id keypairs/deploy-key-$prog.json \
    --keypair keypairs/admin.json \
    target/deploy/$(sed 's/-/_/g' <<< "$prog").so
done
