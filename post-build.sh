#!/bin/sh

wasm-opt dist/.stage/*.wasm -Oz -all -o dist/.stage/*.wasm
