#!/bin/sh
#
# Originally from Godot Engine project (MIT License)
#

die() {
    echo " *** ERROR: " $*
    exit 1
}

CLANG_FORMAT_VERSION="clang-format version 3.9"

if which clang-format-3.9 > /dev/null; then
    alias clang-format=clang-format-3.9
elif which clang-format > /dev/null; then
    case "$(clang-format --version)" in
        "$CLANG_FORMAT_VERSION"*)
            ;;
        *)
            die "clang-format 3.9 required"
            ;;
    esac
else
    die "$CLANG_FORMAT_VERSION required"
fi

if [ "$TRAVIS_PULL_REQUEST" != "false" ]; then
    # Check the whole commit range against $TRAVIS_BRANCH, the base merge branch
    # We could use $TRAVIS_COMMIT_RANGE but it doesn't play well with force pushes
    RANGE="$(git rev-parse $TRAVIS_BRANCH) HEAD"
else
    # Test only the last commit
    RANGE=HEAD
fi

FILES=$(git diff-tree --no-commit-id --name-only -r $RANGE | grep -v mpi/ | grep -E "\.(c|h|cpp|hpp|cc|hh|cxx|m|mm|inc|java|glsl)$")
echo "Checking files:\n$FILES"

# create a random filename to store our generated patch
prefix="static-check-clang-format"
suffix="$(date +%s)"
patch="/tmp/$prefix-$suffix.patch"

for file in $FILES; do
    clang-format -style=Mozilla "$file" | \
        diff -u "$file" - | \
        sed -e "1s|--- |--- a/|" -e "2s|+++ -|+++ b/$file|" >> "$patch"
done

# if no patch has been generated all is ok, clean up the file stub and exit
if [ ! -s "$patch" ] ; then
    printf "Files in this commit comply with the clang-format rules.\n"
    rm -f "$patch"
    exit 0
fi

# a patch has been created, notify the user and exit
printf "\n*** The following differences were found between the code to commit "
printf "and the clang-format rules:\n\n"
cat "$patch"
printf "\n*** Aborting, please fix your commit(s) with 'git commit --amend' or 'git rebase -i <hash>'\n"
exit 1
