#!/usr/bin/env python3
"""Symbolize a samply (`--save-only`) profile's hot frames via llvm-symbolizer + PDB.

samply's headless `--save-only` leaves native frames unresolved as `fun_<hex>` /
`0x<hex>` — but that hex IS the module-relative address (RVA), and the matching
PDB sits next to the exe. This tool tallies leaf self-time per frame, batch-resolves
the RVAs through `llvm-symbolizer --relative-address` against the exe/PDB, and prints
the top-N self-time frames with real Rust names + file:line. The headless route to
NAMED frames (no interactive Firefox needed).

Usage:  python scripts/samply_symbolize.py <profile.json.gz> <exe-path> [topN] [name-filter]
Reusable across any samply profile + its matching exe (PDB beside it).
"""
import json, gzip, sys, re, subprocess, collections, shutil, os

HEX = re.compile(r'^(?:0x|fun_)([0-9a-fA-F]+)$')


def load(path):
    op = gzip.open if path.endswith('.gz') else open
    with op(path, 'rt', encoding='utf-8') as f:
        return json.load(f)


def get_strings(prof, thread):
    for src in (prof.get('shared', {}) if isinstance(prof, dict) else {}, thread, prof):
        if isinstance(src, dict):
            for key in ('stringArray', 'stringTable'):
                v = src.get(key)
                if isinstance(v, list):
                    return v
                if isinstance(v, dict):
                    a = v.get('strings') or v.get('_array')
                    if isinstance(a, list):
                        return a
    return []


def main(prof_path, exe, topn=30, name_filter=None):
    prof = load(prof_path)
    counts = collections.Counter()
    total = 0
    for th in prof.get('threads', []):
        stacks = th.get('samples', {}).get('stack', [])
        sframe = th.get('stackTable', {}).get('frame', [])
        ffunc = th.get('frameTable', {}).get('func', [])
        fname = th.get('funcTable', {}).get('name', [])
        strs = get_strings(prof, th)
        for si in stacks:
            if si is None:
                continue
            try:
                nm = strs[fname[ffunc[sframe[si]]]]
            except (IndexError, TypeError):
                continue
            counts[nm] += 1
            total += 1
    if not total:
        print('no leaf samples parsed (format mismatch?)')
        return

    rva = {nm: '0x' + m.group(1) for nm in counts for m in [HEX.match(nm)] if m}
    resolved = {}
    sym = shutil.which('llvm-symbolizer')
    if sym and rva:
        addrs = sorted(set(rva.values()))
        out = subprocess.run(
            [sym, f'--obj={exe}', '--relative-address', '--output-style=JSON',
             '--functions=linkage', '--demangle'],
            input='\n'.join(addrs) + '\n', capture_output=True, text=True)
        addr_name = {}
        for line in out.stdout.splitlines():
            line = line.strip()
            if not line:
                continue
            try:
                j = json.loads(line)
            except json.JSONDecodeError:
                continue
            a = str(j.get('Address', '')).lower()
            syms = j.get('Symbol', [])
            if syms:
                s = syms[0]
                fn = s.get('FunctionName') or '?'
                fl = (s.get('FileName') or '').replace('\\', '/').split('/')[-1]
                ln = s.get('Line', 0)
                addr_name[a] = f"{fn}  ({fl}:{ln})" if fl and fl != '??' else fn
        for nm, a in rva.items():
            resolved[nm] = addr_name.get(a.lower(), nm)

    n_unres = sum(1 for nm in counts if nm in rva and resolved.get(nm, nm) == nm)
    print(f"total leaf samples: {total}   symbolizer: {'llvm-symbolizer' if sym else 'NONE'}   "
          f"(RVA frames: {len(rva)}, still-unresolved: {n_unres})")
    shown = 0
    for nm, c in counts.most_common():
        disp = resolved.get(nm, nm)
        if name_filter and name_filter.lower() not in disp.lower():
            continue
        print(f"{100 * c / total:6.2f}%  {c:7d}  {disp}")
        shown += 1
        if shown >= topn:
            break


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2],
         int(sys.argv[3]) if len(sys.argv) > 3 else 30,
         sys.argv[4] if len(sys.argv) > 4 else None)
