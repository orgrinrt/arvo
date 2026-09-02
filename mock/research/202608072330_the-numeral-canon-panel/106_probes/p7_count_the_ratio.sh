#!/bin/sh
# 106's opening paragraph makes a countable claim about what the file states
# against what it refutes, because op at 95 requires a unit to end in agreement
# and a consolidation that only refutes has failed its unit. This counts it from
# the file rather than from the paragraph. Run from the panel directory.
F=106_giesen_consolidation_the_strategy_axis.md

echo "definition clauses in section 1 (blockquote paragraphs):"
awk '/^## 1\. What a strategy is/{f=1} /^## 2\./{f=0} f' "$F" \
  | awk '/^> ./{if(!i){n++;i=1}} !/^> ./{i=0} END{printf "   %d\n", n}'

echo "findings placed by rung in section 3 (bolded lead-ins):"
awk '/^## 3\. What the unit settled/{f=1} /^## 4\./{f=0} f' "$F" \
  | grep -cE '^\*\*[A-Z`]' | sed 's/^/   /'

echo "claims this file refutes or corrects (section 12, numbered):"
awk '/^## 12\./{f=1} /^## 13\./{f=0} f' "$F" \
  | grep -cE '^\*\*(One|Two|Three|Four|Five)\.' | sed 's/^/   /'

echo "live options carried (13.1 + 13.2):"
awk '/^### 13\.1/{f=1} /^### 13\.3/{f=0} f' "$F" | grep -cE '^\*\*[A-Z`]' | sed 's/^/   /'

echo "options closed (13.3):"
awk '/^### 13\.3/{f=1} /^## 14\./{f=0} f' "$F" | grep -cE '^\*\*[A-Z`]' | sed 's/^/   /'
