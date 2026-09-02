# One line per [[proposal]] record:  id <TAB> standing <TAB> n distinct files <TAB> files
# Two anchors into one file are ONE file, which is the case a count of
# provenance entries gets wrong and the case the registry contains.
function flush(   i,f,c,last,out,n) {
  if (id == "") return
  n = 0; out = ""
  for (i = 1; i <= np; i++) {
    f = prov[i]; c = split(f, seg, "::")
    if (c >= 2) { last = seg[c]
      if ((last ~ /^#/ || last ~ /^[0-9]+$/) && c >= 3) f = seg[c-1]; else f = last
    } else f = ""
    if (f != "" && !(f in seen)) { seen[f] = 1; n++; out = out (out == "" ? "" : ",") f }
  }
  printf "%s\t%s\t%d\t%s\n", id, (standing == "" ? "-" : standing), n, (out == "" ? "-" : out)
  delete seen; delete prov; np = 0; id = ""; standing = ""
}
/^\[\[proposal\]\]/ { flush(); inprov = 0; next }
/^id = /       { id = $0;       sub(/^id = "/, "", id);             sub(/"$/, "", id);       next }
/^standing = / { standing = $0; sub(/^standing = "/, "", standing); sub(/"$/, "", standing); next }
/^provenance = \[/ {
  inprov = 1; line = $0; sub(/^provenance = \[/, "", line)
  if (line ~ /\]/) { inprov = 0; sub(/\].*$/, "", line) }
  n = split(line, parts, ",")
  for (i = 1; i <= n; i++) { v = parts[i]; gsub(/^[ \t"]+|[ \t",]+$/, "", v); if (v != "") prov[++np] = v }
  next
}
inprov == 1 {
  line = $0
  if (line ~ /^\]/) { inprov = 0; next }
  gsub(/^[ \t"]+|[ \t",]+$/, "", line)
  if (line != "") prov[++np] = line
  next
}
END { flush() }
