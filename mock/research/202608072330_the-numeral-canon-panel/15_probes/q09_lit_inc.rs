// GENERATED. The bridge, and its ENTIRE domain is the literals the programs
// in this directory write: 0, 1, 3, 4, 5, 13, 20, 26, 40, 200.
impl Lit for L<0> {
    type N = Z;
}
impl Lit for L<1> {
    type N = O<Z>;
}
impl Lit for L<3> {
    type N = O<O<Z>>;
}
impl Lit for L<4> {
    type N = E<E<O<Z>>>;
}
impl Lit for L<5> {
    type N = O<E<O<Z>>>;
}
impl Lit for L<13> {
    type N = O<E<O<O<Z>>>>;
}
impl Lit for L<20> {
    type N = E<E<O<E<O<Z>>>>>;
}
impl Lit for L<26> {
    type N = E<O<E<O<O<Z>>>>>;
}
impl Lit for L<40> {
    type N = E<E<E<O<E<O<Z>>>>>>;
}
impl Lit for L<200> {
    type N = E<E<E<O<E<E<O<O<Z>>>>>>>>;
}
