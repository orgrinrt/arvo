#!/usr/bin/perl
# Check a reader's output against the files it cites. For each block, the
# passage (every quoted run of it) must appear in the cited file within the
# cited lines widened by a margin, comparing with whitespace, markdown quote
# marks and typographic quotes folded. Prints one line per block.
use strict; use warnings;
my ($out, $root, $margin) = @ARGV; $margin //= 12;
open my $h, '<', $out or die "$out: $!"; local $/; my $t = <$h>;
my @blocks = split /^(?=## )/m, $t;
sub fold { my $s = shift; $s =~ s/\\\n\s*//g; $s =~ s/^\s*>\s?//mg; $s =~ s/[\x{2018}\x{2019}]|\xE2\x80[\x98\x99]/'/g; $s =~ s/[\x{201C}\x{201D}]|\xE2\x80[\x9C\x9D]/"/g; $s =~ s/[*_`]//g; $s =~ s/\s+/ /g; lc $s }
my ($ok, $bad, $none) = (0, 0, 0);
for my $b (@blocks) {
  my ($slug) = $b =~ /^## (\S+)/ or next;
  my ($class) = $b =~ /^class:\s*(.+)$/m;
  my @ev = $b =~ /^evidence:\s*(.+)$/mg;
  my ($pass) = $b =~ /^passage:\s*(.+?)(?=^\w+:|\z)/ms;
  unless (@ev && defined $pass) { print "NOEVID  $class  $slug\n"; $none++; next; }
  my ($path, $a, $z) = $ev[0] =~ /^(\S+?):(\d+)(?:-(\d+))?/ or do { print "NOPARSE $class  $slug  $ev[0]\n"; $bad++; next; };
  $z //= $a;
  my $file = $path =~ m{^/} ? $path : "$root/$path";
  open my $f, '<', $file or do { print "NOFILE  $class  $slug  $path\n"; $bad++; next; };
  my @lines = split /\n/, <$f>;
  my $lo = $a - 1 - $margin; $lo = 0 if $lo < 0; my $hi = $z - 1 + $margin; $hi = $#lines if $hi > $#lines;
  my $win = fold(join "\n", @lines[$lo .. $hi]);
  my $whole = fold(join "\n", @lines);
  my @runs = $pass =~ /"([^"]{25,})"/g; @runs = ($pass) unless @runs;
  my ($hit, $far, $miss) = (0, 0, 0);
  for my $r (@runs) {
    my $q = fold($r); $q =~ s/^\s+|\s+$//g; $q =~ s/\s*\.\.\.\s*$//; $q = substr($q, 0, 70);
    next unless length $q > 20;
    if (index($win, $q) >= 0) { $hit++ } elsif (index($whole, $q) >= 0) { $far++ } else { $miss++ }
  }
  my $v = $miss ? 'MISS   ' : $far ? 'FAR    ' : $hit ? 'OK     ' : 'SHORT  ';
  $miss ? $bad++ : $ok++;
  print "$v $class  $slug  $path:$a-$z  hit=$hit far=$far miss=$miss\n";
}
print "ok=$ok bad=$bad noevidence=$none\n";
