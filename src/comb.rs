//

macro_rules! mk_combine {
    ($name:ident => $alive:tt $($var:ident $rvar:ident $type:tt),+) => {
      impl<$($type,)+> Stream<($($type,)+)>
            where
              $( $type: Clone, )+
        {
            /// Combine a number of streams into one.
            ///
            /// The resulting stream emits when any of the incoming streams emit, but only
            /// when all incoming have had an initial value.
            pub fn $name($( $var: &Stream<$type>, )+) -> Stream<($( $type, )+)> {
                let inner = SafeInner::new(MemoryMode::NoMemory, None);
                let inner_clone = inner.clone();
                $(
                    let $rvar: Arc<Mutex<Option<$type>>> = Arc::new(Mutex::new(None));
                )+
                let dispatch = {
                    $(
                        let $rvar = $rvar.clone();
                    )+
                    move || {
                        $(
                            let $rvar = $rvar;
                        )+
                        mk_combine!(last_value => [$($var)+] $($var $rvar)+);
                        None
                    }
                };
                let alive = Arc::new(AtomicUsize::new($alive));
                let pegs: Vec<_> = vec![
                    $(
                        {
                            let dispatch = dispatch.clone();
                            let inner_clone = inner_clone.clone();
                            let alive = alive.clone();
                            let $rvar = $rvar.clone();
                            $var.internal_subscribe(move |t| {
                                if let Some(t) = t {
                                    {
                                        let mut $rvar = $rvar.lock().unwrap();
                                        *$rvar = Some(t.clone());
                                    }
                                    let v = dispatch.clone()();
                                    if v.is_some() {
                                        inner_clone.lock().update_owned(v);
                                    }
                                } else if alive.fetch_sub(1, Ordering::SeqCst) == 1 {
                                    inner_clone.lock().update_owned(None);
                                }
                        })
                        }
                    ),+
                ];
                let peg = Peg::many(pegs);
                Stream { peg, inner }
            }
        }
    };
    (last_value => [$($ovar:ident)+] $var:ident $rvar:ident $($a:tt)*) => {
        let $rvar = $rvar.lock().unwrap();
        if let Some($var) = $rvar.as_ref() {
            mk_combine!(last_value => [$($ovar)+] $($a)*);
        }
    };
    (last_value => [$($ovar:ident)+]) => {
        return Some(($($ovar.clone()),+));
    };
}

mk_combine!(combine2 => 2 a ra A, b rb B);
mk_combine!(combine3 => 3 a ra A, b rb B, c rc C);
mk_combine!(combine4 => 4 a ra A, b rb B, c rc C, d rd D);
mk_combine!(combine5 => 5 a ra A, b rb B, c rc C, d rd D, e re E);
mk_combine!(combine6 => 6 a ra A, b rb B, c rc C, d rd D, e re E, f rf F);
mk_combine!(combine7 => 7 a ra A, b rb B, c rc C, d rd D, e re E, f rf F, g rg G);
