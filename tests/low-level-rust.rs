

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");


def_benchmarks! (
		
		
		
		
		benchmark__box_vs_rc__box_clone => {
				with_values
					StdBox::clone (wrapper),
					wrapper => StdBox::new (0),
			},
		
		benchmark__box_vs_rc__rc_clone => {
				with_values
					StdRc::clone (wrapper),
					wrapper => StdRc::new (0),
			},
		
		
		benchmark__box_vs_rc__box_as_ref => {
				with_values
					StdBox::deref (wrapper),
					wrapper => StdBox::new (0),
			},
		
		benchmark__box_vs_rc__rc_as_ref => {
				with_values
					StdRc::deref (wrapper),
					wrapper => StdRc::new (0),
			},
		
		
		
		
		benchmark__internals__rc_only__clone => {
				with_values
					StdRc::clone (wrapper),
					wrapper => StdRc::new ((0, 0)),
			},
		
		benchmark__internals__box_rc__clone => {
				with_values
					StdBox::clone (wrapper),
					wrapper => StdBox::new ((0, StdRc::new (0))),
			},
		
		benchmark__internals__rc_rc__clone => {
				with_values
					StdRc::clone (wrapper),
					wrapper => StdRc::new ((0, StdRc::new (0))),
			},
		
		
		
		
	);

