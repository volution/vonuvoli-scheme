

#[ macro_use ]
extern crate rust_scheme;




def_tests! (values, r#"
	
	#t => #true;
	#f => #false;
	
	#void => #void;
	#undefined => #undefined;
	
	0 => 0;
	1 => 1;
	
"#);




def_tests! (control, r#"
	
	
	(if #t 1 2) => 1;
	(if #f 1 2) => 2;
	
	(if '() 1 2) => 1;
	(if #null 1 2) => 1;
	
	(if 0 1 2) => 1;
	(if 1 1 2) => 1;
	
	
	(begin) => #void;
	(begin 1) => 1;
	(begin 1 2) => 2;
	
	
	(define a 1) => 1;
	a => 1;
	(begin (define b 2) b) => 2;
	
	(define x (if x x x)) => #undefined // FIXME;
	
"#);

