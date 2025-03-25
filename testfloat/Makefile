TESTFLOAT_OPTS="-DFLOAT32 -DFLOAT64 -DFLOAT_ROUND_ODD"
RISCV_CC=riscv64-linux-gnu-gcc
RISCV_MARCH=rv64gc
SIM=../../target/debug/run

all: gen

berkeley-testfloat-3:
	git clone https://github.com/ucb-bar/berkeley-testfloat-3.git

berkeley-softfloat-3:
	git clone https://github.com/ucb-bar/berkeley-softfloat-3.git

berkeley-softfloat-3/build/Linux-x86_64-GCC/softfloat.a: berkeley-softfloat-3
	$(MAKE) -C berkeley-softfloat-3/build/Linux-x86_64-GCC SPECIALIZE_TYPE=RISCV

berkeley-softfloat-3/build/Linux-RISCV64-GCC/softfloat.a: berkeley-softfloat-3
	$(MAKE) -C berkeley-softfloat-3/build/Linux-RISCV64-GCC CC=$(RISCV_CC) MARCH=$(RISCV_MARCH)

berkeley-testfloat-3/build/Linux-x86_64-GCC/testfloat: berkeley-softfloat-3/build/Linux-x86_64-GCC/softfloat.a berkeley-testfloat-3
	$(MAKE) -C berkeley-testfloat-3/build/Linux-x86_64-GCC TESTFLOAT_OPTS=$(TESTFLOAT_OPTS)

berkeley-testfloat-3/build/Linux-RISCV64-GCC/testfloat: berkeley-softfloat-3/build/Linux-RISCV64-GCC/softfloat.a berkeley-testfloat-3
	$(MAKE) -C berkeley-testfloat-3/build/Linux-RISCV64-GCC CC=$(RISCV_CC) MARCH=$(RISCV_MARCH) LINK_FLAGS=-static TESTFLOAT_OPTS=$(TESTFLOAT_OPTS)

clear-libs:
	rm -f berkeley-softfloat-3/build/Linux-x86_64-GCC/softfloat.a \
		  berkeley-softfloat-3/build/Linux-RISCV64-GCC/softfloat.a \
		  berkeley-testfloat-3/build/Linux-x86_64-GCC/testfloat \
		  berkeley-testfloat-3/build/Linux-RISCV64-GCC/testfloat

$(SIM):
	cd ../../ && cargo build

.PHONY: gen
gen: berkeley-testfloat-3/build/Linux-x86_64-GCC/testfloat \
	 berkeley-testfloat-3/build/Linux-RISCV64-GCC/testfloat

.PHONY: run-testfloat
run-testfloat: berkeley-testfloat-3/build/Linux-RISCV64-GCC/testfloat $(SIM)
	$(SIM) $< -- -list
	$(SIM) $< -- -checkAll -all1 -all2 -level 2 -errorstop

.PHONY: clean
clean:
	$(MAKE) -C berkeley-softfloat-3/build/Linux-RISCV64-GCC/ clean
	$(MAKE) -C berkeley-softfloat-3/build/Linux-x86_64-GCC/ clean
	$(MAKE) -C berkeley-testfloat-3/build/Linux-RISCV64-GCC/ clean
	$(MAKE) -C berkeley-testfloat-3/build/Linux-x86_64-GCC/ clean

.PHONY: distclean
distclean:
	rm -rf berkeley-softfloat-3 berkeley-testfloat-3
