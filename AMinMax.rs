trait AMinMax {
    fn amin(&mut self, x: Self);
    fn amax(&mut self, x: Self);
}
impl<T: PartialOrd> AMinMax for T {
    fn amin(&mut self, x: Self) { if *self > x { *self = x; } }
    fn amax(&mut self, x: Self) { if *self < x { *self = x; } }
}
