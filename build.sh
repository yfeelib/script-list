#!/bin/bash
# build.sh - Build with automatic version bump (100 unit increments)

set -e

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"

# Parse version components
MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)

# Increment patch by 100
NEW_PATCH=$((PATCH + 100))
NEW_VERSION="$MAJOR.$MINOR.$NEW_PATCH"

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
