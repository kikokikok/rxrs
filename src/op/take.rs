use crate::*;
use std::marker::PhantomData;
use std::sync::Arc;
use std::cell::UnsafeCell;

pub struct TakeOp<SS, Src>
{
    count: usize,
    src: Src,
    PhantomData: PhantomData<(SS)>
}

pub trait ObsTakeOp<SS, VBy, EBy> : Sized
{
    fn take(self, count: usize) -> TakeOp<SS, Self> { TakeOp{ count, src: self, PhantomData} }
}

impl<'o, VBy: RefOrVal, EBy: RefOrVal, Src: Observable<'o, SS, VBy, EBy>+'o, SS:YesNo>
ObsTakeOp<SS, VBy,EBy>
for Src {}


pub trait DynObsTakeOp<'o, SS: YesNo, VBy: RefOrVal+'o, EBy: RefOrVal+'o>
{
    fn take(self, count: usize) -> Self;
}

impl<'o, SS:YesNo, VBy: RefOrVal+'o, EBy: RefOrVal+'o>
DynObsTakeOp<'o, SS, VBy,EBy>
for DynObservable<'o, 'o, SS, VBy, EBy>
{
    fn take(self, count: usize) -> Self
    { TakeOp{ count, src: self.src, PhantomData }.into_dyn() }
}

impl<'o, SS:YesNo, VBy: RefOrVal+'o, EBy: RefOrVal+'o, Src: Observable<'o, SS, VBy, EBy>>
Observable<'o, SS, VBy, EBy>
for TakeOp<SS, Src>
{
    fn subscribe(&self, next: impl ActNext<'o, SS, VBy>, ec: impl ActEc<'o, SS, EBy>) -> Unsub<'o, SS> where Self: Sized
    {
        if self.count == 0 {
            ec.call_once(None);
            return Unsub::done();
        }

        let next = SSActNextWrap::new(next);
        let sub = Unsub::new();
        let state = Arc::new(unsafe{ AnySendSync::new(UnsafeCell::new((self.count, Some(ec)))) });

        sub.clone().added_each(self.src.subscribe(
            forward_next(next, (sub.clone(), SSWrap::new(state.clone())), |next, (sub, state), v:VBy| {
                sub.if_not_done(|| {
                    let state = unsafe{ &mut *state.get() };

                    let mut val = state.0;
                    if val != 0 {
                        val -= 1;
                        state.0 -= 1;
                        next.call(v.into_v());
                    }
                    if val == 0 {
                        sub.unsub_then(|| state.1.take().map_or((), |ec| ec.call_once(None)));
                    }
                });
            }, |s, (sub, _state)| (s.stopped() || sub.is_done())),

            forward_ec((sub, SSWrap::new(state)), |(sub, state), e:Option<EBy>| {
                sub.unsub_then(|| unsafe{ &mut *state.get() }.1.take().map_or((), |ec| ec.call_once(e.map(|e| e.into_v()))))
            })
        ))
    }

    fn subscribe_dyn(&self, next: Box<ActNext<'o, SS, VBy>>, ec: Box<ActEcBox<'o, SS, EBy>>) -> Unsub<'o, SS>
    { self.subscribe(next, ec) }
}

#[cfg(test)]
mod test
{
    use crate::*;
    use crate::util::clones::*;

    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn smoke()
    {
        let (n, n1, n2) = Rc::new(Cell::new(0)).clones();
        let (s, s1) = Rc::new(Subject::<NO, i32>::new()).clones();

        s.take(3).subscribe(
            |v:&_| { n.replace(*v); },
            |_e:Option<&_>| { n1.replace(n1.get() + 100); }
        );

        s1.next(1);
        assert_eq!(n2.get(), 1);

        s1.next(2);
        assert_eq!(n2.get(), 2);

        s1.next(3);
        assert_eq!(n2.get(), 103);

        s1.next(4);
        assert_eq!(n2.get(), 103);

        s1.complete();
        assert_eq!(n2.get(), 103);
    }

    #[test]
    fn of()
    {
        let n = Cell::new(0);
        Of::value(123).take(100).subscribe(|v:&_| { n.replace(*v); }, |_e:Option<&_>| { n.replace(n.get() + 100); });

        assert_eq!(n.get(), 223);
    }

    #[test]
    fn zero()
    {
        let n = Cell::new(0);
        Of::value(123).take(0).subscribe(|v:&_| { n.replace(*v); }, |_e:Option<&_>| { n.replace(n.get() + 100); });

        assert_eq!(n.get(), 100);
    }

}
