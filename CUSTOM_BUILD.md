# Custom Windows build

This branch tracks RustDesk 1.4.9, upstream commit
`6c578292e8ebbbec708b76986ba8c4bc7c509747`.

The distributed product is **Holistic Remote**, based on RustDesk 1.4.9 and
modified by Holistic IT GmbH since 2026-09-15. It is not affiliated with or
endorsed by the RustDesk project. Corresponding source code is available at
https://github.com/holistic-it/rustdesk-custom.

## Build

Push a tag matching `v*.custom.*` to run the **Windows x64 release** workflow:

```text
git tag -a v1.4.9.custom.1 -m "Custom RustDesk 1.4.9, release 1"
git push origin v1.4.9.custom.1
```

The workflow calls RustDesk's existing reusable Flutter build workflow with
its `windows-x64-only` input. This fork keeps only the Windows x64 entries in
the two Windows matrices and adds `--managed-endpoint` to that build. The two
custom Windows jobs run only for that entry workflow; other callers continue
with the reusable workflow's non-Windows jobs. The upstream Windows build and
packaging steps produce the x64 EXE and MSI and attach them to a GitHub
pre-release. No signing secrets are passed to the reusable workflow, so the
packages are unsigned.

The `managed-endpoint` feature builds an incoming-only Windows endpoint. It
suppresses the viewer and tray, skips the connection manager for read-only
unattended access, leaves the connection window visible for interactive
unattended access, disables unused API and remote printer startup, and packages
without user shortcuts. The Holistic agent applies each session's capability
and network policy before installing the password and reporting the session
ready. Consent requires the session password before displaying a local approval
prompt. The Fedora operator client remains stock.

The selector intentionally still calls RustDesk's existing bridge workflow,
which currently generates both of its bridge artifacts. The unused bridge
artifact costs one additional Linux job, but retaining the upstream workflow
unchanged makes upgrades and rebases easier.

## Maintaining the fork

The custom patch reuses RustDesk's existing build and packaging recipes. When
updating the upstream baseline:

1. Rebase the branch onto the selected upstream tag.
2. Reapply the small managed-endpoint patch series without copying build
   steps.
3. Confirm that all newly added non-Windows jobs are skipped when
   `windows-x64-only` is true.
4. Run `actionlint` and create a new `v*.custom.*` test tag.

## Licensing

RustDesk is licensed under AGPL-3.0. Distributed modified binaries must retain
the applicable notices and make their exact corresponding source available.
Keep release tags immutable so every package maps to a specific source
revision.
