#!/bin/sh
cd "$(dirname "$0")"
npx playwright test "$@"
