#!/bin/bash
# build.sh - Build with semantic versioning

set -e

# Get commit message for version determination
COMMIT_MSG=$(git log -1 --pretty=format:"%s" 2>/dev/null || echo "fix: build")

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"

# Parse version components
MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)

# Determine bump type from commit message
if echo "$COMMIT_MSG" | grep -qE "^[a-z]+(\(.+\))?!:|BREAKING CHANGE"; then
    echo "Breaking change detected - bumping MAJOR"
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
elif echo "$COMMIT_MSG" | grep -qE "^feat(\(.+\))?:"; then
    echo "Feature detected - bumping MINOR"
    MINOR=$((MINOR + 1))
    PATCH=0
else
    echo "Patch change - bumping PATCH"
    PATCH=$((PATCH + 1))
fi

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo "New version: $NEW_VERSION"

# Update Cargo.toml
sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Build release
echo "Building release..."
cargo build --release

# Show binary info
echo ""
echo "✅ Build complete!"
echo "Binary: ./target/release/sl"
echo "Version: $NEW_VERSION"
ls -lh ./target/release/sl
