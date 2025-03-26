PYTHON ?= python3

all:
	cargo build

# Runs all the coverage & permutation tests, then generates HTML report
# This will take a long time, but level2 tests will take even longer!
full-coverage:
	cargo llvm-cov clean
	cargo llvm-cov test --no-report
	$(PYTHON) testfloat-permute.py native coverage
	cargo llvm-cov report --html

coverage:
	cargo llvm-cov clean
	cargo llvm-cov test --html
