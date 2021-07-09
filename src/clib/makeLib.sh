#!/bin/sh
clang -c -g -o libclib.o clib.c
ar -r -c -s libclib.a libclib.o
rm libclib.o
