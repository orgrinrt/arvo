#!/bin/bash
# Anchor census: file:line-style references and probe-file names.
# Run from the panel directory.
set -u
P='[A-Za-z0-9_./-]+\.(rs|md|sh|toml|csv|txt):[0-9]+'
SRC="03_*.md 05_*.md 06_*.md 07_*.md 08_*.md 09_*.md 10_*.md 11_*.md 12_*.md 13_*.md 14_*.md 15_*.md 16_*.md 17_*.md 18_*.md 19_*.md 20_*.md"

echo "== file:line TOTAL occurrences =="
echo -n "sources: "; grep -ohE "$P" $SRC | wc -l
echo -n "MORNING: "; grep -ohE "$P" MORNING.md | wc -l

echo
echo "== file:line UNIQUE targets =="
echo -n "sources: "; grep -ohE "$P" $SRC | sort -u | wc -l
echo -n "MORNING: "; grep -ohE "$P" MORNING.md | sort -u | wc -l

echo
echo "== set difference: in sources, absent from MORNING (first 40) =="
comm -23 <(grep -ohE "$P" $SRC | sort -u) <(grep -ohE "$P" MORNING.md | sort -u) | head -40

echo
echo "== probe filenames actually on disk (basename stems) =="
ls */[a-z]*.rs 2>/dev/null | wc -l

echo
echo "== numeric tokens in MORNING vs anchors =="
echo -n "numeric tokens (>=1 digit, standalone): "; grep -ohE '\b[0-9][0-9,.]*(x|%)?\b' MORNING.md | wc -l
echo -n "file:line anchors:                      "; grep -ohE '[A-Za-z0-9_./-]+\.(rs|md|sh|toml|csv|out|py):[0-9]+' MORNING.md | wc -l
echo -n "probe stems (pNN style):              "; grep -ohE '`[a-z]{1,2}[0-9]{2}[a-z]?`' MORNING.md | wc -l
echo -n "probe FILENAMES (.rs/.py/.out):         "; grep -ohE '[a-z0-9_]+\.(rs|py|out|sh)' MORNING.md | wc -l
