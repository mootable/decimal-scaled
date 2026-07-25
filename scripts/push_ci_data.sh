#!/usr/bin/env bash
#
# push_ci_data.sh — publish one generated data file to the `ci-data` branch.
#
#   Usage: scripts/push_ci_data.sh <src-file> [<dest-path-in-branch>]
#          (dest defaults to <src-file> — every caller keeps the same layout)
#
# WHY THIS EXISTS
#   The timing / correctness producers (bench-branch-compare, golden-
#   comprehensive, history, lib-perf) refresh a TSV under results/ on every
#   run. On `main` they cannot self-commit it: `main` is a protected branch and
#   a direct push is declined (GH006). `ci-data` is an unprotected, data-only
#   ORPHAN branch that holds nothing but those refreshed TSVs — the producers
#   push here instead, and a later docs build reads from it.
#
#   Loop-safe: a GITHUB_TOKEN push does NOT trigger workflows, and no workflow
#   is configured to run on `ci-data`, so publishing data never starts another
#   run. Each producer writes a DISJOINT file, so concurrent runs don't collide
#   on content; the push retry below absorbs the non-fast-forward races.
#
# Isolated by construction: all work happens in a throwaway worktree, so the
# caller's checkout (its ref, its build tree) is never touched.
set -euo pipefail

src="${1:?usage: push_ci_data.sh <src-file> [<dest-path>]}"
dest="${2:-$src}"
branch="${CI_DATA_BRANCH:-ci-data}"

if [ ! -f "$src" ]; then
  echo "push_ci_data: '$src' does not exist — nothing to publish"
  exit 0
fi

git config user.name  "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

wt="$(mktemp -d)"
cleanup() { git worktree remove --force "$wt" >/dev/null 2>&1 || rm -rf "$wt"; }
trap cleanup EXIT

# Materialise `ci-data` in the throwaway worktree: track the remote branch if
# it already exists, otherwise start it as an EMPTY orphan (no main history).
if git fetch --depth 1 origin "$branch:refs/remotes/origin/$branch" 2>/dev/null; then
  git worktree add -B "$branch" "$wt" "refs/remotes/origin/$branch" >/dev/null
else
  echo "push_ci_data: '$branch' absent on origin — creating it as an orphan"
  git worktree add --detach "$wt" >/dev/null
  git -C "$wt" checkout --orphan "$branch" >/dev/null 2>&1
  git -C "$wt" rm -rf . >/dev/null 2>&1 || true
fi

mkdir -p "$wt/$(dirname "$dest")"
cp "$src" "$wt/$dest"
git -C "$wt" add "$dest"

if git -C "$wt" diff --cached --quiet; then
  echo "push_ci_data: '$dest' unchanged on '$branch' — nothing to publish"
  exit 0
fi

git -C "$wt" commit -q -m "data: refresh $dest"

# Retry to absorb a sibling producer landing its own (disjoint) file first.
for attempt in 1 2 3 4 5; do
  if git -C "$wt" push origin "HEAD:$branch" 2>&1; then
    echo "push_ci_data: published '$dest' to '$branch'"
    exit 0
  fi
  echo "push_ci_data: push rejected (attempt $attempt/5) — rebasing on '$branch'"
  git -C "$wt" fetch --depth 1 origin "$branch:refs/remotes/origin/$branch" || true
  git -C "$wt" rebase "refs/remotes/origin/$branch" || git -C "$wt" rebase --abort || true
done

echo "push_ci_data: FAILED to publish '$dest' after 5 attempts" >&2
exit 1
