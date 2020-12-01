#!/bin/bash
#  ===== Date Created: 29 November, 2020 ===== 

# Colours
RED="\033[0;31m"
GREEN="\033[0;92m"
BLUE="\033[0;36m"
DEFAULT_COLOUR="\033[0m"

# Argument one is the colour (ANSI escape sequence), argument two is
# the string to echo
print()
{
	echo -e "$1$2${DEFAULT_COLOUR}"
}

failed_build()
{
	print $RED "\nBuild failed!"
	popd > /dev/null
	exit 1
}

compileFlags="-Wall -Wextra -o aoc2020"
linkFlags=""
files="../src/main.cpp"

mkdir -p bin
pushd bin > /dev/null

print $BLUE "Building..."
clang++ $compileFlags $linkFlags $files || failed_build

print $GREEN "Build succeeded!"
print $BLUE "\nRunning..."

popd > /dev/null

bin/aoc2020 "$@"
echo
