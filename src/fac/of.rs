use crate::{YesNo, YES, NO, Observable, ObservableSendSync, Observer, Subscription};

#[derive(Copy, Clone)]
pub struct Of<V: Clone, SS>(V, SS);

pub fn of<V:Clone, SS>(v:V, s:SS) -> Of<V, SS>
{
    Of(v, s)
}

impl<V> !Send for Of<V, NO> {}
impl<V> !Sync for Of<V, NO> {}

impl<'s, 'o, V: Clone> Observable<'s, 'o, V, ()> for Of<V, NO>
{
    fn subscribe(&'s self, observer: impl Observer<V,()>+'o) -> Subscription<'o, NO>
    {
        observer.next(self.0.clone());
        observer.complete();
        Subscription::done()
    }
}

impl<'s, 'o, V: Clone+Send+Sync> ObservableSendSync<'s, V, ()> for Of<V, YES>
{
    fn subscribe(&'s self, observer: impl Observer<V,()>+Send+Sync+'static) -> Subscription<'static, YES>
    {
        observer.next(self.0.clone());
        observer.complete();
        Subscription::done()
    }
}


#[cfg(test)]
mod test
{
    use crate::{YesNo, YES, NO, Observable, ObservableSendSync, Observer, Subscription};
    use super::of;
    use std::sync::atomic::*;

    #[test]
    fn smoke()
    {
        let o = of(123, NO);
        o.subscribe(|v| println!("it works: {}", v));

        let o = of(456, YES);
        o.subscribe(|v| println!("it works: {}", v));

        ::std::thread::spawn(move ||{
            o.subscribe(|v| println!("it works: {}", v));
        }).join();
    }

    #[test]
    fn side_effects()
    {
        let cell = ::std::cell::Cell::new(123);
        let o = of(456, NO);
        o.subscribe(|v| { cell.replace(v); });
        assert_eq!(cell.get(), 456);

        let arc = ::std::sync::Arc::new(AtomicI32::new(123));
        let o = of(456, YES);
        let arclone = arc.clone();
        ::std::thread::spawn(move || {
            o.subscribe(move |v| { arclone.store(v, Ordering::SeqCst); });
        }).join();
        assert_eq!(arc.load(Ordering::SeqCst), 456);
    }

    #[test]
    fn complete()
    {
        let o = of(123, NO);
        let sub = o.subscribe(|v|{});

        assert!(sub.is_done());
    }
}