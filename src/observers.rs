use std::rc::Rc;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::cell::RefCell;
use std::marker::PhantomData;
use crate::*;
use crate::util::trait_alias::{CSS, ObserverSS};

impl<'a, V:Clone, E:Clone> Observer<V,E> for Box<Observer<V,E>+'a>
{
    #[inline(always)] fn next(&self, value: V) { Box::as_ref(self).next(value); }
    #[inline(always)] fn error(&self, e: E) { Box::as_ref(self).error(e); }
    #[inline(always)] fn complete(&self){ Box::as_ref(self).complete() }
}

impl<V:Clone, E:Clone> Observer<V,E> for Box<ObserverSS<V,E>>
{
    #[inline(always)] fn next(&self, value: V) { Box::as_ref(self).next(value); }
    #[inline(always)] fn error(&self, e: E) { Box::as_ref(self).error(e); }
    #[inline(always)] fn complete(&self){ Box::as_ref(self).complete() }
}

impl<'a, V:Clone, E:Clone> Observer<V,E> for Rc<Observer<V,E>+'a>
{
    #[inline(always)] fn next(&self, value: V) { Rc::as_ref(self).next(value); }
    #[inline(always)] fn error(&self, e: E) { Rc::as_ref(self).error(e); }
    #[inline(always)] fn complete(&self){ Rc::as_ref(self).complete() }
}

impl<'a, V:Clone, E:Clone> Observer<V,E> for Arc<Observer<V,E>+'a>
{
    #[inline(always)] fn next(&self, value: V) { Arc::as_ref(self).next(value); }
    #[inline(always)] fn error(&self, e: E) { Arc::as_ref(self).error(e); }
    #[inline(always)] fn complete(&self){ Arc::as_ref(self).complete() }
}

impl<V:Clone, E:Clone> Observer<V,E> for Arc<ObserverSS<V,E>>
{
    #[inline(always)] fn next(&self, value: V) { Arc::as_ref(self).next(value); }
    #[inline(always)] fn error(&self, e: E) { Arc::as_ref(self).error(e); }
    #[inline(always)] fn complete(&self){ Arc::as_ref(self).complete() }
}

impl<V:Clone, E:Clone, R, FN:Fn(V)->R> Observer<V,E> for FN
{
    #[inline(always)] fn next(&self, value: V) { self(value); }
    #[inline(always)] fn error(&self, _: E) {}
    #[inline(always)] fn complete(&self){}
}

impl<V:Clone, E:Clone, R, FN:Fn(V)->R> Observer<V,E> for (FN,())
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, _: E) {}
    #[inline(always)] fn complete(&self){}
}

impl<V:Clone, E:Clone, R, FN:Fn(V)->R> Observer<V,E> for (FN,(), ())
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, _: E) {}
    #[inline(always)] fn complete(&self){}
}

impl<V:Clone, E:Clone, RN, RE, FN:Fn(V)->RN, FE:Fn(E)->RE> Observer<V,E> for (FN,FE)
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, error: E){ self.1(error); }
    #[inline(always)] fn complete(&self){}
}

impl<V:Clone, E:Clone, RN, RE, FN:Fn(V)->RN, FE:Fn(E)->RE> Observer<V,E> for (FN,FE, ())
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, error: E){ self.1(error); }
    #[inline(always)] fn complete(&self){}
}

impl<V:Clone, E:Clone, RN, RC, FN:Fn(V)->RN, FC:Fn(())->RC> Observer<V,E> for (FN,(),FC)
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, _: E) {}
    #[inline(always)] fn complete(&self){ self.2(()); }
}

impl<V:Clone, E:Clone, RE, RC, FE:Fn(E)->RE, FC:Fn(())->RC> Observer<V,E> for ((),FE,FC)
{
    #[inline(always)] fn next(&self, _:V) {}
    #[inline(always)] fn error(&self, error: E){ self.1(error); }
    #[inline(always)] fn complete(&self){ self.2(()); }
}

impl<V:Clone, E:Clone, RE, FE:Fn(E)->RE> Observer<V,E> for ((),FE,())
{
    #[inline(always)] fn next(&self, _:V) {}
    #[inline(always)] fn error(&self, error: E){ self.1(error); }
    #[inline(always)] fn complete(&self){ }
}

impl<V:Clone, E:Clone, RC, FC:Fn(())->RC> Observer<V,E> for ((),(),FC)
{
    #[inline(always)] fn next(&self, _:V) {}
    #[inline(always)] fn error(&self, _: E) {}
    #[inline(always)] fn complete(&self){ self.2(()); }
}

impl<V:Clone, E:Clone, RN, RE, RC, FN:Fn(V)->RN, FE:Fn(E)->RE, FC:Fn(())->RC> Observer<V,E> for (FN,FE,FC)
{
    #[inline(always)] fn next(&self, value: V) { self.0(value); }
    #[inline(always)] fn error(&self, error: E){ self.1(error); }
    #[inline(always)] fn complete(&self){ self.2(()); }
}



#[cfg(test)]
mod test
{
    use std::rc::Rc;
    use crate::*;
    use crate::util::{ unmut, Clones };

    #[test]
    fn scope()
    {
        let mut arr = vec![1,2,3];

        {
            let (s, s1) = Rc::new(Subject::<i32, (), NO>::new()).clones();

            let r = &mut arr;
            s.sub(unmut(move |v| {}) );

            s.next(1);
            s.next(2);
        }

    }


}


