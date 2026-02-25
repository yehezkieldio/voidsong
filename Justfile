# List available just commands
default:
    @just --list

cargo := "cargo"

# Run all checks (clippy, fmt, and tests)
all: fmt clippy test

# Check code for compilation errors
check:
    {{ cargo }} check --workspace

# Run clippy with strict warnings
clippy *args:
    {{ cargo }} clippy --all-targets --all-features {{ args }} -- -D warnings

# Run all tests using nextest
test *args:
    {{ cargo }} nextest run {{ args }}

# Format code and check for style issues
fmt:
    {{ cargo }} fmt --all

# Check formatting without applying changes
fmt-check:
    {{ cargo }} fmt --all -- --check

# Update version numbers and generate changelog for a new release
release version:
    cargo set-version {{ version }}
    GITHUB_TOKEN=$(gh auth token) git-cliff --tag {{ version }} --output CHANGELOG.md
    git add .
    git commit -m "chore(release): {{ version }}"
    git tag -s v{{ version }} -m ""
    echo "Now do `git push origin master --tags` to publish the release"
