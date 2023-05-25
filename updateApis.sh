#!/bin/bash

subxt codegen --url http://localhost:9933 | rustfmt --edition=2021 > src/chain_apis/energychain_api.rs
