# Custom Windows build

The fork uses RustDesk 1.5.0 at upstream commit
`fada664df7a294d1d1a9ca3e7cd3637069122f17`.

The distributed product is **Holistic Remote**, based on RustDesk 1.5.0 and
modified by Holistic IT GmbH. It is not affiliated with or endorsed by the
RustDesk project. Corresponding source code is available at
https://github.com/holistic-it/rustdesk-custom.

## Build

Pushing a `custom/**` branch runs an unsigned Windows x64 test build. Its MSI
uses the workflow run number as its fourth version component and is available
as a workflow artifact. Branch builds do not publish a GitHub release or sign
packages. Test installs belong on selected devices such as RMM-WINTEST.

For a production release, push a tag matching `v*.custom.*` to run the
**Windows x64 release** workflow:

```text
git tag -a v1.5.0.custom.1 -m "Custom RustDesk 1.5.0, release 1"
git push origin v1.5.0.custom.1
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

The MSI installs the service stopped and disabled, including after repair or
upgrade. The Holistic agent enables it for a session and disables it afterward.
The EXE installer retains its existing service installation behavior.

The Windows installation is `C:\Program Files\HolisticRemote\HolisticRemote.exe`
with `HolisticRemote.dll` and service `HolisticRemote`. Configuration, logs, IPC,
and the MSI upgrade family use the separate `HolisticRemote` identity. The Rust
crate name and exported functions remain unchanged. Managed packages omit the
printer driver and adapter, and neither installer runs printer setup or cleanup.

Existing installations require a separate deployment migration. Configure the
RMM agent's `RustDesk.ExecutablePath` and `RustDesk.ServiceName` for the new
installation; remove only positively identified legacy Holistic packages.

Public rendezvous servers, their public key, and native/WebRTC STUN defaults
are removed. Only explicitly configured rendezvous servers are used; cached
server lists are ignored. Relay discovery uses the configured rendezvous server.
Managed endpoints always use the RustDesk relay, reject WebRTC offers, disable
TCP/UDP/IPv6 punching, skip NAT/STUN discovery, and do not start a direct-access
listener.
`build.py --managed-endpoint` applies `res/hbb-common.patch` to the pinned
upstream shared-library submodule before compiling; no separate fork is needed.

The selector calls RustDesk's existing bridge workflow, which generates both
bridge artifacts even though the managed Windows build uses only one of them.

## Maintaining the fork

The custom patch reuses RustDesk's existing build and packaging recipes. When
updating the upstream baseline:

1. Rebase the branch onto the selected upstream tag.
2. Reapply the small managed-endpoint patch series without copying build
   steps.
3. Confirm that all newly added non-Windows jobs are skipped when
   `windows-x64-only` is true.
4. Run `actionlint`, push the custom branch, and validate its unsigned test MSI
   before creating a production release tag.

## Licensing

RustDesk is licensed under AGPL-3.0. Distributed modified binaries must retain
the applicable notices and make their exact corresponding source available.
Keep release tags immutable so every package maps to a specific source
revision.
Windows packages contain one `LICENSE` file with the modification notice,
source date, exact source URL, and unchanged AGPL text. The build guide stays in
the source repository; it is not installed as `MODIFIED_BUILD.md`.
