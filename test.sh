#!/bin/env bash
Xephyr :2 -ac -br -noreset -screen 800x600 &
sleep 1
export DISPLAY=:2.0
target/debug/patina 2>&1 &
sleep 1
xterm