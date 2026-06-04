#!/usr/bin/env bash
set -euo pipefail

echo "Usage: fund-accounts.sh <address>"
echo "Funding $1..."
curl -s "https://friendbot.stellar.org?addr=$1" | head -c 200
echo
