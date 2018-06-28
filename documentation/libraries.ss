
(library
	
	
	
	
	(identifier vonuvoli:r7rs)
	
	(title "R7RS functionality with Vonuvoli-Scheme extensions")
	
	(description
		#<<<
					
					**FIXME!**
					
		>>>#)
	
	
	
	
	(categories
		
		
		
		
		(r7rs
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:base (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:case-lambda (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:char (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:complex (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:cxr (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:eval (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:file (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:inexact (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:lazy (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:load (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:process-context (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:r5rs (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:read (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:repl (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:time (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs:write (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(r7rs-x (parent r7rs)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs-x:types (parent r7rs-x)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(r7rs-x:types-disjoint (parent r7rs-x:types)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(vs
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vs:arithmetic (parent vs))
		(vs:associations (parent vs))
		(vs:bytes (parent vs))
		(vs:booleans (parent vs))
		(vs:conversions (parent vs))
		(vs:globals (parent vs))
		(vs:file-system (parent vs))
		(vs:characters (parent vs))
		(vs:comparisons (parent vs))
		(vs:compiler (parent vs))
		(vs:contexts (parent vs))
		(vs:continuations (parent vs))
		(vs:control (parent vs))
		
		(vs:equivalence (parent vs)
			(description
				#<<<
					
					A **predicate** is a procedure that always returns a boolean
					value (`#t` or `#f`).  An **equivalence predicate** is
					the computational analogue of a mathematical equivalence relation; it is
					symmetric, reflexive, and transitive.
					
					Of the equivalence predicates
					described in this section, `eq?` is the finest or most
					discriminating, `equal?` is the coarsest, and `eqv?` is
					slightly less discriminating than `eq?`.
					
				>>>#))
		
		(vs:errors (parent vs))
		(vs:evaluator (parent vs))
		(vs:functions (parent vs))
		(vs:lambda (parent vs))
		(vs:lists (parent vs))
		(vs:loops (parent vs))
		(vs:modules (parent vs))
		(vs:pairs (parent vs))
		(vs:parameters (parent vs))
		(vs:ports (parent vs))
		(vs:ports:input (parent vs:ports))
		(vs:ports:output (parent vs:ports))
		(vs:ports:open (parent vs:ports))
		(vs:ports:values (parent vs:ports))
		(vs:promises (parent vs))
		(vs:quotation (parent vs))
		(vs:records (parent vs))
		(vs:strings (parent vs))
		(vs:symbols (parent vs))
		(vs:syntaxes (parent vs))
		(vs:system (parent vs))
		(vs:types (parent vs))
		(vs:unimplemented (parent vs))
		(vs:unsupported (parent vs))
		(vs:values (parent vs))
		(vs:vectors (parent vs))
		
		
		
		
	)
	
	
	
	
	(definitions
		
		
		
		
		(define-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					((keyword identifier))
				(_ keyword @syntax-transformer))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(let-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(keyword identifier)
						(syntaxes pattern
							()
							((keyword @syntax-transformer) ...))
						(expression expression))
				(_ syntaxes)
				(_ syntaxes expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(letrec-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(keyword identifier)
						(syntaxes pattern
							()
							((keyword @syntax-transformer) ...))
						(expression expression))
				(_ syntaxes)
				(_ syntaxes expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(syntax-rules (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(syntax-error (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(message value string)
						(argument value any))
				(_ message)
				(_ message argument ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(_ (category r7rs:base vs:syntaxes) (type auxiliary-syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(... (category r7rs:base vs:syntaxes) (type auxiliary-syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(=> (category r7rs:base vs:syntaxes) (type auxiliary-syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(else (category r7rs:base vs:syntaxes) (type auxiliary-syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(quote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(quasiquote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(unquote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(unquote-splicing (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(lambda (category r7rs:base vs:lambda) (type syntax)
			(syntax-rules
					(
						(argument identifier)
						(argument-rest identifier)
						(arguments pattern
							()
							(argument ...)
							(argument ... . argument-rest)
							argument-rest)
						(expression expression))
				(_ arguments expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(case-lambda (category r7rs:case-lambda vs:lambda) (type syntax)
			(syntax-rules
					(
						(argument identifier)
						(argument-rest identifier)
						(arguments pattern
							()
							(argument ...)
							(argument ... . argument-rest)
							argument-rest)
						(expression expression))
				(_
					(arguments expression)
					...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(define (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(argument identifier)
						(argument-rest identifier)
						(expression expression))
				(_ variable expression)
				(_ (variable) expression ...)
				(_ (variable argument ...) expression ...)
				(_ (variable argument ... . argument-rest) expression ...)
				(_ (variable . argument-rest) expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(let (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(function identifier)
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
				(_ function bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(let* (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(letrec (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(letrec* (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(set! (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(expression expression))
				(_ variable expression))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(define-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(expression expression))
				(_ (variable ...) expression))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(let-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							((variable ...) initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(let*-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							((variable ...) initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(define-record-type (category r7rs:base vs:contexts vs:records) (type syntax)
			(syntax-rules
					(
						(type-identifier identifier)
						(constructor-identifier identifier)
						(predicate-identifier identifier)
						(field-identifier identifier)
						(field-accessor-identifier identifier)
						(field-mutator-identifier identifier)
						(constructor-descriptor pattern
							constructor-identifier
							(constructor-identifier field-identifier ...))
						(field-descriptor pattern
							(field-identifier field-accessor-identifier)
							(field-identifier field-accessor-identifier field-mutator-identifier)))
				(_ type-identifier constructor-descriptor predicate-identifier field-descriptor ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(begin (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(and (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(or (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(if (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression)
						(else-expression expression))
				(_ condition then-expression)
				(_ condition then-expression else-expression))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(unless (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression))
				(_ condition then-expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(when (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression))
				(_ condition then-expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(cond (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(else literal)
						(condition expression)
						(then-expression expression)
						(clause pattern
							(condition)
							(condition then-expression ...)
							(else)
							(else then-expression ...)))
				(_)
				(_ clause ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(case (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(else literal)
						(value expression)
						(variant value any)
						(then-expression expression)
						(clause pattern
							((variant ...))
							((variant ...) then-expression ...)
							(else)
							(else then-expression ...)))
				(_ value)
				(_ value clause ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(do (category r7rs:base vs:control vs:loops) (type syntax)
			(syntax-rules
					(
						(binding-variable identifier)
						(binding-initializer expression)
						(binding-updater expression)
						(binding pattern
							(binding-variable binding-initializer)
							(binding-variable binding-initializer binding-updater))
						(bindings pattern
							()
							(binding ...))
						(exit-test expression)
						(exit-expression expression)
						(exit-clause pattern
							(exit-test)
							(exit-test exit-expression))
						(iteration-expression expression))
				(_ bindings exit-clause)
				(_ bindings exit-clause iteration-expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(eq? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean))
			(description
				#<<<
					
					````
					(eq? obj_1 obj_2)
					````
					
					
					The `eq?` procedure is similar to `eqv?` except that in some cases it is
					capable of discerning distinctions finer than those detectable by
					`eqv?`.  It must always return `#f` when `eqv?` also
					would, but may return `#f` in some cases where `eqv?` would return `#t`.
					
					On symbols, booleans, the empty list, pairs, and records,
					and also on non-empty
					strings, vectors, and bytevectors, `eq?` and `eqv?` are guaranteed to have the same
					behavior.  On procedures, `eq?` must return true if the arguments' location
					tags are equal.  On numbers and characters, `eq?`'s behavior is
					implementation-dependent, but it will always return either true or
					false.  On empty strings, empty vectors, and empty bytevectors, `eq?` may also behave
					differently from `eqv?`.
					
					
					````
					(eq? 'a 'a)                     ===>  #t
					(eq? '(a) '(a))                 ===>  #unspecified
					(eq? (list 'a) (list 'a))       ===>  #f
					(eq? "a" "a")                   ===>  #unspecified
					(eq? "" "")                     ===>  #unspecified
					(eq? '() '())                   ===>  #t
					(eq? 2 2)                       ===>  #unspecified
					(eq? #\A #\A)                   ===>  #unspecified
					(eq? car car)                   ===>  #t
					(let ((n (+ 2 3)))
					  (eq? n n))                    ===>  #unspecified
					(let ((x '(a)))
					  (eq? x x))                    ===>  #t
					(let ((x '#()))
					  (eq? x x))                    ===>  #t
					(let ((p (lambda (x) x)))
					  (eq? p p))                    ===>  #t
					````
					
					
					**Rationale**:  It will usually be possible to implement `eq?` much
					more efficiently than `eqv?`, for example, as a simple pointer
					comparison instead of as some more complicated operation.  One reason is
					that it is not always possible to compute `eqv?` of two numbers in
					constant time, whereas `eq?` implemented as pointer comparison will
					always finish in constant time.
					
				>>>#))
		
		(eqv? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean))
			(description
				#<<<
					
					````
					(eqv? obj_1 obj_2)
					````
					
					
					The `eqv?` procedure defines a useful equivalence relation on objects.
					Briefly, it returns `#t` if `obj_1` and `obj_2` are
					normally regarded as the same object.  This relation is left slightly
					open to interpretation, but the following partial specification of
					`eqv?` holds for all implementations of Scheme.
					
					
					The `eqv?` procedure returns `#t` if:
					
					  * `obj_1` and `obj_2` are both `#t` or both `#f`.
					
					  * `obj_1` and `obj_2` are both symbols and are the same
					symbol according to the `symbol=?` procedure.
					
					  * `obj_1` and `obj_2` are both exact numbers and
					are numerically equal (in the sense of `=`).
					
					  * `obj_1` and `obj_2` are both inexact numbers such that
					they are numerically equal (in the sense of `=`)
					and they yield the same results (in the sense of `eqv?`)
					when passed as arguments to any other procedure
					that can be defined as a finite composition of Scheme's standard
					arithmetic procedures, provided it does not result in a `NaN` value.
					
					  * `obj_1` and `obj_2` are both characters and are the same
					character according to the `char=?` procedure.
					
					  * `obj_1` and `obj_2` are both the empty list.
					
					  * `obj_1` and `obj_2` are pairs, vectors, bytevectors, records,
					or strings that denote the same location in the store.
					
					  * `obj_1` and `obj_2` are procedures whose location tags are
					equal.
					
					
					The `eqv?` procedure returns `#f` if:
					
					  * `obj_1` and `obj_2` are of different types.
					
					  * one of `obj_1` and `obj_2` is `#t` but the other is
					`#f`.
					
					  * `obj_1` and `obj_2` are symbols but are not the same
					symbol according to the `symbol=?` procedure.
					
					  * one of `obj_1` and `obj_2` is an exact number but the other
					is an inexact number.
					
					  * `obj_1` and `obj_2` are both exact numbers and
					are numerically unequal (in the sense of `=`).
					
					  * `obj_1` and `obj_2` are both inexact numbers such that either
					they are numerically unequal (in the sense of `=`),
					or they do not yield the same results (in the sense of `eqv?`)
					when passed as arguments to any other procedure
					that can be defined as a finite composition of Scheme's standard
					arithmetic procedures, provided it does not result in a `NaN` value.
					As an exception, the behavior of `eqv?` is unspecified
					when both `obj_1` and `obj_2` are `NaN`.
					
					  * `obj_1` and `obj_2` are characters for which the `char=?`
					procedure returns `#f`.
					
					  * one of `obj_1` and `obj_2` is the empty list but the other
					is not.
					
					  * `obj_1` and `obj_2` are pairs, vectors, bytevectors, records,
					or strings that denote distinct locations.
					
					  * `obj_1` and `obj_2` are procedures that would behave differently
					(return different values or have different side effects) for some arguments.
					
					
					````
					(eqv? 'a 'a)                     ===>  #t
					(eqv? 'a 'b)                     ===>  #f
					(eqv? 2 2)                       ===>  #t
					(eqv? 2 2.0)                     ===>  #f
					(eqv? '() '())                   ===>  #t
					(eqv? 100000000 100000000)       ===>  #t
					(eqv? 0.0 +nan.0)                ===>  #f
					(eqv? (cons 1 2) (cons 1 2))     ===>  #f
					(eqv? (lambda () 1)
					      (lambda () 2))             ===>  #f
					(let ((p (lambda (x) x)))
					  (eqv? p p))                    ===>  #t
					(eqv? #f 'nil)                   ===>  #f
					````
					
					
					The following examples illustrate cases in which the above rules do
					not fully specify the behavior of `eqv?`.  All that can be said
					about such cases is that the value returned by `eqv?` must be a
					boolean.
					
					````
					(eqv? "" "")             ===>  #unspecified
					(eqv? '#() '#())         ===>  #unspecified
					(eqv? (lambda (x) x)
					      (lambda (x) x))    ===>  #unspecified
					(eqv? (lambda (x) x)
					      (lambda (y) y))    ===>  #unspecified
					(eqv? 1.0e0 1.0f0)       ===>  #unspecified
					(eqv? +nan.0 +nan.0)     ===>  #unspecified
					````
					
					Note that `(eqv? 0.0 -0.0)` will return `#f` if negative zero
					is distinguished, and `#t` if negative zero is not distinguished.
					
					
					Since it is an error to modify constant objects (those returned by
					literal expressions), implementations may
					share structure between constants where appropriate.  Thus
					the value of `eqv?` on constants is sometimes
					implementation-dependent.
					
					````
					(eqv? '(a) '(a))                 ===>  #unspecified
					(eqv? "a" "a")                   ===>  #unspecified
					(eqv? '(b) (cdr '(a b)))         ===>  #unspecified
					(let ((x '(a)))
					  (eqv? x x))                    ===>  #t
					````
					
					
					The above definition of `eqv?` allows implementations latitude in
					their treatment of procedures and literals:  implementations may
					either detect or fail to detect that two procedures or two literals
					are equivalent to each other, and can decide whether or not to
					merge representations of equivalent objects by using the same pointer or
					bit pattern to represent both.
					
					**Note**:  If inexact numbers are represented as IEEE binary floating-point numbers,
					then an implementation of `eqv?` that simply compares equal-sized
					inexact numbers for bitwise equality is correct by the above definition.
					
				>>>#))
		
		(equal? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean))
			(description
				#<<<
					
					````
					(equal? obj_1 obj_2)
					````
					
					
					The `equal?` procedure, when applied to pairs, vectors, strings and
					bytevectors, recursively compares them, returning `#t` when the
					unfoldings of its arguments into (possibly infinite) trees are equal
					(in the sense of `equal?`)
					as ordered trees, and `#f` otherwise.  It returns the same as
					`eqv?` when applied to booleans, symbols, numbers, characters,
					ports, procedures, and the empty list.  If two objects are `eqv?`,
					they must be `equal?` as well.  In all other cases, `equal?`
					may return either `#t` or `#f`.
					
					Note that records are `equal?` if their record types are the same
					and their correspondingly named fields are `equal?`.
					
					Even if its arguments are
					circular data structures, `equal?` must always terminate.
					
					
					````
					(equal? 'a 'a)                  ===>  #t
					(equal? '(a) '(a))              ===>  #t
					(equal? '(a (b) c)
					        '(a (b) c))             ===>  #t
					(equal? "abc" "abc")            ===>  #t
					(equal? 2 2)                    ===>  #t
					(equal? (make-vector 5 'a)
					        (make-vector 5 'a))     ===>  #t
					(equal? '#1=(a b . #1#)
					        '#2=(a b a b . #2#))    ===>  #t
					(equal? (lambda (x) x)
					        (lambda (y) y))         ===>  #unspecified
					````
					
					
					**Note**:  A rule of thumb is that objects are generally `equal?` if they print
					the same.
					
				>>>#))
		
		
		
		
		(boolean? (category r7rs:base vs:booleans vs:types) (type type-predicate)
			(description
				#<<<
					
					````
					(boolean? obj)
					````
					
					
					The `boolean?` predicate returns `#t` if `obj` is either `#t` or
					`#f` and returns `#f` otherwise.
					
					````
					(boolean? #f)         ===>  #t
					(boolean? 0)          ===>  #f
					(boolean? '())        ===>  #f
					````
					
				>>>#))
		
		(boolean=? (category r7rs:base vs:booleans vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					````
					(boolean=? boolean_1 boolean_2 boolean_3 ...)
					````
					
					
					Returns `#t` if all the arguments are booleans and all
					are `#t` or all are `#f`.
					
				>>>#))
		
		
		(not (category r7rs:base) (type predicate)
			(signature (any -> boolean))
			(description
				#<<<
					
					````
					(not obj)
					````
					
					
					The `not` procedure returns `#t` if `obj` is false, and returns
					`#f` otherwise.
					
					````
					(not #t)         ===>  #f
					(not 3)          ===>  #f
					(not (list 3))   ===>  #f
					(not #f)         ===>  #t
					(not '())        ===>  #f
					(not (list))     ===>  #f
					(not 'nil)       ===>  #f
					````
					
				>>>#))
		
		
		
		
		(symbol? (category r7rs:base vs:symbols vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(symbol=? (category r7rs:base vs:symbols vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(number? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					````
					(number? obj)
					(complex? obj)
					(real? obj)
					(rational? obj)
					(integer? obj)
					````
					
					
					These numerical type predicates can be applied to any kind of
					argument, including non-numbers.  They return `#t` if the object is
					of the named type, and otherwise they return `#f`.
					In general, if a type predicate is true of a number then all higher
					type predicates are also true of that number.  Consequently, if a type
					predicate is false of a number, then all lower type predicates are
					also false of that number.
					
					If `z` is a complex number, then `(real? z)` is true if
					and only if `(zero? (imag-part z))` is true.
					If `x` is an inexact real number, then
					`(integer? x)` is true if and only if `(= x (round x))`.
					
					The numbers `+inf.0`, `-inf.0`, and `+nan.0` are real but
					not rational.
					
					
					````
					(complex? 3+4i)         ===>  #t
					(complex? 3)            ===>  #t
					(real? 3)               ===>  #t
					(real? -2.5+0i)         ===>  #t
					(real? -2.5+0.0i)       ===>  #f
					(real? #e1e10)          ===>  #t
					(real? +inf.0)          ===>  #t
					(real? +nan.0)          ===>  #t
					(rational? -inf.0)      ===>  #f
					(rational? 3.5)         ===>  #t
					(rational? 6/10)        ===>  #t
					(rational? 6/3)         ===>  #t
					(integer? 3+0i)         ===>  #t
					(integer? 3.0)          ===>  #t
					(integer? 8/4)          ===>  #t
					````
					
					
					**Note**: The behavior of these type predicates on __inexact__ numbers
					is unreliable, since any inaccuracy might affect the result.
					
					**Note**:  In many implementations the `complex?` procedure will be the same as
					`number?`, but unusual implementations may represent
					some irrational numbers exactly or may extend the number system to
					support some kind of non-complex numbers.
					
				>>>#))
		
		(integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					Please refer to [`number?`]().
					
				>>>#))
		
		(real? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					Please refer to [`number?`]().
					
				>>>#))
		
		(rational? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					Please refer to [`number?`]().
					
				>>>#))
		
		(complex? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					Please refer to [`number?`]().
					
				>>>#))
		
		
		(exact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					````
					(exact? z)
					(inexact? z)
					````
					
					
					These numerical predicates provide tests for the exactness of a
					quantity.  For any Scheme number, precisely one of these predicates
					is true.
					
					````
					(exact? 3.0)           ===>  #f
					(exact? #e3.0)         ===>  #t
					(inexact? 3.)          ===>  #t
					````
					
				>>>#))
		
		(inexact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					Please refer to [`exact?`]().
					
				>>>#))
		
		(exact-integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					````
					(exact-integer? z)
					````
					
					
					Returns `#t` if `z` is both __exact__ and an __integer__;
					otherwise returns `#f`.
					
					````
					(exact-integer? 32)    ===>  #t
					(exact-integer? 32.0)  ===>  #f
					(exact-integer? 32/5)  ===>  #f
					````
					
				>>>#))
		
		
		(zero? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					````
					(zero? z)
					(positive? x)
					(negative? x)
					(odd? n)
					(even? n)
					````
					
					
					These numerical predicates test a number for a particular property,
					returning `#t` or `#f`.  See note above.
					
				>>>#))
		
		(positive? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					Please refer to [`zero?`]().
					
				>>>#))
		
		(negative? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					Please refer to [`zero?`]().
					
				>>>#))
		
		(odd? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					Please refer to [`zero?`]().
					
				>>>#))
		
		(even? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					Please refer to [`zero?`]().
					
				>>>#))
		
		
		(= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator=)
			(description
				#<<<
					
					````
					(= z_1 z_2 z_3 ...)
					(< x_1 x_2 x_3 ...)
					(> x_1 x_2 x_3 ...)
					(<= x_1 x_2 x_3 ...)
					(>= x_1 x_2 x_3 ...)
					````
					
					
					These procedures return `#t` if their arguments are (respectively):
					equal, monotonically increasing, monotonically decreasing,
					monotonically non-decreasing, or monotonically non-increasing,
					and `#f` otherwise.
					If any of the arguments are `+nan.0`, all the predicates return `#f`.
					They do not distinguish between inexact zero and inexact negative zero.
					
					These predicates are required to be transitive.
					
					**Note**:  The implementation approach
					of converting all arguments to inexact numbers
					if any argument is inexact is not transitive.  For example, let
					`big` be `(expt 2 1000)`, and assume that `big` is exact and that
					inexact numbers are represented by 64-bit IEEE binary floating point numbers.
					Then `(= (- big 1) (inexact big))` and
					`(= (inexact big) (+ big 1))` would both be true with this approach,
					because of the limitations of IEEE
					representations of large integers, whereas `(= (- big 1) (+ big 1))`
					is false.  Converting inexact values to exact numbers that are the same (in the sense of `=`) to them will avoid
					this problem, though special care must be taken with infinities.
					
					
					**Note**:  While it is not an error to compare __inexact__ numbers using these
					predicates, the results are unreliable because a small inaccuracy
					can affect the result; this is especially true of `=` and `zero?`.
					When in doubt, consult a numerical analyst.
					
				>>>#))
		
		(< (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<)
			(description
				#<<<
					
					Please refer to [`=`]().
					
				>>>#))
		
		(> (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>)
			(description
				#<<<
					
					Please refer to [`=`]().
					
				>>>#))
		
		(<= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<=)
			(description
				#<<<
					
					Please refer to [`=`]().
					
				>>>#))
		
		(>= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>=)
			(description
				#<<<
					
					Please refer to [`=`]().
					
				>>>#))
		
		
		(+ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(+ z_1 ...)
					(* z_1 ...)
					````
					
					
					These procedures return the sum or product of their arguments.
					
					````
					(+ 3 4)                 ===>  7
					(+ 3)                   ===>  3
					(+)                     ===>  0
					(* 4)                   ===>  4
					(*)                     ===>  1
					````
					
				>>>#))
		
		(- (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(- z)
					(- z_1 z_2 ...)
					(/ z)
					(/ z_1 z_2 ...)
					````
					
					
					With two or more arguments, these procedures return the difference or
					quotient of their arguments, associating to the left.  With one argument,
					however, they return the additive or multiplicative inverse of their argument.
					
					It is an error if any argument of `/` other than the first is an exact zero.
					If the first argument is an exact zero, an implementation may return an
					exact zero unless one of the other arguments is a NaN.
					
					````
					(- 3 4)                 ===>  -1
					(- 3 4 5)               ===>  -6
					(- 3)                   ===>  -3
					(/ 3 4 5)               ===>   3/20
					(/ 3)                   ===>   1/3
					````
					
				>>>#))
		
		(* (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`+`]().
					
				>>>#))
		
		(/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`-`]().
					
				>>>#))
		
		
		(abs (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(abs x)
					````
					
					
					The `abs` procedure returns the absolute value of its argument.
					````
					(abs -7)                ===>  7
					````
					
				>>>#))
		
		
		(floor/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(floor/ n_1 n_2)
					(floor-quotient n_1 n_2)
					(floor-remainder n_1 n_2)
					(truncate/ n_1 n_2)
					(truncate-quotient n_1 n_2)
					(truncate-remainder n_1 n_2)
					````
					
					
					These procedures implement
					number-theoretic (integer) division.  It is an error if `n_2` is zero.
					The procedures ending in `/` return two integers; the other
					procedures return an integer.  All the procedures compute a
					quotient `n_q` and remainder `n_r` such that
					`n_1 = n_2 n_q + n_r`.  For each of the
					division operators, there are three procedures defined as follows:
					
					````
					(<operator>/ n_1 n_2)             ===>  n_q n_r
					(<operator>-quotient n_1 n_2)     ===>  n_q
					(<operator>-remainder n_1 n_2)    ===>  n_r
					````
					
					The remainder `n_r` is determined by the choice of integer
					`n_q`: `n_r = n_1 - n_2 n_q`.  Each set of
					operators uses a different choice of `n_q`:
					
					 * `floor` -- `n_q = floor(n_1 / n_2)`;
					 * `truncate` -- `n_q = truncate(n_1 / n_2)`;
					
					For any of the operators, and for integers `n_1` and `n_2`
					with `n_2` not equal to 0,
					````
					     (= n_1 (+ (* n_2 (<operator>-quotient n_1 n_2))
					           (<operator>-remainder n_1 n_2)))
					                                 ===>  #t
					````
					provided all numbers involved in that computation are exact.
					
					Examples:
					
					````
					(floor/ 5 2)         ===>   2    1
					(floor/ -5 2)        ===>  -3    1
					(floor/ 5 -2)        ===>  -3   -1
					(floor/ -5 -2)       ===>   2   -1
					(truncate/ 5 2)      ===>   2    1
					(truncate/ -5 2)     ===>  -2   -1
					(truncate/ 5 -2)     ===>  -2    1
					(truncate/ -5 -2)    ===>   2   -1
					(truncate/ -5.0 -2)  ===>   2.0 -1.0
					````
					
					
					
					
					````
					(quotient n_1 n_2)
					(remainder n_1 n_2)
					(modulo n_1 n_2)
					````
					
					
					The `quotient` and `remainder` procedures are equivalent to
					`truncate-quotient` and `truncate-remainder`, respectively, and
					`modulo` is equivalent to `floor-remainder`.
					
					**Note**:  These procedures are provided for backward compatibility with earlier
					versions of this report.
					
				>>>#))
		
		(floor-quotient (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`floor/`]().
					
				>>>#))
		
		(floor-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias modulo)
			(description
				#<<<
					
					Please refer to [`floor/`]().
					
				>>>#))
		
		(truncate/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`floor/`]().
					
				>>>#))
		
		(truncate-quotient (category r7rs:base vs:arithmetic) (type procedure) (alias quotient)
			(description
				#<<<
					
					Please refer to [`floor/`]().
					
				>>>#))
		
		(truncate-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias remainder)
			(description
				#<<<
					
					Please refer to [`floor/`]().
					
				>>>#))
		
		
		(floor (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(floor x)
					(ceiling x)
					(truncate x)
					(round x)
					````
					
					
					These procedures return integers.
					
					The `floor` procedure returns the largest integer not larger than `x`.
					The `ceiling` procedure returns the smallest integer not smaller than `x`,
					`truncate` returns the integer closest to `x` whose absolute
					value is not larger than the absolute value of `x`, and `round` returns the
					closest integer to `x`, rounding to even when `x` is halfway between two
					integers.
					
					**Rationale**:  The `round` procedure rounds to even for consistency with the default rounding
					mode specified by the IEEE 754 IEEE floating-point standard.
					
					**Note**:  If the argument to one of these procedures is inexact, then the result
					will also be inexact.  If an exact value is needed, the
					result can be passed to the `exact` procedure.
					If the argument is infinite or a NaN, then it is returned.
					
					
					````
					(floor -4.3)          ===>  -5.0
					(ceiling -4.3)        ===>  -4.0
					(truncate -4.3)       ===>  -4.0
					(round -4.3)          ===>  -4.0
					
					(floor 3.5)           ===>   3.0
					(ceiling 3.5)         ===>   4.0
					(truncate 3.5)        ===>   3.0
					(round 3.5)           ===>   4.0  ; inexact
					
					(round 7/2)           ===>   4    ; exact
					(round 7)             ===>   7
					````
					
				>>>#))
		
		(ceiling (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`floor`]().
					
				>>>#))
		
		(truncate (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`floor`]().
					
				>>>#))
		
		(round (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`floor`]().
					
				>>>#))
		
		
		(min (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(max x_1 x_2 ...)
					(min x_1 x_2 ...)
					````
					
					
					These procedures return the maximum or minimum of their arguments.
					
					````
					(max 3 4)              ===>  4    ; exact
					(max 3.9 4)            ===>  4.0  ; inexact
					````
					
					**Note**:  If any argument is inexact, then the result will also be inexact (unless
					the procedure can prove that the inaccuracy is not large enough to affect the
					result, which is possible only in unusual implementations).  If `min` or
					`max` is used to compare numbers of mixed exactness, and the numerical
					value of the result cannot be represented as an inexact number without loss of
					accuracy, then the procedure may report a violation of an implementation
					restriction.
					
					
				>>>#))
		
		(max (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`min`]().
					
				>>>#))
		
		
		(gcd (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(gcd n_1 ...)
					(lcm n_1 ...)
					````
					
					
					These procedures return the greatest common divisor or least common
					multiple of their arguments.  The result is always non-negative.
					
					````
					(gcd 32 -36)            ===>  4
					(gcd)                   ===>  0
					(lcm 32 -36)            ===>  288
					(lcm 32.0 -36)          ===>  288.0  ; inexact
					(lcm)                   ===>  1
					````
					
				>>>#))
		
		(lcm (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					Please refer to [`gcd`]().
					
				>>>#))
		
		
		(expt (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(expt z_1 z_2)
					````
					
					
					Returns `z_1` raised to the power `z_2`.  For nonzero `z_1`, this is
					`z_1^z_2 = e^(z_2 log z_1)`
					The value of `0^z` is `1` if `(zero? z)`, `0` if `(real-part z)`
					is positive, and an error otherwise.  Similarly for `0.0^z`,
					with inexact results.
					
				>>>#))
		
		(square (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(square z)
					````
					
					
					Returns the square of `z`.
					This is equivalent to `(* z z)`.
					
					````
					(square 42)       ===>  1764
					(square 2.0)      ===>  4.0
					````
					
				>>>#))
		
		(exact-integer-sqrt (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(exact-integer-sqrt k)
					````
					
					
					Returns two non-negative exact integers `s` and `r` where
					`k = s^2 + r` and `k < (s+1)^2`.
					
					````
					(exact-integer-sqrt 4)  ===>  2 0
					(exact-integer-sqrt 5)  ===>  2 1
					````
					
				>>>#))
		
		
		(rationalize (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					````
					(rationalize x y)
					````
					
					
					The `rationalize` procedure returns the __simplest__ rational number
					differing from `x` by no more than `y`.  A rational number `r_1` is
					__simpler__ (simplest rational) than another rational number
					`r_2` if `r_1 = p_1/q_1` and `r_2 = p_2/q_2` (in lowest terms) and
					`|p_1| <= |p_2|` and `|q_1| <= |q_2|`.  Thus `3/5` is simpler than `4/7`.
					Although not all rationals are comparable in this ordering (consider `2/7`
					and `3/5`), any interval contains a rational number that is simpler than
					every other rational number in that interval (the simpler `2/5` lies
					between `2/7` and `3/5`).  Note that `0 = 0/1` is the simplest rational of
					all.
					
					````
					(rationalize
					  (exact .3) 1/10)           ===>  1/3    ; exact
					(rationalize .3 1/10)        ===>  #i1/3  ; inexact
					````
					
				>>>#))
		
		(numerator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					````
					(numerator q)
					(denominator q)
					````
					
					
					These procedures return the numerator or denominator of their
					argument; the result is computed as if the argument was represented as
					a fraction in lowest terms.  The denominator is always positive.  The
					denominator of `0` is defined to be `1`.
					
					````
					(numerator (/ 6 4))    ===>  3
					(denominator (/ 6 4))  ===>  2
					(denominator
					  (inexact (/ 6 4)))   ===>  2.0
					````
					
				>>>#))
		
		(denominator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					Please refer to [`numerator`]().
					
				>>>#))
		
		
		(inexact (category r7rs:complex vs:arithmetic) (type converter)
			(description
				#<<<
					
					````
					(inexact z)
					(exact z)
					````
					
					
					The procedure `inexact` returns an __inexact__ representation of `z`.
					The value returned is the
					__inexact__ number that is numerically closest to the argument.
					For inexact arguments, the result is the same as the argument. For exact
					complex numbers, the result is a complex number whose real and imaginary
					parts are the result of applying `inexact` to the real
					and imaginary parts of the argument, respectively.
					If an __exact__ argument has no reasonably close __inexact__ equivalent
					(in the sense of `=`),
					then a violation of an implementation restriction may be reported.
					
					The procedure `exact` returns an __exact__ representation of
					`z`.  The value returned is the __exact__ number that is numerically
					closest to the argument.
					For exact arguments, the result is the same as the argument. For inexact
					non-integral real arguments, the implementation may return a rational
					approximation, or may report an implementation violation. For inexact
					complex arguments, the result is a complex number whose real and
					imaginary parts are the result of applying `exact` to the
					real and imaginary parts of the argument, respectively.
					If an __inexact__ argument has no reasonably close __exact__ equivalent,
					(in the sense of `=`),
					then a violation of an implementation restriction may be reported.
					
					These procedures implement the natural one-to-one correspondence between
					__exact__ and __inexact__ integers throughout an
					implementation-dependent range.  See section on restrictions.
					
					**Note**:  These procedures were known in __R5RS__ as `exact->inexact` and
					`inexact->exact`, respectively, but they have always accepted
					arguments of any exactness.  The new names are clearer and shorter,
					as well as being compatible with __R6RS__.
					
				>>>#))
		
		(exact (category r7rs:complex vs:arithmetic) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(make-rectangular (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					````
					(make-rectangular x_1 x_2)
					(make-polar x_3 x_4)
					(real-part z)
					(imag-part z)
					(magnitude z)
					(angle z)
					````
					
					
					Let `x_1`, `x_2`, `x_3`, and `x_4` be
					real numbers and `z` be a complex number such that
					`z = x_1 + x_2*i = x_3 * e^(x_4*i)`
					Then all of
					````
					(make-rectangular x_1 x_2)     ===>  z
					(make-polar x_3 x_4)           ===>  z
					(real-part z)                  ===>  x_1
					(imag-part z)                  ===>  x_2
					(magnitude z)                  ===>  | x_3 |
					(angle z)                      ===>  x_angle
					````
					are true, where `-pi <= x_angle <= pi` with `x_angle = x_4 + 2 pi n`
					for some integer `n`.
					
					The `make-polar` procedure may return an inexact complex number even if its
					arguments are exact.
					The `real-part` and `imag-part` procedures may return exact real
					numbers when applied to an inexact complex number if the corresponding
					argument passed to `make-rectangular` was exact.
					
					
					**Rationale**:  The `magnitude` procedure is the same as `abs` for a real argument,
					but `abs` is in the base library, whereas
					`magnitude` is in the optional complex library.
					
				>>>#))
		
		(real-part (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(imag-part (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-polar (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(magnitude (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(angle (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(sqrt (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(sqrt z)
					````
					
					
					Returns the principal square root of `z`.  The result will have
					either a positive real part, or a zero real part and a non-negative imaginary
					part.
					
					````
					(sqrt 9)   ===>   3
					(sqrt -1)  ===>  +i
					````
					
				>>>#))
		
		(exp (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					````
					(exp z)
					(log z)
					(log z_1 z_2)
					(sin z)
					(cos z)
					(tan z)
					(asin z)
					(acos z)
					(atan z)
					(atan y x)
					````
					
					
					These procedures
					compute the usual transcendental functions.  The `log` procedure
					computes the natural logarithm of `z` (not the base ten logarithm)
					if a single argument is given, or the base-`z_2` logarithm of `z_1`
					if two arguments are given.
					The `asin`, `acos`, and `atan` procedures compute arcsine (`sin^-1`),
					arc-cosine (`cos^-1`), and arctangent (`tan^-1`), respectively.
					The two-argument variant of `atan` computes
					`(angle (make-rectangular x y))`
					(see below), even in implementations
					that don't support complex numbers.
					
					In general, the mathematical functions log, arcsine, arc-cosine, and
					arctangent are multiply defined.
					The value of `log z` is defined to be the one whose imaginary part
					lies in the range from `-pi` (inclusive if `-0.0` is distinguished,
					exclusive otherwise) to `pi` (inclusive).
					The value of `log 0` is mathematically undefined.
					With `log` defined this way, the values of `sin^-1 z`, `cos^-1 z`,
					and `tan^-1 z` are according to the following formulae:
					`sin^-1 z = -i log(i z + sqrt(1 - z^2))`
					`cos^-1 z = pi / 2 - sin^-1 z`
					`tan^-1 z = (log(1 + i z) - log(1 - i z)) / (2 i)`
					
					However, `(log 0.0)` returns `-inf.0`
					(and `(log -0.0)` returns `-inf.0+pi*i`) if the
					implementation supports infinities (and `-0.0`).
					
					The range of `(atan y x)` is as in the
					following table. The asterisk (`*`) indicates that the entry applies to
					implementations that distinguish minus zero.
					
					````
					|     | `y` condition | `x` condition | range of result `r` |
					|     |   `y =  0.0`  |   `x >  0.0`  |  ` 0.0`             |
					| `*` |   `y = +0.0`  |   `x >  0.0`  |  `+0.0`             |
					| `*` |   `y = -0.0`  |   `x >  0.0`  |  `-0.0`             |
					|     |   `y >  0.0`  |   `x >  0.0`  |  ` 0.0 < r < pi/2`  |
					|     |   `y >  0.0`  |   `x =  0.0`  |  ` pi/2`            |
					|     |   `y >  0.0`  |   `x <  0.0`  |  ` pi/2 < r < pi`   |
					|     |   `y =  0.0`  |   `x <  0`    |  ` pi`              |
					| `*` |   `y = +0.0`  |   `x <  0.0`  |  ` pi`              |
					| `*` |   `y = -0.0`  |   `x <  0.0`  |  `-pi`              |
					|     |   `y <  0.0`  |   `x <  0.0`  |  `-pi< r < -pi/2`   |
					|     |   `y <  0.0`  |   `x =  0.0`  |  `-pi/2`            |
					|     |   `y <  0.0`  |   `x >  0.0`  |  `-pi/2 < r < 0.0`  |
					|     |   `y =  0.0`  |   `x =  0.0`  |  undefined          |
					| `*` |   `y = +0.0`  |   `x = +0.0`  |  `+0.0`             |
					| `*` |   `y = -0.0`  |   `x = +0.0`  |  `-0.0`             |
					| `*` |   `y = +0.0`  |   `x = -0.0`  |  ` pi`              |
					| `*` |   `y = -0.0`  |   `x = -0.0`  |  `-pi`              |
					| `*` |   `y = +0.0`  |   `x =  0`    |  ` pi/2`            |
					| `*` |   `y = -0.0`  |   `x =  0`    |  `-pi/2`            |
					````
					
					The above specification follows __Common Lisp: The Language, second edition__, which in turn
					cites __Principal values and branch cuts in complex APL__; refer to these sources for more detailed
					discussion of branch cuts, boundary conditions, and implementation of
					these functions.  When it is possible, these procedures produce a real
					result from a real argument.
					
				>>>#))
		
		(log (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(sin (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cos (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(tan (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(asin (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(acos (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(atan (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(finite? (category r7rs:inexact vs:arithmetic) (type predicate)
			(description
				#<<<
					
					````
					(finite? z)
					````
					
					
					The `finite?` procedure returns `#t` on all real numbers except
					`+inf.0`, `-inf.0`, and `+nan.0`, and on complex
					numbers if their real and imaginary parts are both finite.
					Otherwise it returns `#f`.
					
					````
					(finite? 3)            ===>  #t
					(finite? +inf.0)       ===>  #f
					(finite? 3.0+inf.0i)   ===>  #f
					````
					
				>>>#))
		
		(infinite? (category r7rs:inexact vs:arithmetic) (type predicate)
			(description
				#<<<
					
					````
					(infinite? z)
					````
					
					
					The `infinite?` procedure returns `#t` on the real numbers
					`+inf.0` and `-inf.0`, and on complex
					numbers if their real or imaginary parts or both are infinite.
					Otherwise it returns `#f`.
					
					````
					(infinite? 3)            ===>  #f
					(infinite? +inf.0)       ===>  #t
					(infinite? +nan.0)       ===>  #f
					(infinite? 3.0+inf.0i)   ===>  #t
					````
					
				>>>#))
		
		(nan? (category r7rs:inexact vs:arithmetic) (type predicate)
			(description
				#<<<
					
					````
					(nan? z)
					````
					
					
					The `nan?` procedure returns `#t` on `+nan.0`, and on complex
					numbers if their real or imaginary parts or both are `+nan.0`.
					Otherwise it returns `#f`.
					
					````
					(nan? +nan.0)          ===>  #t
					(nan? 32)              ===>  #f
					(nan? +nan.0+5.0i)     ===>  #t
					(nan? 1+2i)            ===>  #f
					````
					
				>>>#))
		
		
		
		
		(pair? (category r7rs:base vs:pairs vs:lists vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cons (category r7rs:base vs:pairs vs:lists) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(car (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdr (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(set-car! (category r7rs:base vs:pairs vs:lists) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(set-cdr! (category r7rs:base vs:pairs vs:lists) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(caar (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cadr (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(cdar (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cddr (category r7rs:base vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(caaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cadar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(cdaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cddar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(caaaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caaadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caadar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caaddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cadaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cadadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(caddar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cadddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(cdaaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdaadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdadar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdaddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cddaar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cddadr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cdddar (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(cddddr (category r7rs:cxr vs:pairs vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(null? (category r7rs:base vs:lists vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(list? (category r7rs:base vs:lists vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(list (category r7rs:base vs:lists) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-list (category r7rs:base vs:lists) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(length (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(append (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(list-copy (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(reverse (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(list-ref (category r7rs:base vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(list-tail (category r7rs:base vs:lists) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(list-set! (category r7rs:base vs:lists) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(map (category r7rs:base vs:lists vs:functions vs:conversions vs:loops) (type map)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(for-each (category r7rs:base vs:lists vs:functions vs:loops) (type for-each)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(member (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(memq (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(memv (category r7rs:base vs:lists) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(assoc (category r7rs:base vs:lists vs:associations) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(assqc (category r7rs:base vs:lists vs:associations) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(assvc (category r7rs:base vs:lists vs:associations) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(vector? (category r7rs:base vs:vectors vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector (category r7rs:base vs:vectors) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-vector (category r7rs:base vs:vectors) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector-length (category r7rs:base vs:vectors) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector-append (category r7rs:base vs:vectors) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector-copy (category r7rs:base vs:vectors) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector-copy! (category r7rs:base vs:vectors) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector-fill! (category r7rs:base vs:vectors) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector-ref (category r7rs:base vs:vectors) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector-set! (category r7rs:base vs:vectors) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector->list (category r7rs:base vs:vectors vs:lists vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(list->vector (category r7rs:base vs:vectors vs:lists vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector-map (category r7rs:base vs:vectors vs:functions vs:conversions vs:loops) (type map)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector-for-each (category r7rs:base vs:vectors vs:functions vs:loops) (type for-each)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(string? (category r7rs:base vs:strings vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string (category r7rs:base vs:strings) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-string (category r7rs:base vs:strings) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-length (category r7rs:base vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-append (category r7rs:base vs:strings) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-copy (category r7rs:base vs:strings) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-copy! (category r7rs:base vs:strings) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-fill! (category r7rs:base vs:strings) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(substring  (category r7rs:base vs:strings) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-ref (category r7rs:base vs:strings) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-set! (category r7rs:base vs:strings) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string<? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator<)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string>? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator>)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string<=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator<=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string>=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator>=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-ci=? (category r7rs:char vs:strings vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-ci<? (category r7rs:char vs:strings vs:comparisons) (type comparator<)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-ci>? (category r7rs:char vs:strings vs:comparisons) (type comparator>)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-ci<=? (category r7rs:char vs:strings vs:comparisons) (type comparator<=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-ci>=? (category r7rs:char vs:strings vs:comparisons) (type comparator>=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(number->string (category r7rs:base vs:strings vs:conversions) (type converter)
			(description
				#<<<
					
					````
					(number->string z)
					(number->string z radix)
					````
					
					
					**Domain**:  It is an error if `radix` is not one of `2`, `8`, `10`, or `16`.
					
					The procedure `number->string` takes a
					number and a radix and returns as a string an external representation of
					the given number in the given radix such that
					````
					(let ((number number)
					      (radix radix))
					  (eqv? number
					        (string->number (number->string number
					                                        radix)
					                        radix)))
					````
					is true.  It is an error if no possible result makes this expression true.
					If omitted, `radix` defaults to `10`.
					
					If `z` is inexact, the radix is `10`, and the above expression
					can be satisfied by a result that contains a decimal point,
					then the result contains a decimal point and is expressed using the
					minimum number of digits (exclusive of exponent and trailing
					zeroes) needed to make the above expression
					true;
					otherwise the format of the result is unspecified.
					
					The result returned by `number->string`
					never contains an explicit radix prefix.
					
					**Note**:  The error case can occur only when `z` is not a complex number
					or is a complex number with a non-rational real or imaginary part.
					
					**Rationale**:  If `z` is an inexact number and
					the radix is `10`, then the above expression is normally satisfied by
					a result containing a decimal point.  The unspecified case
					allows for infinities, NaNs, and unusual representations.
					
				>>>#))
		
		(string->number (category r7rs:base vs:strings vs:conversions) (type converter)
			(description
				#<<<
					
					````
					(string->number string)
					(string->number string radix)
					````
					
					
					Returns a number of the maximally precise representation expressed by the
					given `string`.
					
					**Domain**:  It is an error if `radix` is not `2`, `8`, `10`, or `16`.
					
					If supplied, `radix` is a default radix that will be overridden
					if an explicit radix prefix is present in `string` (e.g. `"#o177"`).  If `radix`
					is not supplied, then the default radix is `10`.  If `string` is not
					a syntactically valid notation for a number, or would result in a
					number that the implementation cannot represent, then `string->number`
					returns `#f`.
					An error is never signaled due to the content of `string`.
					
					````
					(string->number "100")        ===>  100
					(string->number "100" 16)     ===>  256
					(string->number "1e2")        ===>  100.0
					````
					
					**Note**:  The domain of `string->number` may be restricted by implementations
					in the following ways.
					If all numbers supported by an implementation are real, then
					`string->number` is permitted to return `#f` whenever
					`string` uses the polar or rectangular notations for complex
					numbers.  If all numbers are integers, then
					`string->number` may return `#f` whenever
					the fractional notation is used.  If all numbers are exact, then
					`string->number` may return `#f` whenever
					an exponent marker or explicit exactness prefix is used.
					If all inexact
					numbers are integers, then
					`string->number` may return `#f` whenever
					a decimal point is used.
					
					**Note**:  The rules used by a particular implementation for `string->number` must
					also be applied to `read` and to the routine that reads programs, in
					order to maintain consistency between internal numeric processing, I/O,
					and the processing of programs.
					As a consequence, the __R5RS__ permission to return `#f` when
					`string` has an explicit radix prefix has been withdrawn.
					
				>>>#))
		
		
		(symbol->string (category r7rs:base vs:strings vs:symbols vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string->symbol (category r7rs:base vs:strings vs:symbols vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(list->string (category r7rs:base vs:strings vs:lists vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string->list (category r7rs:base vs:strings vs:lists vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(vector->string (category r7rs:base vs:strings vs:vectors vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string->vector (category r7rs:base vs:strings vs:vectors vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-map (category r7rs:base vs:strings vs:functions vs:conversions vs:loops) (type map)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-for-each (category r7rs:base vs:strings vs:functions vs:loops) (type for-each)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(string-upcase (category r7rs:char vs:strings vs:conversions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-downcase (category r7rs:char vs:strings vs:conversions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string-foldcase (category r7rs:char vs:strings vs:conversions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(bytevector? (category r7rs:base vs:bytes) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(bytevector (category r7rs:base vs:bytes) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-bytevector (category r7rs:base vs:bytes) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(bytevector-length (category r7rs:base vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(bytevector-append (category r7rs:base vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(bytevector-copy (category r7rs:base vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(bytevector-copy! (category r7rs:base vs:bytes) (type procedure!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(bytevector-u8-ref (category r7rs:base vs:bytes) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(bytevector-u8-set! (category r7rs:base vs:bytes) (type mutator!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(utf8->string (category r7rs:base vs:bytes vs:strings) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string->utf8 (category r7rs:base vs:bytes vs:strings) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(port? (category r7rs:base vs:ports vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(binary-port? (category r7rs:base vs:ports) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(textual-port? (category r7rs:base vs:ports) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(input-port? (category r7rs:base vs:ports:input) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(input-port-open? (category r7rs:base vs:ports:input vs:ports:open) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(output-port? (category r7rs:base vs:ports:output) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(output-port-open? (category r7rs:base vs:ports:output vs:ports:open) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(open-input-bytevector (category r7rs:base vs:ports:input vs:ports:open vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(open-output-bytevector (category r7rs:base vs:ports:output vs:ports:open vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(get-output-bytevector (category r7rs:base vs:ports:output vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(open-input-string (category r7rs:base vs:ports:input vs:ports:open vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(open-output-string (category r7rs:base vs:ports:output vs:ports:open vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(get-output-string (category r7rs:base vs:ports:output vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(close-port (category r7rs:base vs:ports) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(close-input-port (category r7rs:base vs:ports:input) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(close-output-port (category r7rs:base vs:ports:output) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(u8-ready? (category r7rs:base vs:ports:input vs:bytes) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(peek-u8 (category r7rs:base vs:ports:input vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(read-u8 (category r7rs:base vs:ports:input vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write-u8 (category r7rs:base vs:ports:output vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(read-bytevector (category r7rs:base vs:ports:input vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(read-bytevector! (category r7rs:base vs:ports:input vs:bytes) (type procedure!)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write-bytevector (category r7rs:base vs:ports:output vs:bytes) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char-ready? (category r7rs:base vs:ports:input vs:strings vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(peek-char (category r7rs:base vs:ports:input vs:strings vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(read-char (category r7rs:base vs:ports:input vs:strings vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write-char (category r7rs:base vs:ports:output vs:strings vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(read-string (category r7rs:base vs:ports:input vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(read-line (category r7rs:base vs:ports:input vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(newline (category r7rs:base vs:ports:output vs:bytes vs:strings) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(flush-output-port (category r7rs:base vs:ports:output) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(read (category r7rs:read vs:ports:input vs:ports:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write (category r7rs:write vs:ports:output vs:ports:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write-simple (category r7rs:write vs:ports:output vs:ports:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(write-shared (category r7rs:write vs:ports:output vs:ports:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(display (category r7rs:write vs:ports:output vs:ports:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(open-input-file (category r7rs:file vs:ports:input vs:ports:open) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(open-binary-input-file (category r7rs:file vs:ports:input vs:ports:open) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(open-output-file (category r7rs:file vs:ports:output vs:ports:open) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(open-binary-output-file (category r7rs:file vs:ports:output vs:ports:open) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(call-with-port (category r7rs:base vs:ports vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(call-with-input-file (category r7rs:file vs:ports:input vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(call-with-output-file (category r7rs:file vs:ports:output vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(eof-object (category r7rs:base vs:ports vs:globals) (type constant)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(eof-object? (category r7rs:base vs:ports vs:globals) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(file-exists? (category r7rs:file vs:file-system) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(delete-file (category r7rs:file vs:file-system) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(exit (category r7rs:process-context) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(emergency-exit (category r7rs:process-context) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(command-line (category r7rs:process-context) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(get-environment-variable (category r7rs:process-context) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(get-environment-variables (category r7rs:process-context) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(current-second (category r7rs:time) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(current-jiffy (category r7rs:time) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(jiffies-per-second (category r7rs:time) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(char? (category r7rs:base vs:characters vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char=? (category r7rs:base vs:characters vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char<? (category r7rs:base vs:characters vs:comparisons) (type comparator<)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char>? (category r7rs:base vs:characters vs:comparisons) (type comparator>)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char<=? (category r7rs:base vs:characters vs:comparisons) (type comparator<=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char>=? (category r7rs:base vs:characters vs:comparisons) (type comparator>=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char-ci=? (category r7rs:char vs:characters vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-ci<? (category r7rs:char vs:characters vs:comparisons) (type comparator<)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-ci>? (category r7rs:char vs:characters vs:comparisons) (type comparator>)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-ci<=? (category r7rs:char vs:characters vs:comparisons) (type comparator<=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-ci>=? (category r7rs:char vs:characters vs:comparisons) (type comparator>=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char->integer (category r7rs:base vs:characters) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(integer->char (category r7rs:base vs:characters) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(digit-value (category r7rs:char vs:characters) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char-alphabetic? (category r7rs:char vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-upper-case? (category r7rs:char vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-lower-case? (category r7rs:char vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-numeric? (category r7rs:char vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-whitespace? (category r7rs:char vs:characters) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(char-upcase (category r7rs:char vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-downcase (category r7rs:char vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(char-foldcase (category r7rs:char vs:characters) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(procedure? (category r7rs:base vs:functions vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(apply (category r7rs:base vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(values (category r7rs:base vs:functions vs:values) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(call-with-values (category r7rs:base vs:functions vs:values) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(error-object? (category r7rs:base vs:errors) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(read-error? (category r7rs:base vs:errors) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(file-error? (category r7rs:base vs:errors) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(error (category r7rs:base vs:errors) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(error-object-message (category r7rs:base vs:errors) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(error-object-irritants (category r7rs:base vs:errors) (type accessor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(guard (category r7rs:base vs:errors vs:evaluator) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(else literal)
						(clause-condition expression)
						(clause-expression expression)
						(clause pattern
							(clause-condition)
							(clause-condition clause-expression ...)
							(else clause-expression ...))
						(guarded-expression expression))
				(_ (variable clause ...) guarded-expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(with-exception-handler (category r7rs:base vs:errors vs:evaluator) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(raise (category r7rs:base vs:errors vs:evaluator) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(raise-continuable (category r7rs:base vs:errors vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(parameterize (category r7rs:base vs:parameters) (type syntax)
			(syntax-rules
					(
						(parameter expression)
						(initializer expression)
						(parameters pattern
							()
							((parameter initializer) ...))
						(expression expression))
				(_ parameters)
				(_ parameters expression ...))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(make-parameter (category r7rs:base vs:parameters) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(current-input-port (category r7rs:base vs:parameters) (type parameter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(current-output-port (category r7rs:base vs:parameters) (type parameter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(current-error-port (category r7rs:base vs:parameters) (type parameter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(with-input-from-file (category r7rs:file vs:parameters vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(with-output-from-file (category r7rs:file vs:parameters vs:functions) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(delay (category r7rs:lazy vs:promises vs:evaluator) (type syntax)
			(syntax-rules
					((expression expression))
				(_ expression))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(delay-force (category r7rs:lazy vs:promises vs:evaluator) (type syntax)
			(syntax-rules
					((expression expression))
				(_ expression))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(promise? (category r7rs:lazy vs:promises vs:evaluator) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(make-promise (category r7rs:lazy vs:promises vs:evaluator) (type constructor)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(force (category r7rs:lazy vs:promises vs:evaluator) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(eval (category r7rs:eval vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(environment (category r7rs:eval vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(interaction-environment (category r7rs:r5rs r7rs:repl vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(scheme-report-environment (category r7rs:r5rs vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(null-environment (category r7rs:r5rs vs:evaluator vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(call-with-current-continuation (category r7rs:base vs:continuations vs:unsupported) (type procedure) (alias call/cc)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(dynamic-wind (category r7rs:base vs:continuations vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(cond-expand (category r7rs:base vs:compiler vs:unsupported) (type syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(features (category r7rs:base vs:evaluator vs:compiler vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(include (category r7rs:base vs:compiler vs:unsupported) (type syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(include-ci (category r7rs:base vs:compiler vs:unsupported) (type syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(import (category r7rs:base vs:compiler vs:unsupported) (type syntax)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(load (category r7rs:load vs:compiler vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
	)
	
	
	(types
		
		(any (category r7rs-x:types)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(null (category r7rs-x:types-disjoint) (parent any) (predicate null?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(boolean (category r7rs-x:types-disjoint) (parent any) (predicate boolean?)
			(description
				#<<<
					
					The standard boolean objects for true and false are written as
					`#t` and `#f`.
					Alternatively, they can be written `#true` and `#false`,
					respectively.  What really
					matters, though, are the objects that the Scheme conditional expressions
					(`if`, `cond`, `and`, `or`, `when`, `unless`, `do`) treat as
					true or false.  The phrase __a true value__
					(or sometimes just __true__) means any object treated as true by the
					conditional expressions, and the phrase __a false value__ (or
					__false__) means any object treated as false by the conditional expressions.
					
					Of all the Scheme values, only `#f`
					counts as false in conditional expressions.
					All other Scheme values, including `#t`,
					count as true.
					
					**Note**:  Unlike some other dialects of Lisp,
					Scheme distinguishes `#f` and the empty list __empty list__
					from each other and from the symbol `nil`.
					
					
					Boolean constants evaluate to themselves, so they do not need to be quoted
					in programs.
					
					````
					#t        ===>  #t
					#f        ===>  #f
					'#f       ===>  #f
					````
					
				>>>#))
		
		(number (category r7rs-x:types-disjoint) (parent any) (predicate number?)
			(description
				#<<<
					
					It is important to distinguish between mathematical numbers, the
					Scheme numbers that attempt to model them, the machine representations
					used to implement the Scheme numbers, and notations used to write numbers.
					This report uses the types `number`, `complex`, `real`,
					`rational`, and `integer` to refer to both mathematical numbers
					and Scheme numbers.
					
					
					##### Numerical types
					
					Mathematically, numbers are arranged into a tower of subtypes
					in which each level is a subset of the level above it:
					  * number
					  * complex number
					  * real number
					  * rational number
					  * integer
					
					For example, 3 is an integer.  Therefore 3 is also a rational,
					a real, and a complex number.  The same is true of the Scheme numbers
					that model 3.  For Scheme numbers, these types are defined by the
					predicates `number?`, `complex?`, `real?`, `rational?`,
					and `integer?`.
					
					There is no simple relationship between a number's type and its
					representation inside a computer.  Although most implementations of
					Scheme will offer at least two different representations of 3, these
					different representations denote the same integer.
					
					Scheme's numerical operations treat numbers as abstract data, as
					independent of their representation as possible.  Although an implementation
					of Scheme may use multiple internal representations of
					numbers, this ought not to be apparent to a casual programmer writing
					simple programs.
					
					
					##### Exactness
					
					It is useful to distinguish between numbers that are
					represented exactly and those that might not be.  For example, indexes
					into data structures must be known exactly, as must some polynomial
					coefficients in a symbolic algebra system.  On the other hand, the
					results of measurements are inherently inexact, and irrational numbers
					may be approximated by rational and therefore inexact approximations.
					In order to catch uses of inexact numbers where exact numbers are
					required, Scheme explicitly distinguishes exact from inexact numbers.
					This distinction is orthogonal to the dimension of type.
					
					A Scheme number is
					`exact` if it was written as an exact constant or was derived from
					__exact__ numbers using only __exact__ operations.  A number is
					`inexact` if it was written as an inexact constant,
					if it was
					derived using __inexact__ ingredients, or if it was derived using
					__inexact__ operations. Thus __inexact__-ness is a contagious
					property of a number.
					
					In particular, an __exact complex number__ has an exact real part
					and an exact imaginary part; all other complex numbers are
					__inexact complex numbers__.
					
					If two implementations produce __exact__ results for a
					computation that did not involve __inexact__ intermediate results,
					the two ultimate results will be mathematically equal.  This is
					generally not true of computations involving __inexact__ numbers
					since approximate methods such as floating-point arithmetic may be used,
					but it is the duty of each implementation to make the result as close as
					practical to the mathematically ideal result.
					
					Rational operations such as `+` should always produce
					__exact__ results when given __exact__ arguments.
					If the operation is unable to produce an __exact__ result,
					then it may either report the violation of an implementation restriction
					or it may silently coerce its
					result to an __inexact__ value.
					However, `(/ 3 4)` must not return the mathematically incorrect value `0`.
					
					Except for `exact`, the operations described in
					this section must generally return inexact results when given any inexact
					arguments.  An operation may, however, return an __exact__ result if it can
					prove that the value of the result is unaffected by the inexactness of its
					arguments.  For example, multiplication of any number by an __exact__ zero
					may produce an __exact__ zero result, even if the other argument is
					__inexact__.
					
					Specifically, the expression `(* 0 +inf.0)` may return `0`,
					or `+nan.0`, or report that inexact numbers are not supported,
					or report that non-rational real numbers are not supported, or fail
					silently or noisily in other implementation-specific ways.
					
					
					##### Implementation restrictions
					
					Implementations of Scheme are not required to implement the whole
					tower of subtypes given in the section "Numerical types",
					but they must implement a coherent subset consistent with both the
					purposes of the implementation and the spirit of the Scheme language.
					For example, implementations in which all numbers are __real__,
					or in which non-__real__ numbers are always __inexact__,
					or in which __exact__ numbers are always __integer__,
					are still quite useful.
					
					Implementations may also support only a limited range of numbers of
					any type, subject to the requirements of this section.  The supported
					range for __exact__ numbers of any type may be different from the
					supported range for __inexact__ numbers of that type.  For example,
					an implementation that uses IEEE binary double-precision floating-point numbers to represent all its
					__inexact__ __real__ numbers may also
					support a practically unbounded range of __exact__ __integer__'s
					and __rational__'s
					while limiting the range of __inexact__ __real__'s (and therefore
					the range of __inexact__ __integer__'s and __rational__'s)
					to the dynamic range of the IEEE binary double format.
					Furthermore,
					the gaps between the representable __inexact__ __integer__'s and
					__rational__'s are
					likely to be very large in such an implementation as the limits of this
					range are approached.
					
					An implementation of Scheme must support exact integers
					throughout the range of numbers permitted as indexes of
					lists, vectors, bytevectors, and strings or that result from computing the length of
					one of these.  The `length`, `vector-length`,
					`bytevector-length`, and `string-length` procedures must return an exact
					integer, and it is an error to use anything but an exact integer as an
					index.  Furthermore, any integer constant within the index range, if
					expressed by an exact integer syntax, must be read as an exact
					integer, regardless of any implementation restrictions that apply
					outside this range.  Finally, the procedures listed below will always
					return exact integer results provided all their arguments are exact integers
					and the mathematically expected results are representable as exact integers
					within the implementation:
					
					````
					-                     *
					+                     abs
					ceiling               denominator
					exact-integer-sqrt    expt
					floor                 floor/
					floor-quotient        floor-remainder
					gcd                   lcm
					max                   min
					modulo                numerator
					quotient              rationalize
					remainder             round
					square                truncate
					truncate/             truncate-quotient
					truncate-remainder
					````
					
					It is recommended, but not required, that implementations support
					__exact__ __integer__'s and __exact__ __rational__'s of
					practically unlimited size and precision, and to implement the
					above procedures and the `/` procedure in
					such a way that they always return __exact__ results when given __exact__
					arguments.  If one of these procedures is unable to deliver an __exact__
					result when given __exact__ arguments, then it may either report a
					violation of an
					implementation restriction or it may silently coerce its result to an
					__inexact__ number; such a coercion can cause an error later.
					Nevertheless, implementations that do not provide __exact__ rational
					numbers should return __inexact__ rational numbers rather than
					reporting an implementation restriction.
					
					An implementation may use floating-point and other approximate
					representation strategies for __inexact__ numbers.
					This report recommends, but does not require, that
					implementations that use
					floating-point representations
					follow the IEEE 754 standard,
					and that implementations using
					other representations should match or exceed the precision achievable
					using these floating-point standards.
					In particular, the description of transcendental functions in IEEE 754-2008
					should be followed by such implementations, particularly with respect
					to infinities and `NaN`s.
					
					Although Scheme allows a variety of written
					notations for
					numbers, any particular implementation may support only some of them.
					For example, an implementation in which all numbers are __real__
					need not support the rectangular and polar notations for complex
					numbers.  If an implementation encounters an __exact__ numerical constant that
					it cannot represent as an __exact__ number, then it may either report a
					violation of an implementation restriction or it may silently represent the
					constant by an __inexact__ number.
					
					
					##### Implementation extensions
					
					Implementations may provide more than one representation of
					floating-point numbers with differing precisions.  In an implementation
					which does so, an inexact result must be represented with at least
					as much precision as is used to express any of the inexact arguments
					to that operation.  Although it is desirable for potentially inexact
					operations such as `sqrt` to produce __exact__ answers when
					applied to __exact__ arguments, if an __exact__ number is operated
					upon so as to produce an __inexact__ result, then the most precise
					representation available must be used.  For example, the value of
					`(sqrt 4)` should be `2`, but in an implementation that provides both
					single and double precision floating point numbers it may be the latter
					but must not be the former.
					
					It is the programmer's responsibility to avoid using inexact
					number objects with magnitude or significand too large to be
					represented in the implementation.
					
					In addition, implementations may
					distinguish special numbers called __positive infinity__,
					__negative infinity__, __NaN__, and __negative zero__.
					
					Positive infinity is regarded as an inexact real (but not rational)
					number that represents an indeterminate value greater than the
					numbers represented by all rational numbers. Negative infinity
					is regarded as an inexact real (but not rational) number that
					represents an indeterminate value less than the numbers represented
					by all rational numbers.
					
					Adding or multiplying an infinite value by any finite real value results
					in an appropriately signed infinity; however, the sum of positive and
					negative infinities is a `NaN`.  Positive infinity is the reciprocal
					of zero, and negative infinity is the reciprocal of negative zero.
					The behavior of the transcendental functions is sensitive to infinity
					in accordance with IEEE 754.
					
					A `NaN` is regarded as an inexact real (but not rational) number
					so indeterminate that it might represent any real value, including
					positive or negative infinity, and might even be greater than positive
					infinity or less than negative infinity.
					An implementation that does not support non-real numbers may use `NaN`
					to represent non-real values like `(sqrt -1.0)` and `(asin 2.0)`.
					
					A `NaN` always compares false to any number, including a `NaN`.
					An arithmetic operation where one operand is `NaN` returns `NaN`, unless the
					implementation can prove that the result would be the same if the `NaN`
					were replaced by any rational number.  Dividing zero by zero results in
					`NaN` unless both zeros are exact.
					
					IEEE 754 specifies multiple `NaN` values.  Scheme generally does
					not care if there is a single value (bit pattern) for `NaN`,
					or if there are multiple values: if there are multiple `NaN`
					values, or just one, they are all equivalent in terms of Scheme
					computation.
					
					Negative zero is an inexact real value written `-0.0` and is distinct
					(in the sense of `eqv?`) from `0.0`.  A Scheme implementation
					is not required to distinguish negative zero.  If it does, however, the
					behavior of the transcendental functions is sensitive to the distinction
					in accordance with IEEE 754.
					Specifically, in a Scheme implementing both complex numbers and negative zero,
					the branch cut of the complex logarithm function is such that
					`(imag-part (log -1.0-0.0i))` is `-pi` rather than `pi`.
					
					Furthermore, the negation of negative zero is ordinary zero and vice
					versa.  This implies that the sum of two or more negative zeros is negative,
					and the result of subtracting (positive) zero from a negative zero is
					likewise negative.  However, numerical comparisons treat negative zero
					as equal to zero.
					
					Note that both the real and the imaginary parts of a complex number
					can be infinities, `NaN`s, or negative zero.
					
					
					##### Syntax of numerical constants
					
					The syntax of the written representations for numbers is described formally in the
					section on formal syntax.  Note that case is not significant in numerical
					constants.
					
					A number can be written in binary, octal, decimal, or
					hexa-decimal by the use of a radix prefix.  The radix prefixes are
					`#b` (binary), `#o` (octal),
					`#d` (decimal), and `#x` (hexa-decimal).  With
					no radix prefix, a number is assumed to be expressed in decimal.
					
					A
					numerical constant can be specified to be either __exact__ or
					__inexact__ by a prefix.  The prefixes are `#e`
					for __exact__, and `#i` for __inexact__.  An exactness
					prefix can appear before or after any radix prefix that is used.  If
					the written representation of a number has no exactness prefix, the
					constant is
					__inexact__ if it contains a decimal point or an
					exponent.
					Otherwise, it is __exact__.
					
					In systems with __inexact__ numbers
					of varying precisions it can be useful to specify
					the precision of a constant.  For this purpose,
					implementations may accept numerical constants
					written with an exponent marker that indicates the
					desired precision of the __inexact__
					representation.  If so, the letter `s`, `f`,
					`d`, or `l`, meaning __short__, __single__,
					__double__, or __long__ precision, respectively,
					can be used in place of `e`.
					The default precision has at least as much precision
					as __double__, but
					implementations may allow this default to be set by the user.
					
					````
					3.14159265358979F0
					       Round to single  ---  3.141593
					0.6L0
					       Extend to long   ---  .600000000000000
					````
					
					The numbers positive infinity, negative infinity, and `NaN` are written
					`+inf.0`, `-inf.0` and `+nan.0` respectively.
					`NaN` may also be written `-nan.0`.
					The use of signs in the written representation does not necessarily
					reflect the underlying sign of the `NaN` value, if any.
					Implementations are not required to support these numbers, but if they do,
					they must do so in general conformance with IEEE 754.  However, implementations
					are not required to support signaling `NaN`s, nor to provide a way to distinguish
					between different `NaN`s.
					
					There are two notations provided for non-real complex numbers:
					the __rectangular notation__
					`a + b i`,
					where `a` is the real part and `b` is the imaginary part;
					and the __polar notation__
					`r @ theta`,
					where `r` is the magnitude and `theta` is the phase (angle) in radians.
					These are related by the equation
					`a + b i = r cos(theta) + (r sin (theta)) i`.
					All of `a`, `b`, `r`, and `theta` are real numbers.
					
				>>>#))
		
		(symbol (category r7rs-x:types-disjoint) (parent any) (predicate symbol?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(character (category r7rs-x:types-disjoint) (parent any) (predicate char?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(string (category r7rs-x:types-disjoint) (parent any) (predicate string?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(bytevector (category r7rs-x:types-disjoint) (parent any) (predicate bytevector?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(vector (category r7rs-x:types-disjoint) (parent any) (predicate vector?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(pair (category r7rs-x:types-disjoint) (parent any) (predicate pair?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(port (category r7rs-x:types-disjoint) (parent any) (predicate port?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(eof-object (category r7rs-x:types-disjoint) (parent any) (predicate eof-object?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(procedure (category r7rs-x:types-disjoint) (parent any) (predicate procedure?)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
	)
	
	
	
	
)

