# Release Checklist

Use this checklist when preparing a new release of steelkilt.

## Pre-Release

- [ ] All features for the release are merged to `main`
- [ ] All CI checks pass on `main`
- [ ] All tests pass locally: `cargo test --all-features`
- [ ] Code is properly formatted: `cargo fmt --all -- --check`
- [ ] No clippy warnings: `cargo clippy --all-features -- -D warnings`
- [ ] Documentation is up to date
- [ ] CHANGELOG.md is updated with release notes (if exists)

## Version Update

- [ ] Update version in `Cargo.toml`
  ```toml
  version = "X.Y.Z"
  ```

- [ ] Update version in README.md installation examples
  ```toml
  steelkilt = "X.Y.Z"
  ```

- [ ] Run `cargo build` to update `Cargo.lock`

- [ ] Commit version bump:
  ```bash
  git add Cargo.toml Cargo.lock README.md
  git commit -m "Bump version to X.Y.Z"
  git push origin main
  ```

## Create Release

- [ ] Create and push git tag:
  ```bash
  git tag vX.Y.Z
  git push origin vX.Y.Z
  ```

- [ ] Create GitHub Release:
  1. Go to https://github.com/tonybierman/steelkilt/releases/new
  2. Select tag: `vX.Y.Z`
  3. Release title: `vX.Y.Z - Brief description`
  4. Add release notes (features, fixes, breaking changes)
  5. Click "Publish release"

## Post-Release

- [ ] Watch GitHub Actions workflow complete:
  - https://github.com/tonybierman/steelkilt/actions
  - Verify "Release" workflow succeeds

- [ ] Verify publication on crates.io:
  - https://crates.io/crates/steelkilt
  - Check version appears

- [ ] Test installation of published crate:
  ```bash
  cargo install steelkilt --version X.Y.Z
  ```

- [ ] Announce release (if applicable):
  - [ ] Social media
  - [ ] Reddit (r/rust)
  - [ ] Project forums
  - [ ] Documentation site

## First-Time Setup

Only needed once before the first release:

- [ ] Get crates.io API token:
  1. Go to https://crates.io/me
  2. Generate new token with "Publish updates" scope

- [ ] Add token to GitHub Secrets:
  1. Go to repository Settings → Secrets and variables → Actions
  2. Click "New repository secret"
  3. Name: `CARGO_REGISTRY_TOKEN`
  4. Value: Your crates.io token
  5. Click "Add secret"

## Troubleshooting

If the release workflow fails:

1. **Version mismatch**: Ensure Cargo.toml version matches the git tag (without the `v` prefix)
2. **Tests fail**: Fix failing tests and create a new patch release
3. **Clippy errors**: Fix warnings and create a new patch release
4. **Publication fails**: Check CARGO_REGISTRY_TOKEN is correct and has proper permissions
5. **Verification warning**: The workflow may warn if it can't verify the version on crates.io within 3 minutes. This is normal due to indexing delays - check https://crates.io/crates/steelkilt manually to confirm publication.

## Versioning Guidelines

Follow [Semantic Versioning](https://semver.org/):

- **Major (X.0.0)**: Breaking changes to public API
- **Minor (0.X.0)**: New features, backward compatible
- **Patch (0.0.X)**: Bug fixes, backward compatible

For Draft RPG features:
- Adding new optional modules → Minor version
- Changing existing combat mechanics → Major version
- Bug fixes in calculations → Patch version
