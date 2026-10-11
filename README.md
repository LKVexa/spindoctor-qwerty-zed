# SpinDoctor Qwerty for Zed

**spindoctor-qwerty-zed** is a [Zed](https://zed.dev) extension that adds the SpinDoctor Qwerty quantum-circuit **simulator** to Zed's
Agent Panel as an MCP context server. The AI in Zed can then build and run circuits with SpinDoctor's `q.*` commands, run any of the
241 presets, process a folder of command files with `q.folder`, test a theory you give it, and write TXT, PDF, JSON and CSV reports.

- Version **1.1.0**, engine at **SpinDoctor 3.1.2 parity** (update 3.1.2-298-20261010).
- Host: **Zed** (context server extension: `extension.toml` + Rust `src/lib.rs` compiled to WebAssembly, `zed_extension_api` 0.7.0).
- Self-contained: the engine is bundled in `engine/` (pure Python, standard library only, NumPy optional). No network access, no installs.

> **Honest limits.** SpinDoctor runs classical FP64 arithmetic on your computer. It is **not** a quantum computer, never contacts
> quantum hardware, and gives no quantum speed-up or quantum advantage. Registers hold 1-298 logical qubits, but any group of
> entangled qubits (one "factor") is limited to 59 qubits, and dense factors are limited much further by memory
> (2^n x 16 bytes; about 22 qubits with the standard library, more with NumPy). Registers wider than 29 qubits sample in basis Z only.
> Python 3.8 or newer is required.

## Install (Zed)

1. Install **Python 3.8+** (`python3 --version`, or `python --version` on Windows).
2. Install **Rust 1.88 or newer via rustup** (<https://rustup.rs>; an older distribution `rustc` cannot build the locked dependencies). Zed compiles the extension itself and adds the `wasm32-wasip2` target automatically.
3. Download or clone this repository, for example `git clone https://github.com/LKVexa/spindoctor-qwerty-zed`.
4. In Zed open the Extensions page (`zed: extensions`), click **Install Dev Extension** and choose the `spindoctor-qwerty-zed` folder.
5. Zed shows the configuration modal. Set `engine` to the absolute path of `engine/scripts/spindoctor_mcp.py` in that folder.
   Or add it to your Zed settings (`zed: open settings file`):

   ```json
   {
     "context_servers": {
       "spindoctor-qwerty": {
         "settings": {
           "engine": "/home/you/spindoctor-qwerty-zed/engine/scripts/spindoctor_mcp.py",
           "python": "python3",
           "workspace": "/home/you/spindoctor-workspace"
         }
       }
     }
   }
   ```
   On Windows use forward slashes, e.g. `"C:/Users/you/spindoctor-qwerty-zed/engine/scripts/spindoctor_mcp.py"`, and `"python": "python"`.
6. Open **Settings > AI > MCP Servers**: the `spindoctor-qwerty` dot turns green ("Server is active"). In the Agent Panel ask
   *"show the SpinDoctor functions"*.

**Without the extension (also works):** Zed plans to deprecate MCP-server extensions in favour of the MCP registry. You can add the
same server as a custom local server (Settings > AI > MCP Servers > Add Local Server), which writes:

```json
{ "context_servers": { "spindoctor-qwerty": { "command": "python3", "args": ["/home/you/spindoctor-qwerty-zed/engine/scripts/spindoctor_mcp.py"], "env": {} } } }
```

**Building the WebAssembly yourself (optional).** `extension.wasm` is not committed (the repository is text-only). Zed builds it
during *Install Dev Extension*; to build it by hand run `sh build.sh` (macOS/Linux) or `powershell -ExecutionPolicy Bypass -File build.ps1` (Windows). Both run the Rust unit
tests and then `cargo build --release --target wasm32-wasip2`, copying the result to `extension.wasm`.

**Sample theory PDFs.** `engine/examples/theory/` holds sample theory files and chat transcripts. The two sample PDFs are stored as
text; run `python3 engine/examples/theory/make_sample_pdfs.py` to recreate them.

## What you can ask for

## SpinDoctor Qwerty - plugin tools

| Function | What it does | Command to type |
|---|---|---|
| Function table (spindoctor_functions) | Show this table in chat | Ask: "show the SpinDoctor functions" (CLI: `spindoctor_cli.py functions`) |
| Run a script (spindoctor_run_script) | Run any q.* lines above; returns results + TXT/JSON/CSV/PDF report paths | Ask: "run q.circuit new bell 2 / q.circuit add h 0 / ..." |
| Run a preset (spindoctor_run_preset) | Run qc-001..qc-241 with overrides | Ask: "run preset qc-067 with shots=64" |
| List presets (spindoctor_list_presets) | Search the 241 presets | Ask: "list presets about Grover" |
| Read a report (spindoctor_report) | List/read TXT reports (PDF/JSON paths alongside) | Ask: "show the newest SpinDoctor report" |
| Test a theory from a file (spindoctor_test_theory source=file) | Read a .txt/.md/.pdf/.py/.json/.csv/.q attachment, then hypothesis -> prediction -> q.* script -> run -> verdict + TXT/PDF theory report | Ask: "test the theory in C:/notes/theory.md" (the host must pass the file path/content) |
| Test a theory from text (spindoctor_test_theory source=text) | Same pipeline for pasted text | Ask: "test this theory: a Hadamard on \|0> gives 50/50" |
| Test the theory we discussed (spindoctor_test_theory source=chat) | The AI summarises the theory from the conversation (claims, assumptions, variables, prediction), you confirm, then it is tested; quoted excerpts go in the report under 'Source: chat history'. An exported chat transcript (.txt/.md/.json/.html) also works | Ask: "test the theory we discussed" or "test the theory in my exported chat C:/chats/conversation.json" |

Every command family is listed with a runnable example in **[FUNCTIONS.md](FUNCTIONS.md)** (the exact table the tool shows):
circuits (`q.circuit`, including 298-qubit wide registers and the `mcx7` gate), queries (`q.query`), the tranche register
(`q.tranche`), the procedural grid (`q.prepare`/`q.gate`/`q.measure`), presets and sequences, resource planning, math calculators,
physics models, lessons, noise, error correction, compilation, algorithms, inputs/folders/history (`q.input`, `q.folder`,
`q.history`), and the cartridge commands.

## Theory testing

`spindoctor_test_theory` turns a claim into a test: hypothesis -> measurable prediction -> `q.*` script -> run -> compare -> a TXT +
JSON + PDF **theory test report** with the hypothesis, script, raw results, verdict (*supported*, *not supported* or *inconclusive*)
and limits.

- **From a file:** .txt, .md, .pdf, .py, .json, .csv or .q. In Zed, give the AI a path inside your project.
- **From pasted text.**
- **From the chat history:** say *"test the theory we discussed"*. The AI writes a short summary (claims, assumptions, variables,
  measurable prediction), shows it to you to confirm or correct, then passes it with optional quoted excerpts; the report records them
  under *Source: chat history*. Exported chat transcripts (.txt/.md/.json/.html from ChatGPT, Claude and others) also work.
- The plugin cannot read your chat or attachments by itself: only the AI sees the conversation, and very long chats may already be
  truncated by Zed's context window.

## Folder ingestion (`q.folder`)

`q.folder run path="<folder>"` runs every supported file at the top level of a folder (.txt, .csv, .json, .qasm, .qasm2, .circuit,
.pdf) in ordinal name order: 1-128 supported files, at most 4 MiB per file, 1-128 commands per file and 4096 characters per command,
strict UTF-8. Other files (for example .md) are skipped and listed. The first failing file stops the job.

## Testing

| Test | Command | Result for 1.1.0 |
|---|---|---|
| Rust unit tests (path validation) | `cargo test` | 2/2 pass |
| WebAssembly build + component check | `sh build.sh` | builds; exports the `zed:extension` context-server interface |
| Zed stand-in host + end-to-end | `python3 tests/zed_host_check.py` | see [E2E_RESULTS.md](E2E_RESULTS.md) |

**host-simulated:** Zed 1.23.2 was installed for free on a headless Linux test machine, but it cannot open a window there (no GPU/Vulkan
surface), so it could not load the extension. The extension was therefore checked with the stand-in host `tests/zed_host_check.py`,
which validates the manifests and settings files, builds the context-server command exactly as `src/lib.rs` does, and drives the real
MCP server over stdio. Details in [E2E_RESULTS.md](E2E_RESULTS.md).

## Author and license

Author: **RUSSELL PHILIP SMITHSON** <russellsmithson77@outlook.com>

Copyright (C) 2026 RUSSELL PHILIP SMITHSON. Licensed under the GNU General Public License v3.0 or later
(`GPL-3.0-or-later`): see [LICENSE](LICENSE), [NOTICE](NOTICE) and [COPYRIGHT](COPYRIGHT).
