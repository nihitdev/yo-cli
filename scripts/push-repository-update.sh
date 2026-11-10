#!/usr/bin/env bash
# Publish one already committed repository update without overwriting other work.
set -euo pipefail
repository_path=${1:?usage: push-repository-update.sh REPOSITORY_PATH}
: "${BRANCH:?set the destination branch}"
case "$repository_path" in
  site/public/apt|site/public/alpine|site/public/opensuse) ;;
  *) echo 'Unexpected repository path' >&2; exit 1 ;;
esac

git diff --quiet
git diff --cached --quiet
for attempt in 1 2 3 4 5; do
  base=$(git rev-parse HEAD^)
  git fetch --no-tags origin "refs/heads/$BRANCH"
  # Never merge independently signed metadata from the same repository.
  if ! git diff --quiet "$base" FETCH_HEAD -- "$repository_path"; then
    echo "Remote $repository_path changed during publication; rerun to regenerate against the latest branch." >&2
    exit 1
  fi
  if ! git rebase --onto FETCH_HEAD "$base"; then
    git rebase --abort
    echo 'Concurrent changes conflict with this update; rerun publication.' >&2
    exit 1
  fi
  if git push origin "HEAD:refs/heads/$BRANCH"; then
    exit 0
  fi
  echo "Publication push attempt $attempt failed; refreshing the branch." >&2
done
echo 'Publication push failed after five attempts; no force push was attempted.' >&2
exit 1
