# Counts how many inventory rows mention the strategy axis, and how many name it
# inside their canon sentence (the leading blockquote). Run from the panel directory.
import io, re

s = io.open("23_spj_the_sentences_a_canon_could_carry.md", encoding="utf-8").read()

parts = re.split(r"^#### (S\d+b?)\.", s, flags=re.M)
rows = {}
for i in range(1, len(parts), 2):
    rows[parts[i]] = parts[i + 1]

key = lambda x: (len(x), x)

anywhere = [k for k, v in rows.items() if re.search(r"strateg", v, re.I)]
print("rows total: %d" % len(rows))
print("rows mentioning 'strateg' anywhere: %d -> %s"
      % (len(anywhere), ", ".join(sorted(anywhere, key=key))))

in_sentence = []
for k, v in rows.items():
    m = re.search(r"\n(> .+?)\n\n", v, re.S)
    if m and re.search(r"strateg", m.group(1), re.I):
        in_sentence.append(k)
print("rows whose canon sentence says 'strateg': %d -> %s"
      % (len(in_sentence), ", ".join(sorted(in_sentence, key=key))))
