# End-to-end results - spindoctor-qwerty-zed 1.1.0

Date: 2026-10-10 (PT). Engine: SpinDoctor Qwerty 1.1.0, SpinDoctor 3.1.2 parity (update 3.1.2-298-20261010).

## Publishing decision (recorded as required)

RUSSELL PHILIP SMITHSON approved publishing ("GO") on the basis that the **updated SpinDoctor 3.1.2** builds and passes its own test
suite **18/18**, with the three test checks that still assumed the old 29-qubit limit adapted to the new 298-qubit limit
(`engine_regressions` dense comparison restricted to gates with at most 3 operands now that `mcx7` has 8; `direct_preset_regressions`
and `csv_gate_regressions` now expect 299 qubits, not 30, to be refused). Unmodified, the update passes 15/18 because of exactly
those three stale checks.

## Reference used

All parity comparisons use reference outputs regenerated from the **updated** SpinDoctor 3.1.2 C# build (241 presets in one
session, the folder-ingest chug series, and direct wide-register commands).

## Shared engine (Phase A, run once for all repos)

| Suite | Result |
|---|---|
| 241 presets replayed against the updated 3.1.2 reference | 236 identical; 5 with documented differences (qc-006, qc-008, qc-009, qc-016, qc-019: OpenCL GPU fields absent in a CPU-only engine, tranche checkpoint wording, session-state file name). qc-198 fails preflight in both C# and the engine (`q.compile synthesize` is not implemented in SpinDoctor 3.1.2 either). Work-accounting counters are compared separately and documented. |
| Folder-ingest chug series (41 folders) | 40 pass; `08_cancel` not applicable (the engine runs `q.folder` synchronously, so there is nothing to cancel; the job completes correctly) |
| Wide-register parity (direct 298-qubit commands) | 0 differences |
| 11-file 298-qubit example folder | identical (73 distinct outcomes, histogram preview and counts match) |
| Engine validation suite | 22/22 |
| Theory-testing suite (files, text, chat summary, 5 transcript formats) | 43/43 |
| Function-table examples (every "Command to type") | 168/168 run OK |

## This repository: clean-room test

The staged repository was copied into a fresh empty folder with no access to development folders, under `env -i` (clean PATH,
temporary HOME, no PYTHONPATH). Only the README install steps were followed.

| Step | Result |
|---|---|
| `sh build.sh` (Rust unit tests + `cargo build --release --target wasm32-wasip2`) | 2/2 unit tests pass; `extension.wasm` built (312,464 bytes) |
| `wasm-tools validate` + `wasm-tools component wit` | valid component; exports `context-server-command` and `context-server-configuration` (`zed:extension` world) |
| `python3 tests/zed_host_check.py` (manifests, settings schema, command built as in `src/lib.rs`) | 8/8 checks pass |
| End-to-end through the MCP entry point (`python3 engine/scripts/spindoctor_mcp.py`, stdio) | **12/12 pass** on Python 3.13.5 and on Python 3.8.20 |

End-to-end checks (through the host entry point, `tests/e2e_mcp.py`):

1. `initialize` reports server `spindoctor-qwerty` 1.1.0; `tools/list` exposes the 6 tools.
2. Function table: Markdown, 19 sections, lists the 298-qubit limits; 241 presets listed.
3. One script per command family: 16 families, every table example runs OK.
4. 298-qubit circuit with `mcx7`: prepared-factor-Born-Z sampling, outcomes consistent.
5. `q.folder` on a mixed folder (.txt sequence, .csv commands, .json circuit, .qasm) completes; `notes.md` is skipped and listed.
6. Theory testing from a file, from a chat summary with 2 quoted excerpts, and from an exported chat transcript: all *supported*, TXT+PDF reports written.
7. Presets qc-001..qc-025 in a fresh workspace seeded with the reference seeds: 20 identical to the updated 3.1.2 reference, 5 documented differences.
8. Reports: 255 PDFs structurally valid (header, xref, startxref offset, pages, `pdftotext` finds SpinDoctor text), each with a TXT twin.

## Host app

**host-simulated.** Zed 1.23.2 (free, no sign-in) was installed on the headless Linux test machine, but it cannot open a window
there ("Failed to create surface for any enabled backend": no GPU/Vulkan), so Zed itself could not load the extension. The stand-in
host `tests/zed_host_check.py` was used instead. No language model was called and no account or API key was used.

## Post-push clone

After publishing, the repository was cloned back from GitHub into a fresh folder and re-tested under `env -i`:

| Check | Result |
|---|---|
| Files | 55/55 present; 54 byte-identical to the staged copy. `engine/spindoctor/data/SPINDOCTOR_COMMAND_CATALOGUE.txt` arrived with LF instead of CRLF line endings (reference text only; content otherwise identical) |
| `sh build.sh` | Rust unit tests pass; `extension.wasm` builds and validates |
| `python3 tests/zed_host_check.py` | 8/8 host checks, end-to-end **12/12** |

Binaries: `extension.wasm` is not committed (the publishing connector carries text files only); Zed builds it during
*Install Dev Extension*, or run `sh build.sh`. Commits are made by the LKVexa account (commit e-mail russellsmithson77@outlook.com); the connector cannot
set the author name, so authorship is recorded in the README, NOTICE, COPYRIGHT, manifests and file headers.
