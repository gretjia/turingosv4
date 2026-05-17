#!/usr/bin/env bash
set -euo pipefail

envelope_id="${TURINGOS_RESEARCH_ENVELOPE:-}"
required_id="MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2"

if [[ "${envelope_id}" != "${required_id}" ]]; then
  echo "mode=Constitutional Research Mode"
  echo "required_envelope=${required_id}"
  echo "actual_envelope=${envelope_id:-unset}"
  echo "stop_level=Level3 Constitutional Hard Stop"
  echo "reason=missing_or_wrong_research_envelope"
  echo "action=write STOP_PROOF.md"
  exit 2
fi

for required_file in \
  "handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md" \
  "handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md" \
  "handover/directives/market_autonomy_lab/STOP_PROOF_TEMPLATE.md" \
  "handover/directives/2026-05-16_MARKET_AUTONOMY_LAB_ARCHITECT_ORIGINAL.md"
do
  if [[ ! -f "${required_file}" ]]; then
    echo "mode=Constitutional Research Mode"
    echo "envelope=${required_id}"
    echo "stop_level=Level3 Constitutional Hard Stop"
    echo "reason=missing_required_file:${required_file}"
    echo "action=write STOP_PROOF.md"
    exit 2
  fi
done

forbidden_surfaces=(
  "constitution.md"
  "handover/alignment/TRACE_FLOWCHART_MATRIX.md"
  "src/state/typed_tx.rs"
  "src/state/sequencer.rs"
  "src/bottom_white/cas/schema.rs"
  "src/kernel.rs"
  "src/bus.rs"
  "src/sdk/tools/wallet.rs"
)

mapfile -t touched_files < <(
  {
    git diff --name-only HEAD --
    git ls-files --others --exclude-standard
  } | sort -u
)

for touched in "${touched_files[@]}"; do
  for forbidden in "${forbidden_surfaces[@]}"; do
    if [[ "${touched}" == "${forbidden}" ]]; then
      echo "mode=Constitutional Research Mode"
      echo "envelope=${required_id}"
      echo "forbidden_surfaces=${forbidden_surfaces[*]}"
      echo "touched_forbidden_surface=${touched}"
      echo "stop_level=Level3 Constitutional Hard Stop"
      echo "action=write STOP_PROOF.md"
      exit 2
    fi
  done
done

if printf '%s\n' "${touched_files[@]}" | grep -qx 'genesis_payload.toml'; then
  echo "mode=Constitutional Research Mode"
  echo "envelope=${required_id}"
  echo "forbidden_surfaces=${forbidden_surfaces[*]}"
  echo "stop_level=Level2 Ratification Checkpoint"
  echo "reason=genesis_payload_touched_for_allowed_trust_root_rehash"
  echo "next=rerun Trust Root verification"
  exit 0
fi

echo "mode=Constitutional Research Mode"
echo "envelope=${required_id}"
echo "forbidden_surfaces=${forbidden_surfaces[*]}"
echo "touched_file_count=${#touched_files[@]}"
echo "stop_level=Level0 Continue"
echo "next=continue_in_envelope"

