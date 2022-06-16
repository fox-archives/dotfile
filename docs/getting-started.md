# Getting Started

## Prerequisites

- Decent familiarity with Bash scripting
- Familiarity with Git

## Initialization

There is an initialization script to automatically generate the directories that `dotmgr` uses

```sh
https://raw.githubusercontent.com/hyperupcall/dotmgr/main/scripts/init.sh | bash
```

## Move elsewhere

- Do not create functions that start with `_`. These are reserved by use by `dotmgr` itself

### Hooks

- `actionPlumbingBefore.sh`
- `actionPlumbingAfter.sh`
- `actionBefore.sh`
- `actionAfter.sh`
- `bootstrapBefore.sh`
- `bootstrapAfter.sh`
- `doctorBefore.sh`
- `doctorAfter.sh`
- `updateBefore.sh`
- `updateAfter.sh`
