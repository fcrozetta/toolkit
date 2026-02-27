---
tags:
    - cpp
    - wip
---

[:simple-github: Repository](https://github.com/fcrozetta/kubeforward) |
[:material-download: Releases](https://github.com/fcrozetta/kubeforward/releases/latest)
# kubeforward

Kubeforward is a CLI to validate and inspect Kubernetes port-forward definitions from a config file.

## Install

### macOS / Linux (Homebrew)

```bash
brew tap fcrozetta/tools
brew install kubeforward
```

### Windows (Scoop)

```bash
scoop bucket add fcrozetta https://github.com/fcrozetta/scoop-bucket
scoop install fcrozetta/kubeforward
```

### Manual install (GitHub Releases)

Download the archive for your platform from the [latest release](https://github.com/fcrozetta/kubeforward/releases/latest), extract it, and put `kubeforward` (or `kubeforward.exe`) on your `PATH`.

## Quick start

### 1. Create `kubeforward.yaml`

```yaml
version: 1
metadata:
  project: demo-project
defaults:
  namespace: default
  bindAddress: 127.0.0.1
environments:
  dev:
    forwards:
      - name: api
        resource:
          kind: deployment
          name: api
        ports:
          - local: 7000
            remote: 7000
```

### 2. Validate and inspect the plan

```bash
kubeforward help
kubeforward plan --config kubeforward.yaml
kubeforward plan --config kubeforward.yaml --env dev
```

## Commands

- `help`: shows available commands.
- `plan`: loads the config, validates it, and prints the normalized plan.
- `--config <path>`: config file path for `plan` (default: `kubeforward.yaml`)
- `--env <name>`: environment filter for `plan`

## Notes

- This tool currently focuses on config validation and plan output.
- For full config fields and semantics, see `docs/config-schema.md` in the repository.
