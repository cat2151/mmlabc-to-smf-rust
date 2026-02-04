# GitHub Actions Scripts

This directory contains reusable scripts for GitHub Actions workflows.

## create-build-failure-issue.js

Creates a GitHub issue automatically when a build fails.

### Features

- Creates an issue with build failure details (commit SHA, branch, workflow run link)
- Automatically creates required labels if they don't exist
- Prevents duplicate issues by checking for existing issues with the same commit SHA
- Handles API errors gracefully

### Usage

Use with `actions/github-script@v7`:

```yaml
- name: Create issue if build fails
  if: failure()
  uses: actions/github-script@v7
  with:
    script: |
      const script = require('./.github/scripts/create-build-failure-issue.js');
      await script({github, context, core});
```

### Required Permissions

The workflow job must have `issues: write` permission:

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: write
```

### Environment Variables

The script uses standard GitHub Actions environment variables:
- `GITHUB_SHA`: Commit SHA
- `GITHUB_SERVER_URL`: GitHub server URL
- `GITHUB_REPOSITORY`: Repository (owner/repo)
- `GITHUB_RUN_ID`: Workflow run ID
- `GITHUB_REF`: Git reference

### Labels

The script creates and uses two labels:
- `build-failure` (orange #ffa500): Indicates a build failure
- `bug` (red #d73a4a): Standard bug label

### Reusability

This script can be copied to any repository and used in any workflow. Simply:
1. Copy `.github/scripts/create-build-failure-issue.js` to your repository
2. Add the usage code to your workflow
3. Ensure proper permissions are set

No modifications to the script are needed for different repositories or workflows.
