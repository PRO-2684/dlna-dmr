#!/bin/bash

# If no arguments are passed, print the version
if [ $# -eq 0 ]; then
    # Exit with git's exit code
    git describe --tags --abbrev=0
    exit $?
fi

# If more than one argument is passed, print an error message
if [ $# -gt 1 ]; then
    echo "Error: Too many arguments"
    exit 1
fi

# Otherwise, set the version
version=$1
# Check if the version is valid
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Invalid version format. Use X.Y.Z"
    exit 1
fi
# Check if the version already exists
if git tag | grep -q "v$version"; then
    echo "Error: Version $version already exists"
    exit 1
fi
# Set version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
cargo update
# Commit the changes
git add Cargo.toml Cargo.lock
git commit -S -m "Bump version to v$version"
# Create a new tag
git tag -s v$version -m "Version $version"
