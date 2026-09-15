# Custom Windows build

This branch tracks RustDesk 1.4.9, upstream commit
`6c578292e8ebbbec708b76986ba8c4bc7c509747`.

## Build

Push a tag matching `v*.custom.*` to run the **Windows x64 release** workflow:

```text
git tag -a v1.4.9.custom.1 -m "Custom RustDesk 1.4.9, release 1"
git push origin v1.4.9.custom.1
```

The workflow calls RustDesk's existing reusable Flutter build workflow with
its `windows-x64-only` input. This fork keeps only the Windows x64 entries in
the two Windows matrices. The upstream Windows build and packaging steps
produce the x64 EXE and MSI and attach them to a GitHub pre-release. No signing
secrets are passed to the reusable workflow, so the packages are unsigned.

The selector intentionally still calls RustDesk's existing bridge workflow,
which currently generates both of its bridge artifacts. The unused bridge
artifact costs one additional Linux job, but retaining the upstream workflow
unchanged makes upgrades and rebases easier.

## Maintaining the fork

The custom patch changes only job selection in RustDesk's existing build
workflow; it does not duplicate the Windows build recipe. When updating the
upstream baseline:

1. Rebase the branch onto the selected upstream tag.
2. Resolve selector conflicts, if any, without copying build steps.
3. Confirm that all newly added non-Windows jobs are skipped when
   `windows-x64-only` is true.
4. Run `actionlint` and create a new `v*.custom.*` test tag.

## Licensing

RustDesk is licensed under AGPL-3.0. Distributed modified binaries must retain
the applicable notices and make their exact corresponding source available.
Keep release tags immutable so every package maps to a specific source
revision.
