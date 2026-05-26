#!/usr/bin/env bash
# Package true-suite evidence git stores into committable tarballs.
#
# Going-forward evidence may contain nested Git stores such as:
#   <domain>/runtime_repo/.git
#   <domain>/cas/.git
#   <domain>/tdma_tape.git
#
# Git will not track nested .git directories as ordinary evidence. This script
# converts those stores into deterministic tar.gz archives and removes the loose
# stores after the archive is safely written. It never rewrites old evidence
# unless explicitly pointed at that run root by the caller.

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_ID=""
RUN_ROOT=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --run-id)
            RUN_ID="${2:?--run-id requires a value}"
            shift 2
            ;;
        --run-root)
            RUN_ROOT="${2:?--run-root requires a value}"
            shift 2
            ;;
        -h|--help)
            cat <<'EOF'
package_true_suite_evidence.sh --run-id <RUN_ID>
package_true_suite_evidence.sh --run-root <PATH>

Packages true-suite evidence into deterministic tar.gz files:
  runtime_repo/.git      -> runtime_repo.dotgit.tar.gz
  runtime_repo/ sidecars -> runtime_repo.worktree.tar.gz
  cas/.git               -> cas.dotgit.tar.gz
  cas/ sidecars          -> cas.worktree.tar.gz
  tdma_tape.git/         -> tdma_tape.git.tar.gz

Writes <run-root>/evidence_package_manifest.json.
EOF
            exit 0
            ;;
        *)
            RUN_ID="${1#handover/evidence/true_suite/}"
            shift
            ;;
    esac
done

if [[ -z "$RUN_ROOT" ]]; then
    if [[ -z "$RUN_ID" ]]; then
        echo "ERROR: provide --run-id or --run-root" >&2
        exit 2
    fi
    RUN_ID="${RUN_ID#handover/evidence/true_suite/}"
    RUN_ROOT="$PROJECT_ROOT/handover/evidence/true_suite/$RUN_ID"
fi

if [[ ! -d "$RUN_ROOT" ]]; then
    echo "ERROR: run root is not a directory: $RUN_ROOT" >&2
    exit 3
fi

python3 - "$RUN_ROOT" <<'PY'
import hashlib
import gzip
import json
import os
import shutil
import sys
import tarfile
from pathlib import Path

run_root = Path(sys.argv[1]).resolve()
manifest_path = run_root / "evidence_package_manifest.json"


def rel(path: Path) -> str:
    try:
        return path.resolve().relative_to(run_root).as_posix()
    except ValueError:
        return path.resolve().as_posix()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def sorted_tree(root: Path, skip_dir_names=None):
    skip_dir_names = set(skip_dir_names or [])
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(name for name in dirnames if name not in skip_dir_names)
        filenames.sort()
        current = Path(dirpath)
        yield current
        for name in filenames:
            yield current / name


def clean_info(info: tarfile.TarInfo) -> tarfile.TarInfo:
    info.uid = 0
    info.gid = 0
    info.uname = ""
    info.gname = ""
    info.mtime = 0
    return info


def add_path(tf: tarfile.TarFile, source: Path, arcname: str) -> None:
    st = source.stat()
    info = tarfile.TarInfo(arcname)
    info.mode = st.st_mode & 0o777
    info = clean_info(info)
    if source.is_dir():
        info.type = tarfile.DIRTYPE
        tf.addfile(info)
    elif source.is_file():
        info.size = st.st_size
        with source.open("rb") as f:
            tf.addfile(info, f)


