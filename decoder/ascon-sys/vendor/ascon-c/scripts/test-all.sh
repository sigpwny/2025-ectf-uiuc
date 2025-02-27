#!/bin/bash

if [ -z $1 ]; then
  echo "Usage: $0 [-Os | -O0 | -O1 | -O2 | -O3] [Release | Debug]"
  exit 1
fi

if [[ $# -lt 1 ]]; then
  OPT=-O2
  TYPE=Release
elif [[ $# -lt 2 ]]; then
  OPT=$1
  TYPE=Release
else
  OPT=$1
  TYPE=$2
fi

CLIST="bi32;bi32_lowreg;bi32_lowsize;bi8;esp32;opt32;opt32_lowsize;opt64;opt64_lowsize;opt8;opt8_lowsize;ref"

exec 3>&1 4>&2
exec 1>test-all.log 2>&1

mkdir test-all
cd test-all

echo
echo "Test clang $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=clang -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -UEMULATOR \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;-march=native;-mtune=native" \
         -DIMPL_LIST=$CLIST
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test gcc $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -UEMULATOR \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;-march=native;-mtune=native" \
         -DIMPL_LIST=$CLIST
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test avx512 $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="sde;--" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow;-fsanitize=address,undefined;-march=icelake-client" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;-march=icelake-client" \
         -DIMPL_LIST="avx512;ref"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test neon $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=arm-linux-gnueabi-gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="qemu-arm;-L;/usr/arm-linux-gnueabi" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow;-fsanitize=address,undefined;-latomic;-march=armv7-a" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;-march=armv7-a" \
         -DIMPL_LIST="neon;ref"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test armv7m $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=arm-linux-gnueabi-gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="qemu-arm;-L;/usr/arm-linux-gnueabi" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;-march=armv7" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow;-fsanitize=address,undefined;-latomic;-march=armv7" \
         -DIMPL_LIST="$CLIST;armv6;armv6_lowsize;armv7m;armv7m_lowsize;armv7m_small;bi32_armv6;bi32_armv7m;bi32_armv7m_small;protected_bi32_armv6;protected_bi32_armv6_leveled"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test armv6m $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=arm-none-eabi-gcc -DCMAKE_C_COMPILER_FORCED=ON -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="qemu-system-arm;-semihosting;-nographic;-machine;microbit;-kernel" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;--specs=picolibc.specs;--oslib=semihost;-T../tests/microbit.ld;-mcpu=cortex-m0" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow;--specs=picolibc.specs;--oslib=semihost;-T../tests/microbit.ld;-mcpu=cortex-m0" \
         -DIMPL_LIST="armv6m;armv6m_lowsize;ref"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test rv32 $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=riscv64-unknown-elf-gcc -DCMAKE_C_COMPILER_FORCED=ON -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="qemu-system-riscv32;-semihosting;-nographic;-machine;virt;-cpu;rv32;-bios;none;-kernel" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer;--specs=picolibc.specs;--oslib=semihost;-T../tests/rv32.ld;-march=rv32i;-mabi=ilp32" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow;--specs=picolibc.specs;--oslib=semihost;-T../tests/rv32.ld;-march=rv32i;-mabi=ilp32" \
         -DIMPL_LIST="asm_rv32i;ref"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3


echo
echo "Test mipsel $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=mipsel-linux-gnu-gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST \
         -DEMULATOR="qemu-mipsel;-L;/usr/mipsel-linux-gnu" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow" \
         -DIMPL_LIST=$CLIST
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

echo
echo "Test mips $OPT builds:" | tee -a ../test-all.log | grep Test >&3
rm -f CMakeCache.txt
cmake .. -DCMAKE_C_COMPILER=mips-linux-gnu-gcc -DALG_LIST="" -DIMPL_LIST=""
cmake .. -DCMAKE_BUILD_TYPE=$TYPE -UALG_LIST  \
         -DEMULATOR="qemu-mips;-L;/usr/mips-linux-gnu" \
         -DREL_FLAGS="$OPT;-fomit-frame-pointer" \
         -DDBG_FLAGS="$OPT;-std=c99;-Wall;-Wextra;-Wshadow" \
         -DIMPL_LIST="esp32;opt32;opt32_lowsize;opt64;opt64_lowsize;ref"
cmake --build . --clean-first -- -k | tee -a ../test-all.log | grep "Built target genkat" >&3
ctest | sed -u 's/[0-9.]* sec//g' | tee -a ../test-all.log | grep Test >&3

cd ..
rm -rf test-all

exit $(grep -c Failed test-all.log)
