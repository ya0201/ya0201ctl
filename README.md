## install
```bash
brew install ya0201/tap/ya0201ctl
```

## How to release

### Github action
run `bump-version` action

### Manual release
```
git switch main && export tag=vX.Y.Z && git tag ${tag} && git push origin ${tag}
```
