#!/usr/bin/env bash
set -euo pipefail

# SoroQuest Bounty CLI Interaction Helper
# Helper wrapper for invoking SoroQuest contract methods using stellar-cli

CONTRACT_ID="${CONTRACT_ID:-CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV}"
NETWORK="${NETWORK:-testnet}"

usage() {
    cat <<EOF
SoroQuest CLI Tool

Usage:
  $0 post <owner_address> <title> <description> <amount_stroops> <token_address> [deadline]
  $0 claim <claimant_address> <bounty_id>
  $0 approve <owner_address> <bounty_id>
  $0 cancel <owner_address> <bounty_id>
  $0 get <bounty_id>
  $0 list [start_id] [limit]

Examples:
  $0 get 1
  $0 list 0 10
EOF
    exit 1
}

if [ $# -lt 1 ]; then
    usage
fi

COMMAND="$1"
shift

case "$COMMAND" in
    get)
        BOUNTY_ID="${1:?Missing bounty ID}"
        echo "Querying bounty #$BOUNTY_ID on $NETWORK ($CONTRACT_ID)..."
        stellar contract invoke \
            --id "$CONTRACT_ID" \
            --network "$NETWORK" \
            -- \
            get_bounty --bounty_id "$BOUNTY_ID"
        ;;
    list)
        START="${1:-0}"
        LIMIT="${2:-20}"
        echo "Listing bounties from $START (limit $LIMIT)..."
        stellar contract invoke \
            --id "$CONTRACT_ID" \
            --network "$NETWORK" \
            -- \
            list_bounties --status 0 --start "$START" --limit "$LIMIT"
        ;;
    *)
        echo "Command: $COMMAND (Requires active signer session)"
        echo "Please execute via Freighter on https://soroquest-app.vercel.app or configure local stellar identity."
        ;;
esac
