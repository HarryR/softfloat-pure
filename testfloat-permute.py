#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-3-Clause
"""
This script is used to verify softfloat_pure against Berkeley TestFloat.
It runs every permutation of rounding modes, tininess and exact with every op.
You can run specific test cases by passing them as arguments. e.g:

```bash
testfloat-permute.py f32 f64 roundToInt
```

Which will run all `roundToInt` tests in all modes for both `f32` and `f64`.
This is very useful to isolate specific tests rather than running the full suite.
The full permutation suite is nearly 1000 tests, which may take a while to run.

For full tests you should run with the `level2` parameter, this will take even
longer to run... but is much more comprehensive and will test more edge cases.

```bash
testfloat-permute.py level2
```
"""
import os
import sys
import subprocess

RELEASE = False
COVERAGE = False
QEMU = False
STDERR = False
EXIT = True
LEVEL_2 = False
CWD = os.path.dirname(os.path.realpath(sys.argv[0]))
TESTFLOAT_DIR = os.path.realpath(os.path.join(CWD,'testfloat','berkeley-testfloat-3'))
TESTFLOAT_X86_BUILD_DIR = os.path.realpath(os.path.join(TESTFLOAT_DIR,'build','Linux-x86_64-GCC'))
TESTFLOAT_RV64_BUILD_DIR = os.path.realpath(os.path.join(TESTFLOAT_DIR,'build','Linux-RISCV64-GCC'))
TESTFLOAT_X86_GEN = os.path.join(TESTFLOAT_X86_BUILD_DIR, 'testfloat_gen')
TESTFLOAT_RV64_GEN = os.path.join(TESTFLOAT_RV64_BUILD_DIR, 'testfloat_gen')

ROUND_MODES = ['rnear_even', 'rminMag', 'rmin', 'rmax', 'rnear_maxMag']
TININESS = ['tininessbefore', 'tininessafter']
EXACT = ['exact', 'notexact']
INT_TYPES = ['ui32', 'ui64', 'i32', 'i64']
FP_TYPES = ['f32', 'f64']
FP_OPS = ['roundToInt', 'add', 'sub', 'mul', 'mulAdd', 'div', 'rem', 'sqrt',
          'eq', 'le', 'lt', 'eq_signaling', 'le_quiet', 'lt_quiet', 'to']

NEW_ROUND_MODES = []
NEW_TININESS = []
NEW_EXACT = []
NEW_INT_TYPES = []
NEW_FP_TYPES = []
NEW_FP_OPS = []
for a in sys.argv[1:]:
    if a == 'level2':
        LEVEL_2 = True
    elif a == 'release':
        RELEASE = True
    elif a == 'noexit':
        EXIT = False
    elif a == 'stderr':
        STDERR = True
    elif a == 'coverage':
        COVERAGE = True
    elif a == 'qemu':
        QEMU = True
    elif a == 'native':
        QEMU = False
    elif a in ROUND_MODES or a == 'rodd':
        NEW_ROUND_MODES.append(a)
    elif a in TININESS:
        NEW_TININESS.append(a)
    elif a in INT_TYPES:
        NEW_INT_TYPES.append(a)
    elif a in FP_TYPES:
        NEW_FP_TYPES.append(a)
    elif a in FP_OPS:
        NEW_FP_OPS.append(a)
    else:
        print("Unrecognized flag:", a)
        sys.exit(1)

# 'rodd' gets added if 'native' flag is added
if not QEMU:
    ROUND_MODES.append('rodd')

if len(NEW_ROUND_MODES):
    ROUND_MODES = NEW_ROUND_MODES
if len(NEW_TININESS):
    TININESS = NEW_TININESS
if len(NEW_INT_TYPES):
    INT_TYPES = NEW_INT_TYPES
if len(NEW_FP_TYPES):
    FP_TYPES = NEW_FP_TYPES
if len(NEW_FP_OPS):
    FP_OPS = NEW_FP_OPS

ALL_MODES = []
for a in ROUND_MODES:
    for b in TININESS:
        for c in EXACT:
            ALL_MODES.append([a,b,c])

ALL_OPS = []

if 'to' in FP_OPS:
    for a in INT_TYPES:
        for b in FP_TYPES:
            ALL_OPS.append(f"{a}_to_{b}")
            ALL_OPS.append(f"{b}_to_{a}")

    for a in FP_TYPES:
        for b in FP_TYPES:
            if a == b:
                continue
            ALL_OPS.append(f"{a}_to_{b}")

for a in FP_OPS:
    if a == 'to':
        continue
    for b in FP_TYPES:
        ALL_OPS.append(f"{b}_{a}")

def run(m, o):
    margs = [f'-{_}' for _ in m]
    l2arg = ['-level', '2'] if LEVEL_2 else []
    if QEMU:
        p1_args = ['qemu-riscv64', TESTFLOAT_RV64_GEN]
    else:
        p1_args = [TESTFLOAT_X86_GEN]
    p1_args += margs + l2arg + [o]

    exitargs = ['-exit'] if EXIT else ['-noexit']
    llvm_cov = []
    run_args = ['-q']
    if RELEASE:
        run_args.append('-r')
    if COVERAGE:
        llvm_cov = [
            'llvm-cov',
            '--offline',
            '--no-clean',
            '--no-cfg-coverage',
        ]
    p2_args = ['cargo'] + llvm_cov + ['run'] + run_args + ['--'] + [o] + margs + exitargs
    print()
    print(' '.join([o] + m))
    print("\t\t", " ".join(p1_args), '|', ' '.join(p2_args))
    stderr = subprocess.DEVNULL if not STDERR else None
    p1 = subprocess.Popen(p1_args, stdout=subprocess.PIPE, stderr=stderr)
    p2 = subprocess.Popen(p2_args, stdin=p1.stdout, stdout=subprocess.PIPE, text=True)
    p1.stdout.close()

    for line in p2.stdout:
        line = line.rstrip()
        print(f"\t{line}")

    p2.wait()

    if p2.returncode != 0:
        print("\tERROR", p2.returncode)
        print("\t\t", " ".join(p1_args), '|', ' '.join(p2_args))
    return p2.returncode

for m in ALL_MODES:
    margs = [f'-{_}' for _ in m]
    for o in ALL_OPS:
        r = run(m, o)
        if r != 0:
            sys.exit(r)
            #pass
