
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
					
					The `eqv?` procedure defines a useful equivalence relation on objects.
					Briefly, it returns `#t` if `obj-1` and `obj-2` are
					normally regarded as the same object.  This relation is left slightly
					open to interpretation, but the following partial specification of
					`eqv?` holds for all implementations of Scheme.
					
					
					The `eqv?` procedure returns `#t` if:
					
					  * `obj-1` and `obj-2` are both `#t` or both `#f`.
					
					  * `obj-1` and `obj-2` are both symbols and are the same
					symbol according to the `symbol=?` procedure.
					
					  * `obj-1` and `obj-2` are both exact numbers and
					are numerically equal (in the sense of `=`).
					
					  * `obj-1` and `obj-2` are both inexact numbers such that
					they are numerically equal (in the sense of `=`)
					and they yield the same results (in the sense of `eqv?`)
					when passed as arguments to any other procedure
					that can be defined as a finite composition of Scheme's standard
					arithmetic procedures, provided it does not result in a `NaN` value.
					
					  * `obj-1` and `obj-2` are both characters and are the same
					character according to the `char=?` procedure.
					
					  * `obj-1` and `obj-2` are both the empty list.
					
					  * `obj-1` and `obj-2` are pairs, vectors, bytevectors, records,
					or strings that denote the same location in the store.
					
					  * `obj-1` and `obj-2` are procedures whose location tags are
					equal.
					
					
					The `eqv?` procedure returns `#f` if:
					
					  * `obj-1` and `obj-2` are of different types.
					
					  * one of `obj-1` and `obj-2` is `#t` but the other is
					`#f`.
					
					  * `obj-1` and `obj-2` are symbols but are not the same
					symbol according to the `symbol=?` procedure.
					
					  * one of `obj-1` and `obj-2` is an exact number but the other
					is an inexact number.
					
					  * `obj-1` and `obj-2` are both exact numbers and
					are numerically unequal (in the sense of `=`).
					
					  * `obj-1` and `obj-2` are both inexact numbers such that either
					they are numerically unequal (in the sense of `=`),
					or they do not yield the same results (in the sense of `eqv?`)
					when passed as arguments to any other procedure
					that can be defined as a finite composition of Scheme's standard
					arithmetic procedures, provided it does not result in a `NaN` value.
					As an exception, the behavior of `eqv?` is unspecified
					when both `obj-1` and `obj-2` are `NaN`.
					
					  * `obj-1` and `obj-2` are characters for which the `char=?`
					procedure returns `#f`.
					
					  * one of `obj-1` and `obj-2` is the empty list but the other
					is not.
					
					  * `obj-1` and `obj-2` are pairs, vectors, bytevectors, records,
					or strings that denote distinct locations.
					
					  * `obj-1` and `obj-2` are procedures that would behave differently
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
		
		
		
		
		(not (category r7rs:base) (type predicate)
			(signature (any -> boolean))
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		
		
		(boolean? (category r7rs:base vs:booleans vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(boolean=? (category r7rs:base vs:booleans vs:comparisons vs:equivalence) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
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
					
					**FIXME!**
					
				>>>#))
		
		(integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(real? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(rational? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(complex? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(exact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(inexact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(exact-integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(zero? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(positive? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(odd? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(even? (category r7rs:base vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(< (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(> (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(<= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(>= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>=)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(+ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(- (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(* (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(abs (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(floor/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(floor-quotient (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(floor-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias modulo)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(truncate/ (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(truncate-quotient (category r7rs:base vs:arithmetic) (type procedure) (alias quotient)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(truncate-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias remainder)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(floor (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(ceiling (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(truncate (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(round (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(min (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(max (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(gcd (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(lcm (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(expt (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(square (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(exact-integer-sqrt (category r7rs:base vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(rationalize (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(numerator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(denominator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(inexact (category r7rs:complex vs:arithmetic) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(exact (category r7rs:complex vs:arithmetic) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		
		(make-rectangular (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
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
					
					**FIXME!**
					
				>>>#))
		
		(exp (category r7rs:inexact vs:arithmetic) (type procedure)
			(description
				#<<<
					
					**FIXME!**
					
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
					
					**FIXME!**
					
				>>>#))
		
		(infinite? (category r7rs:inexact vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
				>>>#))
		
		(nan? (category r7rs:inexact vs:arithmetic) (type predicate)
			(description
				#<<<
					
					**FIXME!**
					
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
					
					**FIXME!**
					
				>>>#))
		
		(string->number (category r7rs:base vs:strings vs:conversions) (type converter)
			(description
				#<<<
					
					**FIXME!**
					
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
					
					**FIXME!**
					
				>>>#))
		
		(number (category r7rs-x:types-disjoint) (parent any) (predicate number?)
			(description
				#<<<
					
					**FIXME!**
					
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

