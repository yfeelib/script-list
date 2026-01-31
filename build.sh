#!/bin/bash
# build.sh - Build with custom versioning rules
# - PATCH increments on every build
# - MINOR increments when PATCH reaches 100, then PATCH resets to 0
# - MAJOR increments when year changes

set -e

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"

# Parse version components
MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)

# Get current year
CURRENT_YEAR=$(date +%Y)

# Check if year changed for MAJOR bump
if [ "$MAJOR" != "$CURRENT_YEAR" ]; then
    echo "New year detected: $CURRENT_YEAR (was $MAJOR)"
    MAJOR=$CURRENT_YEAR
    MINOR=0
    PATCH=1
else
    # Increment PATCH
    PATCH=$((PATCH + 1))
    
    # If PATCH reaches 100, bump MINOR and reset PATCH
    if [ $PATCH -ge 100 ]; then
        echo "PATCH reached 100 - bumping MINOR"
        MINOR=$((MINOR + 1))
        PATCH=0
    fi
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
