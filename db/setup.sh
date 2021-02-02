#!/usr/bin/env bash
cd $(dirname "$0")
export DATABASE="blescanner.db"
sqlite3 ../blescanner.db < schema.sql
