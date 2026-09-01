#!/usr/bin/env bash
# Seat 244. Seat 242 reported, at its section 1, that eighteen of thirty-two
# ratified rulings name no ratifier, and left two readings open: the field is
# optional and marks a different row shape, or eighteen rows are defective.
# This partitions them and asks whether any of the eighteen claims the governing
# tier with no human in the loop.
#
# The case that must fail: the classifier has to be able to report a row with no
# op-verbatim field. If every row comes back carrying one, the finding could be
# the classifier saying yes to everything. Section 3 is that control and it must
# print at least one row in each of its two columns.
set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
REG=mock/registry/ruling.toml

census() {
  awk '
    /^\[\[ruling\]\]/{ flush(); id="";kind="";rung="";by="";q=0;r=0 }
    /^id *= /{ if(id==""){s=$0;sub(/^id *= *"/,"",s);sub(/"$/,"",s);id=s} }
    /^kind *= /{ s=$0;sub(/^kind *= *"/,"",s);sub(/"$/,"",s);kind=s }
    /^rung *= /{ s=$0;sub(/^rung *= *"/,"",s);sub(/"$/,"",s);rung=s }
    /^ratified_by *= /{ s=$0;sub(/^ratified_by *= *"/,"",s);sub(/"$/,"",s);by=s }
    /^quote *= /{ q=1 }
    /^ratification *= /{ r=1 }
    END{ flush() }
    function flush(){ if(id!="") printf "%s\t%s\t%s\t%s\t%d\t%d\n", id, kind, rung, (by==""?"-":by), q, r }
  ' "$REG"
}

C=$(census)

echo "======== 1. The whole namespace, by rung"
printf '%s\n' "$C" | awk -F'\t' '{h[$3]++} END{for(k in h) printf "  rung %-10s %d\n", k, h[k]}' | sort
printf '  total rows %d\n' "$(printf '%s\n' "$C" | wc -l | tr -d ' ')"

echo
echo "======== 2. The ratified rows, partitioned by whether they name a ratifier"
printf '%s\n' "$C" | awk -F'\t' '$3=="ratified"{ if($4=="-") n++; else b[$4]++ }
  END{ printf "  ratified with no ratified_by : %d\n", n
       for(k in b) printf "  ratified_by = %-8s        : %d\n", k, b[k] }'

echo
echo "======== 3. CONTROL: can the classifier report a row carrying no op-verbatim field"
echo "  (a quote field, or a ratification field, is op's own words on the row)"
printf '%s\n' "$C" | awk -F'\t' '$3=="ratified"{
    if($5==0 && $6==0) noevid++;
    else if($5==1) hasq++;
    else hasr++ }
  END{ printf "  ratified rows with a quote            : %d\n", hasq
       printf "  ratified rows with only a ratification: %d\n", hasr
       printf "  ratified rows with NEITHER            : %d\n", noevid }'
echo "  Both columns of the control must be nonzero for the census to mean anything:"
printf '%s\n' "$C" | awk -F'\t' '$3=="ratified"{ if($5==1) a++; else b++ }
  END{ printf "    quote=1 rows: %d   quote=0 rows: %d\n", a, b
       if(a>0 && b>0) print "    CONTROL PASSES: the classifier distinguishes."
       else print "    CONTROL FAILED: the classifier says the same thing about every row." }'

echo
echo "======== 4. The eighteen, listed, with kind and what op-verbatim they carry"
printf '%s\n' "$C" | awk -F'\t' '$3=="ratified" && $4=="-"{
    printf "  %-70s kind=%-8s quote=%d ratification=%d\n", $1, $2, $5, $6 }'

echo
echo "======== 5. FINDING"
printf '%s\n' "$C" | awk -F'\t' '$3=="ratified" && $4=="-"{
    tot++; if($5==0 && $6==0) bare++ }
  END{ printf "  of the %d ratified rows naming no ratifier, %d carry no op-verbatim field at all.\n", tot, bare+0 }'
echo "  A row carrying op's own verbatim words does not need a ratified_by: the quote"
echo "  is the ratification. ratified_by records how a PANEL-DERIVED proposition was"
echo "  ratified, which is why the rows carrying it are the ones with a panel origin."
