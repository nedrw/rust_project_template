#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Template Combinatorial Test Script
#
# Iterates all 2^5 = 32 feature flag combinations, runs
# `cargo generate` + `cargo check` for each, and reports results.
#
# Usage:
#   ./test-template.sh              # Test all 32 combinations
#   ./test-template.sh --quick      # Test only 5 representative combinations
#   ./test-template.sh --clean      # Remove test artifacts after run
# ============================================================

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TEMPLATE_DIR="$(cd "$(dirname "$0")" && pwd)"
WORK_DIR="${TEMPLATE_DIR}/test-output"
QUICK_MODE=false
CLEAN_AFTER=false

for arg in "$@"; do
    case "$arg" in
        --quick) QUICK_MODE=true ;;
        --clean) CLEAN_AFTER=true ;;
        --help|-h)
            echo "Usage: $0 [--quick] [--clean]"
            echo "  --quick   Test only 5 representative combinations"
            echo "  --clean   Remove test-output/ after run"
            exit 0
            ;;
    esac
done

# --- Prerequisites ---
command -v cargo-generate >/dev/null 2>&1 || {
    echo -e "${RED}ERROR: cargo-generate not found.${NC}"
    echo "Install with: cargo install cargo-generate"
    exit 1
}

command -v cargo >/dev/null 2>&1 || {
    echo -e "${RED}ERROR: cargo not found.${NC}"
    exit 1
}

# --- Setup ---
mkdir -p "$WORK_DIR"
echo "Test artifacts will be created in: $WORK_DIR"
echo ""

PASSED=()
FAILED=()

# --- Test a single combination ---
test_one() {
    local logging="$1" tracing="$2" jemalloc="$3" settings="$4" cli="$5"
    local label="${logging:0:1}${tracing:0:1}${jemalloc:0:1}${settings:0:1}${cli:0:1}"
    #           L      T      J      S      C

    printf "  %-8s  " "$label"

    local proj_dir="$WORK_DIR/$label"
    rm -rf "$proj_dir" 2>/dev/null || true

    # Generate project via cargo-generate (non-interactive: all placeholders defined)
    local gen_out
    gen_out=$(cargo generate --path "$TEMPLATE_DIR" --name "$label" --destination "$WORK_DIR" \
        --define "project_description=Test project ($label)" \
        --define "author_name=TestRunner" \
        --define "author_email=test@example.com" \
        --define "use_logging=$logging" \
        --define "use_tracing=$tracing" \
        --define "use_jemalloc=$jemalloc" \
        --define "use_settings=$settings" \
        --define "use_cli=$cli" \
        2>&1) || {
        echo -e "${RED}FAIL${NC} (generate: ${gen_out:0:80})"
        FAILED+=("$label")
        return 1
    }

    # Compile check
    local check_out
    check_out=$(cd "$proj_dir" && cargo check 2>&1) || {
        local err_summary
        err_summary=$(echo "$check_out" | grep "^error" | head -3 | tr '\n' '; ')
        echo -e "${RED}FAIL${NC} (check: ${err_summary:0:100})"
        FAILED+=("$label")
        return 1
    }

    echo -e "${GREEN}PASS${NC}"
    PASSED+=("$label")
}

# --- Representative quick set (5 combos) ---
test_quick() {
    echo "=== Quick Mode: 5 representative combinations ==="
    echo ""
    printf "  %-8s  %s\n" "Combo" "Result"
    printf "  %-8s  %s\n" "--------" "------"

    test_one "true"  "true"  "true"  "true"  "true"   # All on
    test_one "false" "false" "false" "false" "false"  # All off (minimal)
    test_one "true"  "false" "false" "false" "false"  # Logging only
    test_one "false" "false" "true"  "false" "false"  # jemalloc only
    test_one "false" "false" "false" "true"  "true"   # Settings + CLI (sync)
}

# --- Full combinatorial (32 combos) ---
test_all() {
    echo "=== Full Mode: 32 combinations ==="
    echo ""
    printf "  %-8s  %s\n" "Combo" "Result"
    printf "  %-8s  %s\n" "--------" "------"

    for logging in true false; do
    for tracing in true false; do
    for jemalloc in true false; do
    for settings in true false; do
    for cli in true false; do
        test_one "$logging" "$tracing" "$jemalloc" "$settings" "$cli"
    done
    done
    done
    done
    done
}

# --- Run ---
if $QUICK_MODE; then
    test_quick
else
    test_all
fi

# --- Summary ---
echo ""
echo "=========================================="
echo -e "Results: ${GREEN}${#PASSED[@]} passed${NC} / ${RED}${#FAILED[@]} failed${NC} / $(( ${#PASSED[@]} + ${#FAILED[@]} )) total"
echo ""

if [ ${#PASSED[@]} -gt 0 ]; then
    echo "Passed: ${PASSED[*]}"
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    echo -e "Failed: ${RED}${FAILED[*]}${NC}"
fi

# --- Cleanup ---
if $CLEAN_AFTER; then
    echo ""
    echo "Cleaning up $WORK_DIR ..."
    rm -rf "$WORK_DIR"
else
    echo ""
    echo "Test artifacts kept at: $WORK_DIR"
    echo "Run with --clean to remove them."
fi

# Exit with failure if any combo failed
[ ${#FAILED[@]} -eq 0 ] || exit 1
