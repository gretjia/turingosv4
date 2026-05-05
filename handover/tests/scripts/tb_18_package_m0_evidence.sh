#!/usr/bin/env bash
# TB-18 M0 retry evidence packaging — applies EvidencePackagingPolicy §3 +
# §3.1 to a completed M0 batch directory. Tars `.git` dirs in
# runtime_repo/ and cas/ subdirectories of every per-problem dir; produces
# `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per packaged run.
#
# Tamper subdirectories are tarred similarly (preserving full restorable
# evidence per architect §2.3 + TB-7R/TB-8/TB-9 precedent).
#
# Per `feedback_evidence_packaging_policy_required` + TB-18 charter §1.4
# SG-18.13: M0 = FULL restorable evidence for every problem.
#
# Usage:
#   bash handover/tests/scripts/tb_18_package_m0_evidence.sh \
#     handover/evidence/tb_18_m0_retry_2026-05-05/r1
#
# Exit codes:
#   0 — all per-problem directories packaged (or already packaged)
#   1 — invalid args / target dir missing
#   2 — tar failure on any subdirectory

set -euo pipefail

TARGET="${1:-}"
if [ -z "$TARGET" ]; then
  echo "ERROR: usage: $0 <target-dir>" >&2
  exit 1
fi
if [ ! -d "$TARGET" ]; then
  echo "ERROR: target dir not found: $TARGET" >&2
  exit 1
fi

PACKAGED=0
SKIPPED=0

# Top-level per-problem dirs (P0X_<problem>)
for prob_dir in "$TARGET"/P*/; do
  [ -d "$prob_dir" ] || continue
  for sub in runtime_repo cas; do
    if [ -d "$prob_dir$sub/.git" ]; then
      tarball="${prob_dir}${sub}.dotgit.tar.gz"
      if [ -f "$tarball" ]; then
        echo "[skip] $tarball already exists"
        SKIPPED=$((SKIPPED + 1))
        continue
      fi
      echo "[tar] $prob_dir$sub/.git → $tarball"
      tar -czf "$tarball" -C "$prob_dir$sub" .git
      PACKAGED=$((PACKAGED + 1))
      # Per TB-18 H0 evidence pattern: delete .git after tar to avoid git
      # submodule semantics on parent repo `git add`.
      rm -rf "$prob_dir$sub/.git"
    fi
  done

  # Tamper subdirs (each has runtime_repo/.git + cas/.git)
  for tamper_dir in "$prob_dir"tamper/*/; do
    [ -d "$tamper_dir" ] || continue
    for sub in runtime_repo cas; do
      if [ -d "$tamper_dir$sub/.git" ]; then
        tarball="${tamper_dir}${sub}.dotgit.tar.gz"
        if [ -f "$tarball" ]; then
          continue
        fi
        tar -czf "$tarball" -C "$tamper_dir$sub" .git
        PACKAGED=$((PACKAGED + 1))
        # Tamper subdirs we leave .git in place (working tree may need it
        # for replay) AND keep tarball — gitignore handles double-handling.
      fi
    done
  done
done

echo ""
echo "[tb_18_package_m0_evidence] complete: $PACKAGED tarballs created; $SKIPPED already-tarred skipped"
