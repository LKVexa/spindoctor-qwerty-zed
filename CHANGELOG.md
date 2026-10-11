# Changelog

## 1.1.0 (2026-10-10)

- Engine rebased on **SpinDoctor 3.1.2** (update 3.1.2-298-20261010): 1-298 logical qubits with 59-qubit entangled factors,
  prepared-factor Born-Z sampling above 29 qubits, the `mcx7` gate, wide basis indices, `q.folder` with 3.1.2 semantics
  (.qasm2/.circuit, skipped-file listing, compact status, 128 files / 4 MiB / 128 commands / 4096 characters), CSV report copies
  and measurement-CSV fitting equations. 241 presets replayed against the updated 3.1.2 reference.
- New tools: `spindoctor_functions` (in-chat function table) and `spindoctor_test_theory` (file, pasted text, chat history, exported
  chat transcripts).
- Zed extension: engine path validated (absolute, must end in `spindoctor_mcp.py`, no control characters) with clear errors; empty
  settings fall back to defaults; `PYTHONUTF8=1` for UTF-8 stdio on Windows; settings schema tightened; Rust unit tests added.
- Manifests: author RUSSELL PHILIP SMITHSON, license GPL-3.0-or-later (was MIT), repository URL corrected, description updated
  from 3.0.3 to 3.1.2.
- Optional reproducibility: `SPINDOCTOR_MEASUREMENT_SEED` / `SPINDOCTOR_FIELD_SEED` (64 hex characters) seed a new workspace.
- Function table: the q.query heading now says how to prepare a circuit (the old "run the Circuits rows first" did not leave a run circuit).
- Repository is text-only: `extension.wasm` is built by Zed or `build.sh`/`build.ps1`; sample PDFs are recreated by `make_sample_pdfs.py`.

## 1.0.0

- First release (SpinDoctor 3.0.3 semantics).
