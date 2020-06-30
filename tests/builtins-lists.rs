

#![ feature (test) ]
#![ no_implicit_prelude ]
#![ allow (deprecated) ] // FIXME
#![ allow (bare_trait_objects) ] // FIXME
include! ("prelude.in");




def_benchmarks! (
		
		
		
		
		benchmark__list_empty => {
				list_empty ()
			},
		
		
		benchmark__list_build_1__immutable => {
				list_build_1 (&VOID_VALUE, None, Some (true))
			},
		
		benchmark__list_build_2__immutable => {
				list_build_2 (&VOID_VALUE, &VOID_VALUE, None, Some (true))
			},
		
		benchmark__list_build_3__immutable => {
				list_build_3 (&VOID_VALUE, &VOID_VALUE, &VOID_VALUE, None, Some (true))
			},
		
		benchmark__list_build_4__immutable => {
				list_build_4 (&VOID_VALUE, &VOID_VALUE, &VOID_VALUE, &VOID_VALUE, None, Some (true))
			},
		
		
		benchmark__list_build_1__mutable => {
				list_build_1 (&VOID_VALUE, None, Some (false))
			},
		
		benchmark__list_build_2__mutable => {
				list_build_2 (&VOID_VALUE, &VOID_VALUE, None, Some (false))
			},
		
		benchmark__list_build_3__mutable => {
				list_build_3 (&VOID_VALUE, &VOID_VALUE, &VOID_VALUE, None, Some (false))
			},
		
		benchmark__list_build_4__mutable => {
				list_build_4 (&VOID_VALUE, &VOID_VALUE, &VOID_VALUE, &VOID_VALUE, None, Some (false))
			},
		
		
		benchmark__list_make__0__immutable => {
				list_make (0, Some (&VOID_VALUE), Some (true))
			},
		
		benchmark__list_make__1__immutable => {
				list_make (1, Some (&VOID_VALUE), Some (true))
			},
		
		benchmark__list_make__10__immutable => {
				list_make (10, Some (&VOID_VALUE), Some (true))
			},
		
		benchmark__list_make__100__immutable => {
				list_make (100, Some (&VOID_VALUE), Some (true))
			},
		
		benchmark__list_make__1000__immutable => {
				list_make (1000, Some (&VOID_VALUE), Some (true))
			},
		
		benchmark__list_make__10000__immutable => {
				list_make (10000, Some (&VOID_VALUE), Some (true))
			},
		
		
		benchmark__list_make__0__mutable => {
				list_make (0, Some (&VOID_VALUE), Some (false))
			},
		
		benchmark__list_make__1__mutable => {
				list_make (1, Some (&VOID_VALUE), Some (false))
			},
		
		benchmark__list_make__10__mutable => {
				list_make (10, Some (&VOID_VALUE), Some (false))
			},
		
		benchmark__list_make__100__mutable => {
				list_make (100, Some (&VOID_VALUE), Some (false))
			},
		
		benchmark__list_make__1000__mutable => {
				list_make (1000, Some (&VOID_VALUE), Some (false))
			},
		
		benchmark__list_make__10000__mutable => {
				list_make (10000, Some (&VOID_VALUE), Some (false))
			},
		
		
		
		
		benchmark__list_length__0 => {
				with_values
					list_length (list),
					list => list_empty (),
			},
		
		
		benchmark__list_length__1__immutable => {
				with_values
					list_length (list),
					list => list_build_1 (&VOID_VALUE, None, Some (true)),
			},
		
		benchmark__list_length__10__immutable => {
				with_values
					list_length (list),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_length__100__immutable => {
				with_values
					list_length (list),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_length__1000__immutable => {
				with_values
					list_length (list),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_length__10000__immutable => {
				with_values
					list_length (list),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (true))),
			},
		
		
		benchmark__list_length__1__mutable => {
				with_values
					list_length (list),
					list => list_build_1 (&VOID_VALUE, None, Some (false)),
			},
		
		benchmark__list_length__10__mutable => {
				with_values
					list_length (list),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_length__100__mutable => {
				with_values
					list_length (list),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_length__1000__mutable => {
				with_values
					list_length (list),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_length__10000__mutable => {
				with_values
					list_length (list),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (false))),
			},
		
		
		
		
		benchmark__list_clone__0__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => list_empty (),
			},
		
		benchmark__list_clone__1__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => list_build_1 (&VOID_VALUE, None, Some (true)),
			},
		
		benchmark__list_clone__10__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_clone__100__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_clone__1000__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_clone__10000__immutable => {
				with_values
					list_clone (list, Some (true)),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (true))),
			},
		
		
		benchmark__list_clone__0__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => list_empty (),
			},
		
		benchmark__list_clone__1__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => list_build_1 (&VOID_VALUE, None, Some (false)),
			},
		
		benchmark__list_clone__10__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_clone__100__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_clone__1000__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_clone__10000__mutable => {
				with_values
					list_clone (list, Some (false)),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (false))),
			},
		
		
		
		
		benchmark__list_reverse__0__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => list_empty (),
			},
		
		benchmark__list_reverse__1__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => list_build_1 (&VOID_VALUE, None, Some (true)),
			},
		
		benchmark__list_reverse__10__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_reverse__100__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_reverse__1000__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (true))),
			},
		
		benchmark__list_reverse__10000__immutable => {
				with_values
					list_reverse (list, Some (true)),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (true))),
			},
		
		
		benchmark__list_reverse__0__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => list_empty (),
			},
		
		benchmark__list_reverse__1__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => list_build_1 (&VOID_VALUE, None, Some (false)),
			},
		
		benchmark__list_reverse__10__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => try! (list_make (10, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_reverse__100__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => try! (list_make (100, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_reverse__1000__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => try! (list_make (1000, Some (&VOID_VALUE), Some (false))),
			},
		
		benchmark__list_reverse__10000__mutable => {
				with_values
					list_reverse (list, Some (false)),
					list => try! (list_make (10000, Some (&VOID_VALUE), Some (false))),
			},
		
		
		
		
	);

