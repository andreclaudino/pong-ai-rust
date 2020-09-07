#!/usr/bin/env bash
clear;clear; P2_IS_REMOTE=true P1_IS_REMOTE=true P1_BOT_URL="http://0.0.0.0:8081/agent" P2_BOT_URL="http://0.0.0.0:8080/agent" target/release/pong
