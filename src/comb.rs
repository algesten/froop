//

macro_rules! mk_combine {
    ($name:ident => $($var:ident $rvar:ident $type:tt),+) => {
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
                    let $rvar = $var.remember_mode(MemoryMode::KeepAfterEnd);
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
                let pegs: Vec<_> = vec![
                    $(
                        {
                            let dispatch = dispatch.clone();
                            let inner_clone = inner_clone.clone();
                            $rvar.internal_subscribe(move |_, imit| {
                                let v = dispatch.clone()();
                                inner_clone.lock().update_owned(v, imit);
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
        let $rvar = $rvar.inner.lock();
        if let Some($var) = $rvar.peek_memory() {
            mk_combine!(last_value => [$($ovar)+] $($a)*);
        }
    };
    (last_value => [$($ovar:ident)+]) => {
        return Some(($($ovar.clone()),+));
    };
}

mk_combine!(combine2 => a ra A, b rb B);
mk_combine!(combine3 => a ra A, b rb B, c rc C);
mk_combine!(combine4 => a ra A, b rb B, c rc C, d rd D);
mk_combine!(combine5 => a ra A, b rb B, c rc C, d rd D, e re E);
mk_combine!(combine6 => a ra A, b rb B, c rc C, d rd D, e re E, f rf F);
mk_combine!(combine7 => a ra A, b rb B, c rc C, d rd D, e re E, f rf F, g rg G);