def package_tree(
    source: Path,
    archive: Path,
    arc_prefix: str,
    remove_source: bool,
    skip_dir_names=None,
):
    if archive.exists():
        raise SystemExit(f"archive already exists, refusing overwrite: {archive}")
    tmp = archive.with_suffix(archive.suffix + ".tmp")
    if tmp.exists():
        tmp.unlink()
    with tmp.open("wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as gz:
            with tarfile.open(fileobj=gz, mode="w") as tf:
                for item in sorted_tree(source, skip_dir_names=skip_dir_names):
                    if item == source:
                        if arc_prefix:
                            add_path(tf, item, arc_prefix)
                        continue
                    item_rel = item.relative_to(source).as_posix()
                    arcname = f"{arc_prefix}/{item_rel}" if arc_prefix else item_rel
                    add_path(tf, item, arcname)
    tmp.replace(archive)
    if remove_source:
        shutil.rmtree(source)


def has_packable_payload(source: Path, skip_dir_names=None) -> bool:
    for item in sorted_tree(source, skip_dir_names=skip_dir_names):
        if item != source and item.is_file():
            return True
    return False


def archive_metadata(archive: Path):
    name = archive.name
    parent = archive.parent
    if name == "runtime_repo.dotgit.tar.gz":
        kind = "runtime_repo_dotgit"
        removed_loose_store = True
        restore_into = rel(parent / "runtime_repo")
        source_path = rel(parent / "runtime_repo/.git")
    elif name == "runtime_repo.worktree.tar.gz":
        kind = "runtime_repo_worktree"
        removed_loose_store = False
        restore_into = rel(parent / "runtime_repo")
        source_path = rel(parent / "runtime_repo")
    elif name == "cas.dotgit.tar.gz":
        kind = "cas_dotgit"
        removed_loose_store = True
        restore_into = rel(parent / "cas")
        source_path = rel(parent / "cas/.git")
    elif name == "cas.worktree.tar.gz":
        kind = "cas_worktree"
        removed_loose_store = False
        restore_into = rel(parent / "cas")
        source_path = rel(parent / "cas")
    elif name == "tdma_tape.git.tar.gz":
        kind = "tdma_tape_git"
        removed_loose_store = True
        restore_into = rel(parent / "tdma_tape.git")
        source_path = rel(parent / "tdma_tape.git")
    else:
        return None
    return {
        "archive_bytes": archive.stat().st_size,
        "archive_path": rel(archive),
        "archive_sha256": sha256_file(archive),
        "kind": kind,
        "removed_loose_store": removed_loose_store,
        "restore_into": restore_into,
        "source_path": source_path,
    }


for store in sorted(run_root.rglob("*")):
    if not store.is_dir() or store.name not in {"runtime_repo", "cas"}:
        continue
    archive = store.parent / f"{store.name}.worktree.tar.gz"
    if archive.exists():
        continue
    if has_packable_payload(store, skip_dir_names={".git"}):
        package_tree(
            store,
            archive,
            "",
            False,
            skip_dir_names={".git"},
        )

for dotgit in sorted(run_root.rglob(".git")):
    if not dotgit.is_dir():
        continue
    parent = dotgit.parent
    if parent.name not in {"runtime_repo", "cas"}:
        continue
    archive = parent.parent / f"{parent.name}.dotgit.tar.gz"
    package_tree(
        dotgit,
        archive,
        ".git",
        True,
    )

for tdma in sorted(run_root.rglob("tdma_tape.git")):
    if not tdma.is_dir():
        continue
    archive = tdma.with_name("tdma_tape.git.tar.gz")
    package_tree(
        tdma,
        archive,
        "",
        True,
    )

packages = []
for archive in sorted(run_root.rglob("*.tar.gz")):
    metadata = archive_metadata(archive)
    if metadata is not None:
        packages.append(metadata)

manifest = {
    "package_count": len(packages),
    "packages": packages,
    "restore_notes": [
        "Extract runtime_repo.worktree.tar.gz into the corresponding runtime_repo directory.",
        "Extract runtime_repo.dotgit.tar.gz into the corresponding runtime_repo directory.",
        "Extract cas.worktree.tar.gz into the corresponding cas directory.",
        "Extract cas.dotgit.tar.gz into the corresponding cas directory.",
        "Create tdma_tape.git then extract tdma_tape.git.tar.gz into it.",
    ],
    "run_root": str(run_root),
    "schema_version": "turingosv4.true_suite.evidence_package_manifest.v1",
}
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"TRUE-SUITE packaged evidence stores: {manifest_path} ({len(packages)} packages)")
PY
