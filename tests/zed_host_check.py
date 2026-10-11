#!/usr/bin/env python3
# SpinDoctor Qwerty 1.1.0 - Zed extension
# Copyright (C) 2026 RUSSELL PHILIP SMITHSON <russellsmithson77@outlook.com>
# SPDX-License-Identifier: GPL-3.0-or-later
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU General
# Public License as published by the Free Software Foundation, either version 3 of the License, or (at your
# option) any later version. It is distributed WITHOUT ANY WARRANTY; see the LICENSE file for details.
"""Zed stand-in host check (no Zed needed): validates extension.toml, Cargo.toml and the configuration files,
then builds the context-server command exactly as src/lib.rs does from the user's settings and runs the
end-to-end test (tests/e2e_mcp.py) through that command.  Standard library only (Python 3.8+).

usage: python3 tests/zed_host_check.py [--python python3]"""
import json, os, re, subprocess, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def toml_kv(path):
    """Tiny TOML reader for the flat keys used here (tomllib is used when available)."""
    try:
        import tomllib
        with open(path, "rb") as fh: return tomllib.load(fh)
    except ImportError:
        out = {}; cur = out
        for line in open(path, encoding="utf-8"):
            line = line.split("#", 1)[0].strip() if not line.strip().startswith("authors") else line.strip()
            if not line: continue
            m = re.match(r"^\[([^\]]+)\]$", line)
            if m:
                cur = out
                for part in m.group(1).split("."): cur = cur.setdefault(part, {})
                continue
            k, v = [x.strip() for x in line.split("=", 1)]
            cur[k] = json.loads(v) if v.startswith(("[", '"')) else int(v) if v.isdigit() else v
        return out


def jsonc(path):
    text = open(path, encoding="utf-8").read()
    return json.loads("\n".join(re.sub(r'^\s*//.*$', "", l) for l in text.splitlines()))


def main():
    py = sys.argv[sys.argv.index("--python") + 1] if "--python" in sys.argv else sys.executable
    ok = True

    def T(name, cond, detail=""):
        nonlocal ok
        ok = ok and bool(cond); print(("PASS " if cond else "FAIL ") + name + ("" if cond else "  -- " + str(detail)))

    ext = toml_kv(os.path.join(ROOT, "extension.toml")); cargo = toml_kv(os.path.join(ROOT, "Cargo.toml"))
    T("extension.toml required keys (id, name, version, schema_version, authors, description, repository)",
      all(k in ext for k in ("id", "name", "version", "schema_version", "authors", "description", "repository")), sorted(ext))
    T("extension.toml declares [context_servers.spindoctor-qwerty]", "spindoctor-qwerty" in ext.get("context_servers", {}), ext.get("context_servers"))
    T("schema_version = 1, version 1.1.0 in extension.toml and Cargo.toml", ext["schema_version"] == 1 and ext["version"] == cargo["package"]["version"] == "1.1.0")
    T("Cargo: cdylib + zed_extension_api 0.7.0 + GPL-3.0-or-later", cargo["lib"]["crate-type"] == ["cdylib"] and cargo["dependencies"]["zed_extension_api"] == "0.7.0" and cargo["package"]["license"] == "GPL-3.0-or-later")
    T("author in manifests", all("RUSSELL PHILIP SMITHSON" in a for a in ext["authors"]) and all("RUSSELL PHILIP SMITHSON" in a for a in cargo["package"]["authors"]))
    schema = json.load(open(os.path.join(ROOT, "configuration", "settings_schema.json"), encoding="utf-8"))
    defaults = jsonc(os.path.join(ROOT, "configuration", "default_settings.jsonc"))
    T("settings schema + default settings parse and agree", set(defaults) == set(schema["properties"]) and schema["required"] == ["engine"])
    engine = os.path.join(ROOT, "engine", "scripts", "spindoctor_mcp.py")
    settings = {"engine": engine, "python": py, "workspace": ""}
    T("example settings satisfy the schema pattern", re.search(schema["properties"]["engine"]["pattern"], settings["engine"]) and os.path.isabs(engine) and os.path.exists(engine))
    # same rule as src/lib.rs: python setting (or python3 / python on Windows) + [engine], PYTHONUTF8=1
    cmd = [settings["python"] or ("python" if os.name == "nt" else "python3"), settings["engine"]]
    env = dict(os.environ, PYTHONUTF8="1", PYTHONDONTWRITEBYTECODE="1")
    r = subprocess.run([sys.executable, os.path.join(ROOT, "tests", "e2e_mcp.py"), "--server"] + cmd +
                       ["--examples", os.path.join(ROOT, "engine", "examples", "theory"), "--out", os.path.join(ROOT, "tests", "e2e_results.json")], env=env)
    T("end-to-end test through the context-server command", r.returncode == 0)
    print("HOST-CHECK", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
