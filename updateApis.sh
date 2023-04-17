#!/bin/bash

subxt codegen --url ws://localhost:9944 | rustfmt --edition=2021 > src/chain_apis/energychain_api.rs
