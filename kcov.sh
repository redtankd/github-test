#!/bin/bash

TEST_FILES=`find target/debug/ -maxdepth 1 -executable -type f`
echo TEST_FILES = $TEST_FILES
echo

echo collect coverage data
echo
for TEST_FILE in $TEST_FILES ; do
  echo TEST_FILE = $TEST_FILE
  mkdir -p $TRAVIS_BUILD_DIR/target/cov/$TEST_FILE
  kcov --verify \
    --strip-path=$TRAVIS_BUILD_DIR \
    --exclude-pattern=/.cargo,target \
    --exclude-region='#[cfg(test)]://[cfg(test)]' \
    $TRAVIS_BUILD_DIR/target/cov/$TEST_FILE \
    $TEST_FILE
done

echo
echo merge coverage data and upload coveralls
cd $TRAVIS_BUILD_DIR/target/cov/
pwd
mkdir -p kcov-merged
kcov --merge --coveralls-id=$TRAVIS_JOB_ID \
  --strip-path=$TRAVIS_BUILD_DIR \
  --exclude-pattern=/.cargo,target \
  --exclude-region='#[cfg(test)]://[cfg(test)]' \
  ./kcov-merged $TEST_FILES