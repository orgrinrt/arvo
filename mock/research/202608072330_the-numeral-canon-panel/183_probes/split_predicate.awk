# Split one predicate span into its phrases.
#
# The corpus uses two separators and one of them also occurs inside its values:
# topic six writes `W = 3, F = 0, operations in {add, sub, mul}, threads = 1`
# and topics seven and eight write the same thing with semicolons. Splitting on
# commas naively turns `{add, sub, mul}` into three phrases named `sub` and
# `mul`; splitting on semicolons only leaves topic six as one phrase, which is
# what the second version of this census did and why `radix` went missing from
# a key list built over nine spans that each contain it.
#
# So: split on `,` and `;` at brace depth zero, and nowhere else.
{
    depth = 0
    phrase = ""
    n = length($0)
    for (i = 1; i <= n; i++) {
        c = substr($0, i, 1)
        if (c == "{" || c == "(" || c == "[") depth++
        else if (c == "}" || c == ")" || c == "]") depth--
        if ((c == "," || c == ";") && depth <= 0) {
            print phrase
            phrase = ""
        } else {
            phrase = phrase c
        }
    }
    print phrase
}
