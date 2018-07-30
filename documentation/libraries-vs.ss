
(library
	
	
	
	
	(identifier vonuvoli)
	
	(title "Vonuvoli-Scheme builtin functionality")
	
	(description
		#<<<
					
					**FIXME!**
					
		>>>#)
	
	
	
	
	(use-types r7rs)
	
	
	
	
	(categories
		
		(vs:arithmetic)
		
		(vs:associations)
		
		(vs:bytes)
		
		(vs:booleans)
		
		(vs:conversions)
		
		(vs:globals)
		
		(vs:file-system)
		
		(vs:characters)
		
		(vs:comparisons)
		
		(vs:compiler)
		
		(vs:contexts)
		
		(vs:continuations)
		
		(vs:control)
		
		(vs:equivalence)
		
		(vs:errors)
		
		(vs:evaluator)
		
		(vs:functions)
		
		(vs:lambda)
		
		(vs:lists)
		
		(vs:loops)
		
		(vs:modules)
		
		(vs:pairs)
		
		(vs:parameters)
		
		(vs:ports)
		
		(vs:ports:input
			(parent vs:ports))
		
		(vs:ports:output
			(parent vs:ports))
		
		(vs:ports:open
			(parent vs:ports))
		
		(vs:ports:values
			(parent vs:ports))
		
		(vs:promises)
		
		(vs:quotation)
		
		(vs:records)
		
		(vs:strings)
		
		(vs:symbols)
		
		(vs:syntaxes)
		
		(vs:system)
		
		(vs:types)
		
		(vs:unimplemented)
		
		(vs:unsupported)
		
		(vs:values)
		
		(vs:vectors)
		
		(vs:r7rs)
	)
	
	
	
	
	(definitions
		
		
		
		
		;; ---- syntaxes
		
		
		(quote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(implements (r7rs quote)))
		
		(quasiquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(implements (r7rs quasiquote)))
		
		(unquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(implements (r7rs unquote)))
		
		(unquote-splicing
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(implements (r7rs unquote-splicing)))
		
		
		(_
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(implements (r7rs _)))
		
		(...
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(implements (r7rs ...)))
		
		(=>
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(implements (r7rs =>)))
		
		(else
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(implements (r7rs else)))
		
		
		(begin
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs begin)))
		
		(and
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs and)))
		
		(or
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs or)))
		
		
		(if
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs if)))
		
		(when
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs when)))
		
		(unless
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs unless)))
		
		(cond
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs cond)))
		
		(case
			(category vs:r7rs vs:control)
			(type syntax)
			(implements (r7rs case)))
		
		
		(do
			(category vs:r7rs vs:control vs:loops)
			(type syntax)
			(implements (r7rs do)))
		
		(while
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		(until
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		
		(do-cond
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		(while-cond
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		(until-cond
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		
		(loop
			(category vs:loops vs:syntaxes)
			(type syntax))
		
		
		(guard
			(category vs:r7rs vs:errors vs:evaluator)
			(type syntax)
			(implements (r7rs guard)))
		
		(guard*
			(category vs:syntaxes)
			(type syntax))
		
		
		(locals
			(category vs:contexts vs:syntaxes)
			(type syntax))
		
		(let
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs let)))
		
		(let*
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs let*)))
		
		(letrec
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs letrec)))
		
		(letrec*
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs letrec*)))
		
		(let-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(implements (r7rs let-values)))
		
		(let*-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(implements (r7rs let*-values)))
		
		
		(define
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs define)))
		
		(redefine
			(category vs:contexts vs:syntaxes)
			(type syntax))
		
		(define-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(implements (r7rs define-values)))
		
		(redefine-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax))
		
		(set!
			(category vs:r7rs vs:contexts)
			(type syntax)
			(implements (r7rs set!)))
		
		(set!-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax))
		
		
		(define-record-type
			(category vs:r7rs vs:contexts vs:records)
			(type syntax)
			(implements (r7rs define-record-type)))
		
		
		(lambda
			(category vs:r7rs vs:lambda)
			(type syntax)
			(implements (r7rs lambda)))
		
		
		(parameterize
			(category vs:r7rs vs:parameters)
			(type syntax)
			(implements (r7rs parameterize)))
		
		
		
		
		;; ---- types
		
		
		(not
			(category vs:r7rs vs:booleans)
			(type predicate)
			(implements (r7rs not)))
		
		
		(null?
			(category vs:r7rs vs:lists vs:types)
			(type type-predicate)
			(extends (r7rs null?))
			(signature
				((null 1...) -> true)
				((pair 1...) -> false)
				((any 1...) -> false))
		)
		
		(void?
			(type type-predicate))
		
		(undefined?
			(type type-predicate))
		
		
		(boolean?
			(category vs:r7rs vs:booleans vs:types)
			(type type-predicate)
			(extends (r7rs boolean?))
			(signature
				((boolean 1...) -> true)
				((any 1...) -> false))
		)
		
		(true?
			(type predicate))
		
		(false?
			(type predicate))
		
		(true-or-equivalent?
			(type predicate))
		
		(false-or-equivalent?
			(type predicate))
		
		
		(number?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs number?))
			(signature
				((integer 1...) -> true)
				((rational 1...) -> true)
				((real 1...) -> true)
				((complex 1...) -> true)
				((number 1...) -> true)
				((any 1...) -> false))
		)
		
		(complex?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs complex?))
			(signature
				((integer 1...) -> true)
				((rational 1...) -> true)
				((real 1...) -> true)
				((complex 1...) -> true)
				((number 1...) -> false)
				((any 1...) -> false))
		)
		
		(real?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs real?))
			(signature
				((integer 1...) -> true)
				((rational 1...) -> true)
				((real 1...) -> true)
				((complex 1...) -> false)
				((number 1...) -> false)
				((any 1...) -> false))
		)
		
		(rational?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs rational?))
			(signature
				((integer 1...) -> true)
				((rational 1...) -> true)
				((real 1...) -> false)
				((complex 1...) -> false)
				((number 1...) -> false)
				((any 1...) -> false))
		)
		
		(integer?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs integer?))
			(signature
				((integer 1...) -> true)
				((rational 1...) -> false)
				((real 1...) -> false)
				((complex 1...) -> false)
				((number 1...) -> false)
				((any 1...) -> false))
		)
		
		(exact-integer?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs exact-integer?))
			(signature
				((exact-integer 1...) -> true)
				((exact-number 1...) -> false)
				((inexact-number 1...) -> false)
				((number 1...) -> false))
		)
		
		(exact?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs exact?))
			(signature
				((exact-number 1...) -> true)
				((inexact-number 1...) -> false)
				((number 1...) -> false))
		)
		
		(inexact?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(extends (r7rs inexact?))
			(signature
				((inexact-number 1...) -> true)
				((exact-number 1...) -> false)
				((number 1...) -> false))
		)
		
		
		(char?
			(category vs:r7rs vs:characters vs:types)
			(type type-predicate)
			(extends (r7rs char?))
			(signature
				((character 1...) -> true)
				((any 1...) -> false))
		)
		
		(symbol?
			(category vs:r7rs vs:symbols vs:types)
			(type type-predicate)
			(extends (r7rs symbol?))
			(signature
				((symbol 1...) -> true)
				((any 1...) -> false))
		)
		
		(keyword?
			(type type-predicate))
		
		(unique?
			(type type-predicate))
		
		
		(string?
			(category vs:r7rs vs:strings vs:types)
			(type type-predicate)
			(extends (r7rs string?))
			(signature
				((string 1...) -> true)
				((any 1...) -> false))
		)
		
		(string-immutable?
			(type type-predicate))
		
		(string-mutable?
			(type type-predicate))
		
		(string-empty?
			(type predicate))
		
		(string-immutable-empty?
			(type predicate))
		
		(string-mutable-empty?
			(type predicate))
		
		
		(bytevector?
			(category vs:r7rs vs:bytes)
			(type type-predicate)
			(extends (r7rs bytevector?))
			(signature
				((bytevector 1...) -> true)
				((any 1...) -> false))
		)
		
		(bytevector-immutable?
			(type type-predicate))
		
		(bytevector-mutable?
			(type type-predicate))
		
		(bytevector-empty?
			(type predicate))
		
		(bytevector-immutable-empty?
			(type predicate))
		
		(bytevector-mutable-empty?
			(type predicate))
		
		
		(string-regex?
			(type type-predicate))
		
		(bytevector-regex?
			(type type-predicate))
		
		
		(pair?
			(category vs:r7rs vs:pairs vs:lists vs:types)
			(type type-predicate)
			(extends (r7rs pair?))
			(signature
				((pair 1...) -> true)
				((null 1...) -> false)
				((any 1...) -> false))
		)
		
		(pair-immutable?
			(type type-predicate))
		
		(pair-mutable?
			(type type-predicate))
		
		
		(vector?
			(category vs:r7rs vs:vectors vs:types)
			(type type-predicate)
			(extends (r7rs vector?))
			(signature
				((vector 1...) -> true)
				((any 1...) -> false))
		)
		
		(vector-immutable?
			(type type-predicate))
		
		(vector-mutable?
			(type type-predicate))
		
		(vector-empty?
			(type predicate))
		
		(vector-immutable-empty?
			(type predicate))
		
		(vector-mutable-empty?
			(type predicate))
		
		
		(values?
			(type type-predicate))
		
		(values-empty?
			(type predicate))
		
		
		(record-type?
			(type type-predicate))
		
		(record?
			(type type-predicate))
		
		(record-immutable?
			(type type-predicate))
		
		(record-mutable?
			(type type-predicate))
		
		
		(error-object?
			(category vs:r7rs vs:errors)
			(type type-predicate)
			(extends (r7rs error-object?))
			(signature
				((error-object 1...) -> true)
				((any 1...) -> false))
		)
		
		(syntax-error?
			(type predicate))
		
		(file-error?
			(category vs:r7rs vs:errors)
			(type predicate)
			(extends (r7rs file-error?))
			(signature
				((any 1...) -> boolean))
		)
		
		(port-error?
			(type type-predicate))
		
		(read-error?
			(category vs:r7rs vs:errors)
			(type predicate)
			(extends (r7rs read-error?))
			(signature
				((any 1...) -> boolean))
		)
		
		(write-error?
			(type type-predicate))
		
		
		
		
		(any-list?
			(type predicate))
		
		(empty-list?
			(type predicate)
			(alias null-list?))
		
		(any-or-empty-list?
			(type predicate))
		
		
		(proper-list?
			(type predicate))
		
		(proper-or-empty-list?
			(category vs:r7rs vs:lists vs:types)
			(type predicate)
			(alias list?)
			(extends (r7rs list?))
			(signature
				((null 1...) -> true)
				((list-proper 1...) -> true)
				((list-dotted 1...) -> false)
				((list-circular 1...) -> false)
				((any 1...) -> false))
		)
		
		
		(dotted-list?
			(type predicate))
		
		(dotted-or-empty-list?
			(type predicate))
		
		
		(circular-list?
			(type predicate))
		
		(circular-or-empty-list?
			(type predicate))
		
		
		(procedure?
			(category vs:r7rs vs:functions vs:types)
			(type type-predicate)
			(extends (r7rs procedure?))
			(signature
				((procedure 1...) -> true)
				((any 1...) -> false))
		)
		
		(syntax?
			(type predicate))
		
		
		(port?
			(category vs:r7rs vs:ports vs:types)
			(type type-predicate)
			(extends (r7rs port?))
			(signature
				((port 1...) -> true)
				((any 1...) -> false))
		)
		
		(input-port?
			(category vs:r7rs vs:ports:input)
			(type predicate)
			(extends (r7rs input-port?))
			(signature
				((input-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(output-port?
			(category vs:r7rs vs:ports:output)
			(type predicate)
			(extends (r7rs output-port?))
			(signature
				((output-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(binary-port?
			(category vs:r7rs vs:ports)
			(type predicate)
			(extends (r7rs binary-port?))
			(signature
				((binary-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(textual-port?
			(category vs:r7rs vs:ports)
			(type predicate)
			(extends (r7rs textual-port?))
			(signature
				((textual-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		
		(binary-input-port?
			(type predicate))
		
		(textual-input-port?
			(type predicate))
		
		(binary-output-port?
			(type predicate))
		
		(textual-output-port?
			(type predicate))
		
		
		(eof-object?
			(category vs:r7rs vs:ports vs:globals)
			(type predicate)
			(extends (r7rs eof-object?))
			(signature
				((eof-object 1...) -> true)
				((any 1...) -> false))
		)
		
		
		(path?
			(type type-predicate))
		
		(path-absolute?
			(type predicate))
		
		(path-relative?
			(type predicate))
		
		
		(process?
			(type type-predicate))
		
		
		(context?
			(type type-predicate))
		
		(binding?
			(type type-predicate))
		
		(parameters?
			(type type-predicate))
		
		(parameter?
			(type type-predicate))
		
		(promise?
			(category vs:r7rs vs:promises vs:evaluator)
			(type type-predicate)
			(extends (r7rs promise?))
			(signature
				((promise 1...) -> true)
				((any 1...) -> false))
		)
		
		(resource?
			(type type-predicate))
		
		(internal?
			(type type-predicate))
		
		(opaque?
			(type predicate))
		
		
		(zero?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(extends (r7rs zero?))
			(signature
				((number-zero 1...) -> true)
				((number 1...) -> false))
		)
		
		(positive?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(extends (r7rs positive?))
			(signature
				((real-zero 1...) -> false)
				((real-positive 1...) -> true)
				((real-negative 1...) -> false)
				((real 1...) -> false))
		)
		
		(negative?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(extends (r7rs negative?))
			(signature
				((real-zero 1...) -> false)
				((real-positive 1...) -> false)
				((real-negative 1...) -> true)
				((real 1...) -> false))
		)
		
		(finite?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(implements (r7rs finite?)))
		
		(infinite?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(implements (r7rs infinite?)))
		
		(nan?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(implements (r7rs nan?)))
		
		(even?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(extends (r7rs even?))
			(signature
				((integer-zero 1...) -> true)
				((integer-even 1...) -> true)
				((integer-odd 1...) -> false)
				((integer 1...) -> false))
		)
		
		(odd?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(extends (r7rs odd?))
			(signature
				((integer-zero 1...) -> false)
				((integer-even 1...) -> false)
				((integer-odd 1...) -> true)
				((integer 1...) -> false))
		)
		
		(char-numeric?
			(category vs:r7rs vs:characters)
			(type predicate)
			(implements (r7rs char-numeric?)))
		
		(char-alphabetic?
			(category vs:r7rs vs:characters)
			(type predicate)
			(implements (r7rs char-alphabetic?)))
		
		(char-upper-case?
			(category vs:r7rs vs:characters)
			(type predicate)
			(implements (r7rs char-upper-case?)))
		
		(char-lower-case?
			(category vs:r7rs vs:characters)
			(type predicate)
			(implements (r7rs char-lower-case?)))
		
		(char-alphabetic-or-numeric?
			(type predicate))
		
		(char-whitespace?
			(category vs:r7rs vs:characters)
			(type predicate)
			(implements (r7rs char-whitespace?)))
		
		(char-control?
			(type predicate))
		
		(char-ascii?
			(type predicate))
		
		(char-ascii-numeric?
			(type predicate))
		
		(char-ascii-numeric-8?
			(type predicate))
		
		(char-ascii-numeric-16?
			(type predicate))
		
		(char-ascii-alphabetic?
			(type predicate))
		
		(char-ascii-upper-case?
			(type predicate))
		
		(char-ascii-lower-case?
			(type predicate))
		
		(char-ascii-alphabetic-or-numeric?
			(type predicate))
		
		(char-ascii-whitespace?
			(type predicate))
		
		(char-ascii-control?
			(type predicate))
		
		(char-ascii-punctuation?
			(type predicate))
		
		(char-ascii-graphic?
			(type predicate))
		
		
		(fs-metadata?
			(type type-predicate))
		
		(cache?
			(type type-predicate))
		
		
		
		
		;; ---- types (negated)
		
		
		(not-null?
			(type predicate))
		
		(not-void?
			(type predicate))
		
		(not-undefined?
			(type predicate))
		
		(not-boolean?
			(type predicate))
		
		(not-true?
			(type predicate))
		
		(not-false?
			(type predicate))
		
		(not-true-or-equivalent?
			(type predicate))
		
		(not-false-or-equivalent?
			(type predicate))
		
		(not-number?
			(type predicate))
		
		(not-complex?
			(type predicate))
		
		(not-real?
			(type predicate))
		
		(not-rational?
			(type predicate))
		
		(not-integer?
			(type predicate))
		
		(not-exact-integer?
			(type predicate))
		
		(not-exact?
			(type predicate))
		
		(not-inexact?
			(type predicate))
		
		(not-char?
			(type predicate))
		
		(not-symbol?
			(type predicate))
		
		(not-keyword?
			(type predicate))
		
		(not-unique?
			(type predicate))
		
		(not-string?
			(type predicate))
		
		(not-string-immutable?
			(type predicate))
		
		(not-string-mutable?
			(type predicate))
		
		(not-string-empty?
			(type predicate))
		
		(not-string-immutable-empty?
			(type predicate))
		
		(not-string-mutable-empty?
			(type predicate))
		
		(not-bytevector?
			(type predicate))
		
		(not-bytevector-immutable?
			(type predicate))
		
		(not-bytevector-mutable?
			(type predicate))
		
		(not-bytevector-empty?
			(type predicate))
		
		(not-bytevector-immutable-empty?
			(type predicate))
		
		(not-bytevector-mutable-empty?
			(type predicate))
		
		(not-string-regex?
			(type predicate))
		
		(not-bytevector-regex?
			(type predicate))
		
		(not-pair?
			(type predicate))
		
		(not-pair-mutable?
			(type predicate))
		
		(not-pair-immutable?
			(type predicate))
		
		(not-vector?
			(type predicate))
		
		(not-vector-mutable?
			(type predicate))
		
		(not-vector-immutable?
			(type predicate))
		
		(not-vector-empty?
			(type predicate))
		
		(not-vector-mutable-empty?
			(type predicate))
		
		(not-vector-immutable-empty?
			(type predicate))
		
		(not-values?
			(type predicate))
		
		(not-values-empty?
			(type predicate))
		
		(not-record-type?
			(type predicate))
		
		(not-record?
			(type predicate))
		
		(not-record-immutable?
			(type predicate))
		
		(not-record-mutable?
			(type predicate))
		
		(not-error-object?
			(type predicate))
		
		(not-syntax-error?
			(type predicate))
		
		(not-file-error?
			(type predicate))
		
		(not-port-error?
			(type predicate))
		
		(not-read-error?
			(type predicate))
		
		(not-write-error?
			(type predicate))
		
		(not-any-list?
			(type predicate))
		
		(not-empty-list?
			(type predicate))
		
		(not-any-or-empty-list?
			(type predicate))
		
		(not-proper-list?
			(type predicate))
		
		(not-proper-or-empty-list?
			(type predicate)
			(alias not-list?))
		
		(not-dotted-list?
			(type predicate))
		
		(not-dotted-list-or-emtpy?
			(type predicate))
		
		(not-circular-list?
			(type predicate))
		
		(not-circular-or-empty-list?
			(type predicate))
		
		(not-procedure?
			(type predicate))
		
		(not-syntax?
			(type predicate))
		
		(not-port?
			(type predicate))
		
		(not-input-port?
			(type predicate))
		
		(not-output-port?
			(type predicate))
		
		(not-binary-port?
			(type predicate))
		
		(not-textual-port?
			(type predicate))
		
		(not-binary-input-port?
			(type predicate))
		
		(not-textual-input-port?
			(type predicate))
		
		(not-binary-output-port?
			(type predicate))
		
		(not-textual-output-port?
			(type predicate))
		
		(not-eof-object?
			(type predicate))
		
		(not-path?
			(type predicate))
		
		(not-path-absolute?
			(type predicate))
		
		(not-path-relative?
			(type predicate))
		
		(not-process?
			(type predicate))
		
		(not-context?
			(type predicate))
		
		(not-binding?
			(type predicate))
		
		(not-parameters?
			(type predicate))
		
		(not-parameter?
			(type predicate))
		
		(not-promise?
			(type predicate))
		
		(not-resource?
			(type predicate))
		
		(not-internal?
			(type predicate))
		
		(not-opaque?
			(type predicate))
		
		(not-zero?
			(type predicate))
		
		(not-positive?
			(type predicate))
		
		(not-negative?
			(type predicate))
		
		(not-finite?
			(type predicate))
		
		(not-infinite?
			(type predicate))
		
		(not-nan?
			(type predicate))
		
		(not-even?
			(type predicate))
		
		(not-odd?
			(type predicate))
		
		(not-char-numeric?
			(type predicate))
		
		(not-char-alphabetic?
			(type predicate))
		
		(not-char-upper-case?
			(type predicate))
		
		(not-char-lower-case?
			(type predicate))
		
		(not-char-alphabetic-or-numeric?
			(type predicate))
		
		(not-char-whitespace?
			(type predicate))
		
		(not-char-control?
			(type predicate))
		
		(not-char-ascii?
			(type predicate))
		
		(not-char-ascii-numeric?
			(type predicate))
		
		(not-char-ascii-numeric-8?
			(type predicate))
		
		(not-char-ascii-numeric-16?
			(type predicate))
		
		(not-char-ascii-alphabetic?
			(type predicate))
		
		(not-char-ascii-upper-case?
			(type predicate))
		
		(not-char-ascii-lower-case?
			(type predicate))
		
		(not-char-ascii-alphabetic-or-numeric?
			(type predicate))
		
		(not-char-ascii-whitespace?
			(type predicate))
		
		(not-char-ascii-control?
			(type predicate))
		
		(not-char-ascii-punctuation?
			(type predicate))
		
		(not-char-ascii-graphic?
			(type predicate))
		
		(not-fs-metadata?
			(type predicate))
		
		(not-cache?
			(type predicate))
		
		
		
		
		;; ---- booleans
		
		
		(not*
			(type procedure))
		
		
		(and*
			(type procedure))
		
		(or*
			(type procedure)
			(alias ior*))
		
		(xor*
			(type procedure))
		
		
		(nand*
			(type procedure))
		
		(nor*
			(type procedure)
			(alias nior*))
		
		(nxor*
			(type procedure))
		
		
		
		
		;; ---- arithmetic
		
		
		(negative
			(type procedure))
		
		(abs
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs abs)))
		
		(signum
			(type procedure))
		
		(floor
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs floor)))
		
		(ceiling
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs ceiling)))
		
		(round
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs round)))
		
		(truncate
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs truncate)))
		
		(fractional
			(type procedure))
		
		(exact
			(category vs:r7rs vs:arithmetic)
			(type converter)
			(implements (r7rs exact)))
		
		(inexact
			(category vs:r7rs vs:arithmetic)
			(type converter)
			(implements (r7rs inexact)))
		
		(square
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs square)))
		
		(sqrt
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs sqrt)))
		
		(exact-integer-sqrt
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs exact-integer-sqrt)))
		
		(exp
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs exp)))
		
		(log
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs log)))
		
		(sin
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs sin)))
		
		(cos
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs cos)))
		
		(tan
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs tan)))
		
		(asin
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs asin)))
		
		(acos
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs acos)))
		
		(atan
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs atan)))
		
		(floor/
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs floor/)))
		
		(floor-quotient
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs floor-quotient)))
		
		(floor-remainder
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias modulo)
			(implements (r7rs floor-remainder)))
		
		(truncate/
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs truncate/)))
		
		(truncate-quotient
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias quotient)
			(implements (r7rs truncate-quotient)))
		
		(truncate-remainder
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias remainder)
			(implements (r7rs truncate-remainder)))
		
		(expt
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs expt)))
		
		(+
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs +)))
		
		(-
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs -)))
		
		(*
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs *)))
		
		(/
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs /)))
		
		(gcd
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs gcd)))
		
		(lcm
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs lcm)))
		
		(min
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs min)))
		
		(max
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(implements (r7rs max)))
		
		
		
		
		;; ---- bitwise
		
		
		(bitwise-not
			(type predicate))
		
		
		(bitwise-and
			(type predicate))
		
		(bitwise-or
			(type predicate)
			(alias bitwise-ior))
		
		(bitwise-xor
			(type predicate))
		
		
		(bitwise-nand
			(type predicate))
		
		(bitwise-nor
			(type predicate)
			(alias bitwise-nior))
		
		(bitwise-nxor
			(type predicate))
		
		
		(bitwise-shift-left
			(type predicate))
		
		(bitwise-shift-right
			(type predicate))
		
		(bitwise-rotate-left
			(type predicate))
		
		(bitwise-rotate-right
			(type predicate))
		
		
		
		
		;; ---- comparisons
		
		
		(equivalent-by-identity?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias eq?)
			(extends (r7rs eq?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		(equivalent-by-value-strict?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias eqv?)
			(extends (r7rs eqv?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		(equivalent-by-value-strict-recursive?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias equal?)
			(extends (r7rs equal?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		
		(equivalent-by-value-coerced?
			(type procedure))
		
		(equivalent-by-value-coerced-recursive?
			(type procedure))
		
		
		(generic<?
			(type procedure))
		
		(generic<=?
			(type procedure))
		
		(generic=?
			(type procedure))
		
		(generic>=?
			(type procedure))
		
		(generic>?
			(type procedure))
		
		
		(boolean<?
			(type procedure))
		
		(boolean<=?
			(type procedure))
		
		(boolean=?
			(category vs:r7rs vs:booleans vs:comparisons vs:equivalence)
			(type comparator=)
			(extends (r7rs boolean=?))
			(signature
				((boolean) -> true)
				((boolean 2...) -> boolean))
		)
		
		(boolean>=?
			(type procedure))
		
		(boolean>?
			(type procedure))
		
		
		(<
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<)
			(extends (r7rs <))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(<=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<=)
			(extends (r7rs <=))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(=
			(category vs:r7rs vs:arithmetic vs:comparisons vs:equivalence)
			(type comparator=)
			(extends (r7rs =))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(>=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>=)
			(extends (r7rs >=))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(>
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>)
			(extends (r7rs >))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		
		(char<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<)
			(implements (r7rs char<?)))
		
		(char<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=)
			(implements (r7rs char<=?)))
		
		(char=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=)
			(implements (r7rs char=?)))
		
		(char>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=)
			(implements (r7rs char>=?)))
		
		(char>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>)
			(implements (r7rs char>?)))
		
		
		(char-ci<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<)
			(implements (r7rs char-ci<?)))
		
		(char-ci<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=)
			(implements (r7rs char-ci<=?)))
		
		(char-ci=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=)
			(implements (r7rs char-ci=?)))
		
		(char-ci>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=)
			(implements (r7rs char-ci>=?)))
		
		(char-ci>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>)
			(implements (r7rs char-ci>?)))
		
		
		(string<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<)
			(implements (r7rs string<?)))
		
		(string<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=)
			(implements (r7rs string<=?)))
		
		(string=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=)
			(implements (r7rs string=?)))
		
		(string>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=)
			(implements (r7rs string>=?)))
		
		(string>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>)
			(implements (r7rs string>?)))
		
		
		(string-ci<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<)
			(implements (r7rs string-ci<?)))
		
		(string-ci<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=)
			(implements (r7rs string-ci<=?)))
		
		(string-ci=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=)
			(implements (r7rs string-ci=?)))
		
		(string-ci>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=)
			(implements (r7rs string-ci>=?)))
		
		(string-ci>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>)
			(implements (r7rs string-ci>?)))
		
		
		(symbol<?
			(type procedure))
		
		(symbol<=?
			(type procedure))
		
		(symbol=?
			(category vs:r7rs vs:symbols vs:comparisons vs:equivalence)
			(type comparator=)
			(extends (r7rs symbol=?))
			(signature
				((symbol) -> true)
				((symbol 2...) -> boolean))
		)
		
		(symbol>=?
			(type procedure))
		
		(symbol>?
			(type procedure))
		
		
		(symbol-ci<?
			(type procedure))
		
		(symbol-ci<=?
			(type procedure))
		
		(symbol-ci=?
			(type procedure))
		
		(symbol-ci>=?
			(type procedure))
		
		(symbol-ci>?
			(type procedure))
		
		
		(keyword<?
			(type procedure))
		
		(keyword<=?
			(type procedure))
		
		(keyword=?
			(type procedure))
		
		(keyword>=?
			(type procedure))
		
		(keyword>?
			(type procedure))
		
		
		(keyword-ci<?
			(type procedure))
		
		(keyword-ci<=?
			(type procedure))
		
		(keyword-ci=?
			(type procedure))
		
		(keyword-ci>=?
			(type procedure))
		
		(keyword-ci>?
			(type procedure))
		
		
		(unique<?
			(type procedure))
		
		(unique<=?
			(type procedure))
		
		(unique=?
			(type procedure))
		
		(unique>=?
			(type procedure))
		
		(unique>?
			(type procedure))
		
		
		(bytevector<?
			(type procedure))
		
		(bytevector<=?
			(type procedure))
		
		(bytevector=?
			(type procedure))
		
		(bytevector>=?
			(type procedure))
		
		(bytevector>?
			(type procedure))
		
		
		(pair<?
			(type procedure))
		
		(pair<=?
			(type procedure))
		
		(pair=?
			(type procedure))
		
		(pair>=?
			(type procedure))
		
		(pair>?
			(type procedure))
		
		
		(vector<?
			(type procedure))
		
		(vector<=?
			(type procedure))
		
		(vector=?
			(type procedure))
		
		(vector>=?
			(type procedure))
		
		(vector>?
			(type procedure))
		
		
		(values<?
			(type procedure))
		
		(values<=?
			(type procedure))
		
		(values=?
			(type procedure))
		
		(values>=?
			(type procedure))
		
		(values>?
			(type procedure))
		
		
		(record<?
			(type procedure))
		
		(record<=?
			(type procedure))
		
		(record=?
			(type procedure))
		
		(record>=?
			(type procedure))
		
		(record>?
			(type procedure))
		
		
		(path<?
			(type procedure))
		
		(path<=?
			(type procedure))
		
		(path=?
			(type procedure))
		
		(path>=?
			(type procedure))
		
		(path>?
			(type procedure))
		
		
		
		
		;; ---- comparisons
		
		
		(not-equivalent-by-identity?
			(type procedure)
			(alias not-eq?))
		
		(not-equivalent-by-value-strict?
			(type procedure)
			(alias not-eqv?))
		
		(not-equivalent-by-value-strict-recursive?
			(type procedure)
			(alias not-equal?))
		
		
		(not-equivalent-by-value-coerced?
			(type procedure))
		
		(not-equivalent-by-value-coerced-recursive?
			(type procedure))
		
		
		(not-generic<?
			(type procedure))
		
		(not-generic<=?
			(type procedure))
		
		(not-generic=?
			(type procedure))
		
		(not-generic>=?
			(type procedure))
		
		(not-generic>?
			(type procedure))
		
		
		(not-boolean<?
			(type procedure))
		
		(not-boolean<=?
			(type procedure))
		
		(not-boolean=?
			(type procedure))
		
		(not-boolean>=?
			(type procedure))
		
		(not-boolean>?
			(type procedure))
		
		
		(not-<?
			(type procedure))
		
		(not-<=?
			(type procedure))
		
		(not-=?
			(type procedure))
		
		(not->=?
			(type procedure))
		
		(not->?
			(type procedure))
		
		
		(not-char<?
			(type procedure))
		
		(not-char<=?
			(type procedure))
		
		(not-char=?
			(type procedure))
		
		(not-char>=?
			(type procedure))
		
		(not-char>?
			(type procedure))
		
		
		(not-char-ci<?
			(type procedure))
		
		(not-char-ci<=?
			(type procedure))
		
		(not-char-ci=?
			(type procedure))
		
		(not-char-ci>=?
			(type procedure))
		
		(not-char-ci>?
			(type procedure))
		
		
		(not-string<?
			(type procedure))
		
		(not-string<=?
			(type procedure))
		
		(not-string=?
			(type procedure))
		
		(not-string>=?
			(type procedure))
		
		(not-string>?
			(type procedure))
		
		
		(not-string-ci<?
			(type procedure))
		
		(not-string-ci<=?
			(type procedure))
		
		(not-string-ci=?
			(type procedure))
		
		(not-string-ci>=?
			(type procedure))
		
		(not-string-ci>?
			(type procedure))
		
		
		(not-symbol<?
			(type procedure))
		
		(not-symbol<=?
			(type procedure))
		
		(not-symbol=?
			(type procedure))
		
		(not-symbol>=?
			(type procedure))
		
		(not-symbol>?
			(type procedure))
		
		
		(not-symbol-ci<?
			(type procedure))
		
		(not-symbol-ci<=?
			(type procedure))
		
		(not-symbol-ci=?
			(type procedure))
		
		(not-symbol-ci>=?
			(type procedure))
		
		(not-symbol-ci>?
			(type procedure))
		
		
		(not-keyword<?
			(type procedure))
		
		(not-keyword<=?
			(type procedure))
		
		(not-keyword=?
			(type procedure))
		
		(not-keyword>=?
			(type procedure))
		
		(not-keyword>?
			(type procedure))
		
		
		(not-keyword-ci<?
			(type procedure))
		
		(not-keyword-ci<=?
			(type procedure))
		
		(not-keyword-ci=?
			(type procedure))
		
		(not-keyword-ci>=?
			(type procedure))
		
		(not-keyword-ci>?
			(type procedure))
		
		
		(not-unique<?
			(type procedure))
		
		(not-unique<=?
			(type procedure))
		
		(not-unique=?
			(type procedure))
		
		(not-unique>=?
			(type procedure))
		
		(not-unique>?
			(type procedure))
		
		
		(not-bytevector<?
			(type procedure))
		
		(not-bytevector<=?
			(type procedure))
		
		(not-bytevector=?
			(type procedure))
		
		(not-bytevector>=?
			(type procedure))
		
		(not-bytevector>?
			(type procedure))
		
		
		(not-pair<?
			(type procedure))
		
		(not-pair<=?
			(type procedure))
		
		(not-pair=?
			(type procedure))
		
		(not-pair>=?
			(type procedure))
		
		(not-pair>?
			(type procedure))
		
		
		(not-vector<?
			(type procedure))
		
		(not-vector<=?
			(type procedure))
		
		(not-vector=?
			(type procedure))
		
		(not-vector>=?
			(type procedure))
		
		(not-vector>?
			(type procedure))
		
		
		(not-values<?
			(type procedure))
		
		(not-values<=?
			(type procedure))
		
		(not-values=?
			(type procedure))
		
		(not-values>=?
			(type procedure))
		
		(not-values>?
			(type procedure))
		
		
		(not-record<?
			(type procedure))
		
		(not-record<=?
			(type procedure))
		
		(not-record=?
			(type procedure))
		
		(not-record>=?
			(type procedure))
		
		(not-record>?
			(type procedure))
		
		
		(not-path<?
			(type procedure))
		
		(not-path<=?
			(type procedure))
		
		(not-path=?
			(type procedure))
		
		(not-path>=?
			(type procedure))
		
		(not-path>?
			(type procedure))
		
		
		
		
		;; ---- lists
		
		
		(car
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs car)))
		
		(cdr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdr)))
		
		(caar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caar)))
		
		(cdar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdar)))
		
		
		(first-pair
			(type procedure))
		
		(second-pair
			(type procedure))
		
		(third-pair
			(type procedure))
		
		(fourth-pair
			(type procedure))
		
		(fifth-pair
			(type procedure))
		
		(sixth-pair
			(type procedure))
		
		(seventh-pair
			(type procedure))
		
		(eighth-pair
			(type procedure))
		
		(ninth-pair
			(type procedure))
		
		(tenth-pair
			(type procedure))
		
		
		(first
			(type procedure))
		
		(second
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(alias cadr)
			(implements (r7rs cadr)))
		
		(third
			(type procedure))
		
		(fourth
			(type procedure))
		
		(fifth
			(type procedure))
		
		(sixth
			(type procedure))
		
		(seventh
			(type procedure))
		
		(eighth
			(type procedure))
		
		(ninth
			(type procedure))
		
		(tenth
			(type procedure))
		
		
		(first-tail
			(type procedure))
		
		(second-tail
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(alias cddr)
			(implements (r7rs cddr)))
		
		(third-tail
			(type procedure))
		
		(fourth-tail
			(type procedure))
		
		(fifth-tail
			(type procedure))
		
		(sixth-tail
			(type procedure))
		
		(seventh-tail
			(type procedure))
		
		(eighth-tail
			(type procedure))
		
		(ninth-tail
			(type procedure))
		
		(tenth-tail
			(type procedure))
		
		
		(length
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs length)))
		
		(reverse
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs reverse)))
		
		(pair->immutable
			(type procedure))
		
		(pair->mutable
			(type procedure))
		
		(list->immutable
			(type procedure))
		
		(list->mutable
			(type procedure))
		
		
		(cons
			(category vs:r7rs vs:pairs vs:lists)
			(type constructor)
			(implements (r7rs cons)))
		
		(xcons
			(type procedure))
		
		(set-car!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!)
			(extends (r7rs set-car!))
			(signature
				((pair any) -> any))
		)
		
		(set-cdr!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!)
			(extends (r7rs set-cdr!))
			(signature
				((pair any) -> any))
		)
		
		(list-ref-cons
			(category vs:r7rs vs:lists)
			(type procedure)
			(alias list-tail)
			(implements (r7rs list-tail)))
		
		(list-ref-car
			(category vs:r7rs vs:lists)
			(type accessor)
			(alias list-ref)
			(implements (r7rs list-ref)))
		
		(list-ref-cdr
			(type procedure))
		
		
		(memq
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs memq)))
		
		(memv
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs memv)))
		
		(assq
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(implements (r7rs assq)))
		
		(assv
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(implements (r7rs assv)))
		
		(find
			(type procedure))
		
		(list-set-car!
			(category vs:r7rs vs:lists)
			(type procedure)
			(alias list-set!)
			(extends (r7rs list-set!))
			(signature
				((list-not-null range-offset any) -> any))
		)
		
		(list-set-cdr!
			(type procedure))
		
		
		(make-pair
			(type procedure))
		
		(make-list
			(category vs:r7rs vs:lists)
			(type constructor)
			(implements (r7rs make-list)))
		
		(list
			(category vs:r7rs vs:lists)
			(type constructor)
			(implements (r7rs list)))
		
		(list*
			(type procedure))
		
		(append
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs append)))
		
		(list-fill!
			(type procedure))
		
		(list-copy!
			(type procedure))
		
		(list-copy
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs list-copy)))
		
		(list-reverse!
			(type procedure))
		
		(member
			(category vs:r7rs vs:lists)
			(type procedure)
			(implements (r7rs member)))
		
		(assoc
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(implements (r7rs assoc)))
		
		
		
		
		;; ---- arrays
		
		
		(vector-length
			(category vs:r7rs vs:vectors)
			(type procedure)
			(implements (r7rs vector-length)))
		
		(vector-reverse
			(type procedure))
		
		(vector->immutable
			(type procedure))
		
		(vector->mutable
			(type procedure))
		
		(vector-clear!
			(type procedure))
		
		(vector-ref
			(category vs:r7rs vs:vectors)
			(type accessor)
			(implements (r7rs vector-ref)))
		
		(vector-push-from!
			(type procedure))
		
		(vector-find
			(type procedure))
		
		(vector-set!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(extends (r7rs vector-set!))
			(signature
				((vector-not-empty range-offset any) -> any))
		)
		
		(vector-insert-from!
			(type procedure))
		
		(vector-swap!
			(type procedure))
		
		(vector-clear-at!
			(type procedure))
		
		(make-vector
			(category vs:r7rs vs:vectors)
			(type constructor)
			(implements (r7rs make-vector)))
		
		(vector
			(category vs:r7rs vs:vectors)
			(type constructor)
			(implements (r7rs vector)))
		
		(vector-append
			(category vs:r7rs vs:vectors)
			(type procedure)
			(implements (r7rs vector-append)))
		
		(vector-fill!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(implements (r7rs vector-fill!)))
		
		(vector-copy!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(implements (r7rs vector-copy!)))
		
		(vector-append!
			(type procedure))
		
		(vector-copy
			(category vs:r7rs vs:vectors)
			(type accessor)
			(implements (r7rs vector-copy)))
		
		(vector-reverse!
			(type procedure))
		
		(vector->list
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter)
			(implements (r7rs vector->list)))
		
		(list->vector
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter)
			(implements (r7rs list->vector)))
		
		(vector-push!
			(type procedure))
		
		(vector-pop!
			(type procedure))
		
		(vector-insert!
			(type procedure))
		
		(vector-remove!
			(type procedure))
		
		(vector-resize!
			(type procedure))
		
		(vector-resize-at!
			(type procedure))
		
		
		
		
		;; ---- bytes
		
		
		(bytevector-length
			(category vs:r7rs vs:bytes)
			(type procedure)
			(implements (r7rs bytevector-length)))
		
		(bytevector-reverse
			(type procedure))
		
		(bytevector->immutable
			(type procedure))
		
		(bytevector->mutable
			(type procedure))
		
		(bytevector-u8-ref
			(category vs:r7rs vs:bytes)
			(type accessor)
			(implements (r7rs bytevector-u8-ref)))
		
		(bytevector-u8-set!
			(category vs:r7rs vs:bytes)
			(type mutator!)
			(extends (r7rs bytevector-u8-set!))
			(signature
				((bytevector-not-empty range-offset byte) -> byte))
		)
		
		(make-bytevector
			(category vs:r7rs vs:bytes)
			(type constructor)
			(implements (r7rs make-bytevector)))
		
		(bytevector
			(category vs:r7rs vs:bytes)
			(type constructor)
			(implements (r7rs bytevector)))
		
		(bytevector-append
			(category vs:r7rs vs:bytes)
			(type procedure)
			(implements (r7rs bytevector-append)))
		
		(bytevector-u8-fill!
			(type procedure))
		
		(bytevector-copy!
			(category vs:r7rs vs:bytes)
			(type procedure!)
			(implements (r7rs bytevector-copy!)))
		
		(bytevector-copy
			(category vs:r7rs vs:bytes)
			(type procedure)
			(implements (r7rs bytevector-copy)))
		
		(bytevector-reverse!
			(type procedure))
		
		(bytevector->list
			(type procedure))
		
		(list->bytevector
			(type procedure))
		
		(bytevector->vector
			(type procedure))
		
		(vector->bytevector
			(type procedure))
		
		
		
		
		;; ---- bytes regular-expressions
		
		
		(make-bytevector-regex
			(type procedure))
		
		(bytevector-regex-match?
			(type procedure))
		
		(bytevector-regex-match
			(type procedure))
		
		(bytevector-regex-match-all
			(type procedure))
		
		(bytevector-regex-match-all->vector
			(type procedure))
		
		(bytevector-regex-match-position
			(type procedure))
		
		(bytevector-regex-match-position-all
			(type procedure))
		
		(bytevector-regex-match-position-all->vector
			(type procedure))
		
		(bytevector-regex-match-captures
			(type procedure))
		
		(bytevector-regex-match-captures->assoc
			(type procedure))
		
		(bytevector-regex-match-captures->vector
			(type procedure))
		
		(bytevector-regex-match-captures-all
			(type procedure))
		
		(bytevector-regex-match-captures-all->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-all->vector
			(type procedure))
		
		(bytevector-regex-match-captures-position
			(type procedure))
		
		(bytevector-regex-match-captures-position->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-position->vector
			(type procedure))
		
		(bytevector-regex-match-captures-position-all
			(type procedure))
		
		(bytevector-regex-match-captures-position-all->assoc
			(type procedure))
		
		(bytevector-regex-match-captures-position-all->vector
			(type procedure))
		
		
		
		
		;; ---- string
		
		
		(string-length
			(category vs:r7rs vs:strings)
			(type procedure)
			(implements (r7rs string-length)))
		
		(string-reverse
			(type procedure))
		
		(string->symbol
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter)
			(implements (r7rs string->symbol)))
		
		(symbol->string
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter)
			(implements (r7rs symbol->string)))
		
		(char->integer
			(category vs:r7rs vs:characters)
			(type converter)
			(implements (r7rs char->integer)))
		
		(integer->char
			(category vs:r7rs vs:characters)
			(type converter)
			(implements (r7rs integer->char)))
		
		(string->keyword
			(type procedure))
		
		(keyword->string
			(type procedure))
		
		(symbol->keyword
			(type procedure))
		
		(keyword->symbol
			(type procedure))
		
		(string-upcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(implements (r7rs string-upcase)))
		
		(string-downcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(implements (r7rs string-downcase)))
		
		(string-foldcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(implements (r7rs string-foldcase)))
		
		(symbol-upcase
			(type procedure))
		
		(symbol-downcase
			(type procedure))
		
		(symbol-foldcase
			(type procedure))
		
		(char-upcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(implements (r7rs char-upcase)))
		
		(char-downcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(implements (r7rs char-downcase)))
		
		(char-foldcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(implements (r7rs char-foldcase)))
		
		(keyword-upcase
			(type procedure))
		
		(keyword-downcase
			(type procedure))
		
		(keyword-foldcase
			(type procedure))
		
		(string->immutable
			(type procedure))
		
		(string->mutable
			(type procedure))
		
		(string-ref
			(category vs:r7rs vs:strings)
			(type accessor)
			(implements (r7rs string-ref)))
		
		(string-set!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(extends (r7rs string-set!))
			(signature
				((string-not-empty range-offset character) -> character))
		)
		
		(make-string
			(category vs:r7rs vs:strings)
			(type constructor)
			(implements (r7rs make-string)))
		
		(string
			(category vs:r7rs vs:strings)
			(type constructor)
			(implements (r7rs string)))
		
		(string-append
			(category vs:r7rs vs:strings)
			(type constructor)
			(implements (r7rs string-append)))
		
		(string-fill!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(implements (r7rs string-fill!)))
		
		(string-copy!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(implements (r7rs string-copy!)))
		
		(string-copy
			(category vs:r7rs vs:strings)
			(type accessor)
			(alias substring)
			(implements (r7rs string-copy) (r7rs substring)))
		
		(string-reverse!
			(type procedure))
		
		(string->list
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter)
			(implements (r7rs string->list)))
		
		(list->string
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter)
			(implements (r7rs list->string)))
		
		(string->vector
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter)
			(implements (r7rs string->vector)))
		
		(vector->string
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter)
			(implements (r7rs vector->string)))
		
		(string->utf8
			(category vs:r7rs vs:bytes vs:strings)
			(type converter)
			(implements (r7rs string->utf8)))
		
		(utf8->string
			(category vs:r7rs vs:bytes vs:strings)
			(type converter)
			(implements (r7rs utf8->string)))
		
		(string->number
			(category vs:r7rs vs:strings vs:conversions)
			(type converter)
			(implements (r7rs string->number)))
		
		(number->string
			(category vs:r7rs vs:strings vs:conversions)
			(type converter)
			(implements (r7rs number->string)))
		
		(digit-value
			(category vs:r7rs vs:characters)
			(type converter)
			(implements (r7rs digit-value)))
		
		
		
		
		;; ---- string regular-expressions
		
		
		(make-string-regex
			(type procedure))
		
		(string-regex-match?
			(type procedure))
		
		(string-regex-match
			(type procedure))
		
		(string-regex-match-all
			(type procedure))
		
		(string-regex-match-all->vector
			(type procedure))
		
		(string-regex-match-position
			(type procedure))
		
		(string-regex-match-position-all
			(type procedure))
		
		(string-regex-match-position-all->vector
			(type procedure))
		
		(string-regex-match-captures
			(type procedure))
		
		(string-regex-match-captures->assoc
			(type procedure))
		
		(string-regex-match-captures->vector
			(type procedure))
		
		(string-regex-match-captures-all
			(type procedure))
		
		(string-regex-match-captures-all->assoc
			(type procedure))
		
		(string-regex-match-captures-all->vector
			(type procedure))
		
		(string-regex-match-captures-position
			(type procedure))
		
		(string-regex-match-captures-position->assoc
			(type procedure))
		
		(string-regex-match-captures-position->vector
			(type procedure))
		
		(string-regex-match-captures-position-all
			(type procedure))
		
		(string-regex-match-captures-position-all->assoc
			(type procedure))
		
		(string-regex-match-captures-position-all->vector
			(type procedure))
		
		
		
		
		;; ---- functional
		
		
		(identity
			(type procedure))
		
		(constant-fn
			(type procedure))
		
		(constant-fn*
			(type procedure))
		
		(not-fn
			(type procedure))
		
		(call-with-list*
			(type procedure))
		
		(call-with-list
			(type procedure))
		
		(call-with-vector*
			(type procedure))
		
		(call-with-vector
			(type procedure))
		
		(call-with-values*
			(type procedure))
		
		(call-with-values
			(category vs:r7rs vs:functions vs:values)
			(type procedure)
			(implements (r7rs call-with-values)))
		
		(map-in-order
			(type procedure))
		
		(call
			(type procedure))
		
		(apply
			(category vs:r7rs vs:functions)
			(type procedure)
			(implements (r7rs apply)))
		
		(map
			(category vs:r7rs vs:lists vs:functions vs:conversions vs:loops)
			(type map)
			(implements (r7rs map)))
		
		(for-each
			(category vs:r7rs vs:lists vs:functions vs:loops)
			(type for-each)
			(extends (r7rs for-each))
			(signature
				((for-each-procedure list 1...) -> void))
		)
		
		(vector-map
			(category vs:r7rs vs:vectors vs:functions vs:conversions vs:loops)
			(type map)
			(implements (r7rs vector-map)))
		
		(vector-for-each
			(category vs:r7rs vs:vectors vs:functions vs:loops)
			(type for-each)
			(extends (r7rs vector-for-each))
			(signature
				((for-each-procedure vector 1...) -> void))
		)
		
		(bytevector-u8-map
			(type map))
		
		(bytevector-u8-for-each
			(type for-each))
		
		(string-map
			(category vs:r7rs vs:strings vs:functions vs:conversions vs:loops)
			(type map)
			(implements (r7rs string-map)))
		
		(string-for-each
			(category vs:r7rs vs:strings vs:functions vs:loops)
			(type for-each)
			(extends (r7rs string-for-each))
			(signature
				((for-each-procedure string 1...) -> void))
		)
		
		(values
			(category vs:r7rs vs:functions vs:values)
			(type constructor)
			(implements (r7rs values)))
		
		(curry
			(type procedure))
		
		(curry-last
			(type procedure))
		
		(compose
			(type procedure))
		
		(compose*
			(type procedure))
		
		
		
		
		;; ---- records
		
		
		(record-type-identifier
			(type procedure))
		
		(record-type-size
			(type procedure))
		
		(record-type
			(type procedure))
		
		(record->immutable
			(type procedure))
		
		(record->mutable
			(type procedure))
		
		(make-record-type
			(type procedure))
		
		(record-type-predicate
			(type procedure))
		
		(record-type-accessor
			(type procedure))
		
		(record-type-mutator
			(type procedure))
		
		(record-type-constructor
			(type procedure))
		
		(record-type-constructor*
			(type procedure))
		
		(record-of?
			(type procedure))
		
		(record-set!
			(type procedure))
		
		(record-ref
			(type procedure))
		
		(make-record
			(type procedure))
		
		(make-record*
			(type procedure))
		
		(record->vector
			(type procedure))
		
		(vector->record
			(type procedure))
		
		(record->values
			(type procedure))
		
		(values->record
			(type procedure))
		
		(record->list
			(type procedure))
		
		(list->record
			(type procedure))
		
		
		
		
		;; ---- runtime
		
		
		(command-line
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs command-line)))
		
		(command-line->vector
			(type procedure))
		
		(command-line-length
			(type procedure))
		
		(get-environment-variables
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs get-environment-variables)))
		
		(get-environment-variables->vector
			(type procedure))
		
		(get-environment-fingerprint
			(type procedure))
		
		
		(current-second
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs current-second)))
		
		(current-jiffy
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs current-jiffy)))
		
		(jiffies-per-second
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs jiffies-per-second)))
		
		
		(raise
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure)
			(implements (r7rs raise)))
		
		(error-object-message
			(category vs:r7rs vs:errors)
			(type accessor)
			(implements (r7rs error-object-message)))
		
		(error-object-irritants
			(category vs:r7rs vs:errors)
			(type accessor)
			(implements (r7rs error-object-irritants)))
		
		(error-object-irritants->vector
			(type procedure))
		
		(error-object-irritants->values
			(type procedure))
		
		(command-line-ref
			(type procedure))
		
		(get-environment-variable
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs get-environment-variable)))
		
		(process-wait-poll
			(type procedure))
		
		(process-wait-try
			(type procedure))
		
		(process-wait
			(type procedure))
		
		(process-stdin
			(type procedure))
		
		(process-stdout
			(type procedure))
		
		(process-stderr
			(type procedure))
		
		(cache-close
			(type procedure))
		
		(serialize-bytevector
			(type procedure))
		
		(deserialize-bytevector
			(type procedure))
		
		(hash
			(type procedure))
		
		(hash-sip-unseeded
			(type procedure))
		
		(hash-sea-unseeded
			(type procedure))
		
		(hash-blake2b-unseeded
			(type procedure))
		
		(hash-blake2s-unseeded
			(type procedure))
		
		(process-spawn
			(type procedure))
		
		(process-exec
			(type procedure))
		
		(process-run-try
			(type procedure))
		
		(process-run
			(type procedure))
		
		(error
			(category vs:r7rs vs:errors)
			(type constructor)
			(implements (r7rs error)))
		
		(make-error
			(type procedure))
		
		(make-parameter
			(category vs:r7rs vs:parameters)
			(type constructor)
			(implements (r7rs make-parameter)))
		
		(parameter-ref
			(type procedure))
		
		(parameter-set!
			(type procedure))
		
		(trace-critical
			(type procedure))
		
		(trace-error
			(type procedure))
		
		(trace-warning
			(type procedure))
		
		(trace-notice
			(type procedure))
		
		(trace-information
			(type procedure))
		
		(trace-internal
			(type procedure))
		
		(trace-debugging
			(type procedure))
		
		(abort
			(type procedure))
		
		(pause
			(type procedure))
		
		(exit
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs exit)))
		
		(emergency-exit
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs emergency-exit)))
		
		(process-spawn*
			(type procedure))
		
		(process-exec*
			(type procedure))
		
		(cache-open
			(type procedure))
		
		(cache-select-bytevector
			(type procedure))
		
		(cache-include-bytevector
			(type procedure))
		
		(cache-exclude-bytevector
			(type procedure))
		
		(cache-resolve-bytevector
			(type procedure))
		
		(cache-select
			(type procedure))
		
		(cache-include
			(type procedure))
		
		(cache-exclude
			(type procedure))
		
		(cache-resolve
			(type procedure))
		
		(cache-exclude-all
			(type procedure))
		
		(cache-prune-all
			(type procedure))
		
		(hash-sip-seeded
			(type procedure))
		
		(hash-sea-seeded
			(type procedure))
		
		(hash-blake2b-seeded
			(type procedure))
		
		(hash-blake2s-seeded
			(type procedure))
		
		
		
		
		;; ---- ports
		
		
		(current-input-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(implements (r7rs current-input-port)))
		
		(current-output-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(implements (r7rs current-output-port)))
		
		(current-error-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(implements (r7rs current-error-port)))
		
		(eof-object
			(category vs:r7rs vs:ports vs:globals)
			(type constructor)
			(implements (r7rs eof-object)))
		
		(open-input-bytevector
			(category vs:r7rs vs:ports:input vs:ports:open vs:bytes)
			(type procedure)
			(implements (r7rs open-input-bytevector)))
		
		(open-input-string
			(category vs:r7rs vs:ports:input vs:ports:open vs:strings)
			(type procedure)
			(implements (r7rs open-input-string)))
		
		(get-output-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(implements (r7rs get-output-bytevector)))
		
		(get-output-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure)
			(implements (r7rs get-output-string)))
		
		(open-binary-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure)
			(implements (r7rs open-binary-input-file)))
		
		(open-binary-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure)
			(implements (r7rs open-binary-output-file)))
		
		(open-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure)
			(implements (r7rs open-input-file)))
		
		(open-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure)
			(implements (r7rs open-output-file)))
		
		(port-descriptor
			(type procedure))
		
		(port-descriptor-clone
			(type procedure))
		
		(port-descriptor-ref
			(type procedure))
		
		(port-descriptor-path
			(type procedure))
		
		(port-temporary-release
			(type procedure))
		
		(port-temporary-path
			(type procedure))
		
		(call-with-port
			(category vs:r7rs vs:ports vs:functions)
			(type procedure)
			(implements (r7rs call-with-port)))
		
		(call-with-binary-input-file
			(type procedure))
		
		(call-with-binary-output-file
			(type procedure))
		
		(call-with-input-file
			(category vs:r7rs vs:ports:input vs:functions)
			(type procedure)
			(implements (r7rs call-with-input-file)))
		
		(call-with-output-file
			(category vs:r7rs vs:ports:output vs:functions)
			(type procedure)
			(implements (r7rs call-with-output-file)))
		
		(with-binary-input-file
			(type procedure))
		
		(with-binary-output-file
			(type procedure))
		
		(with-input-from-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure)
			(implements (r7rs with-input-from-file)))
		
		(with-output-to-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure)
			(implements (r7rs with-output-to-file)))
		
		(port-descriptor-flag-ref
			(type procedure))
		
		(port-descriptor-flag-set!
			(type procedure))
		
		(open-output-bytevector
			(category vs:r7rs vs:ports:output vs:ports:open vs:bytes)
			(type procedure)
			(implements (r7rs open-output-bytevector)))
		
		(open-output-string
			(category vs:r7rs vs:ports:output vs:ports:open vs:strings)
			(type procedure)
			(implements (r7rs open-output-string)))
		
		(input-port-open?
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type predicate)
			(extends (r7rs input-port-open?))
			(signature
				((input-port-open 1...) -> true)
				((input-port 1...) -> false)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(output-port-open?
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type predicate)
			(extends (r7rs output-port-open?))
			(signature
				((output-port-open 1...) -> true)
				((output-port 1...) -> false)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(close-port
			(category vs:r7rs vs:ports)
			(type procedure)
			(implements (r7rs close-port)))
		
		(close-input-port
			(category vs:r7rs vs:ports:input)
			(type procedure)
			(implements (r7rs close-input-port)))
		
		(close-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure)
			(implements (r7rs close-output-port)))
		
		(u8-ready?
			(category vs:r7rs vs:ports:input vs:bytes)
			(type predicate)
			(implements (r7rs u8-ready?)))
		
		(peek-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(implements (r7rs peek-u8)))
		
		(read-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(implements (r7rs read-u8)))
		
		(char-ready?
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type predicate)
			(implements (r7rs char-ready?)))
		
		(peek-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure)
			(implements (r7rs peek-char)))
		
		(read-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure)
			(implements (r7rs read-char)))
		
		(read-bytevector!
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure!)
			(implements (r7rs read-bytevector!)))
		
		(read-bytevector-append!
			(type procedure))
		
		(read-bytevector
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(implements (r7rs read-bytevector)))
		
		(read-bytevector-chunk
			(type procedure))
		
		(read-bytevector-line
			(type procedure))
		
		(read-bytevector-zero
			(type procedure))
		
		(read-string-append!
			(type procedure))
		
		(read-string
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure)
			(implements (r7rs read-string)))
		
		(read-string-chunk
			(type procedure))
		
		(read-string-line
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure)
			(alias read-line)
			(implements (r7rs read-line)))
		
		(read-string-zero
			(type procedure))
		
		(read
			(category vs:r7rs vs:ports:input vs:ports:values)
			(type procedure)
			(implements (r7rs read)))
		
		(read-bytevector-fold
			(type procedure))
		
		(read-bytevector-chunk-fold
			(type procedure))
		
		(read-bytevector-line-fold
			(type procedure))
		
		(read-bytevector-zero-fold
			(type procedure))
		
		(read-string-fold
			(type procedure))
		
		(read-string-chunk-fold
			(type procedure))
		
		(read-string-line-fold
			(type procedure))
		
		(read-string-zero-fold
			(type procedure))
		
		(read-fold
			(type procedure))
		
		(write-u8
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(extends (r7rs write-u8))
			(signature
				((byte) -> void)
				((byte binary-output-port-open) -> void))
		)
		
		(write-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(implements (r7rs write-bytevector)))
		
		(write-bytevector-line
			(type procedure))
		
		(write-bytevector-zero
			(type procedure))
		
		(write-char
			(category vs:r7rs vs:ports:output vs:strings vs:characters)
			(type procedure)
			(extends (r7rs write-char))
			(signature
				((character) -> void)
				((character textual-output-port-open) -> void))
		)
		
		(write-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure)
			(implements (r7rs write-string)))
		
		(write-string-line
			(type procedure))
		
		(write-string-zero
			(type procedure))
		
		(write
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(implements (r7rs write)))
		
		(write-shared
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(implements (r7rs write-shared)))
		
		(write-simple
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(implements (r7rs write-simple)))
		
		(display
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(implements (r7rs display)))
		
		(write-line
			(type procedure))
		
		(display-line
			(type procedure))
		
		(newline
			(category vs:r7rs vs:ports:output vs:bytes vs:strings)
			(type procedure)
			(implements (r7rs newline)))
		
		(flush-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure)
			(implements (r7rs flush-output-port)))
		
		(open-binary-temporary
			(type procedure))
		
		(open-temporary
			(type procedure))
		
		
		
		
		;; ---- file-system
		
		
		(file-exists?
			(category vs:r7rs vs:file-system)
			(type procedure)
			(implements (r7rs file-exists?)))
		
		(fs-file-delete
			(category vs:r7rs vs:file-system)
			(type procedure)
			(alias delete-file)
			(implements (r7rs delete-file)))
		
		
		
		
		;; ---- crypto
		
		
		(crypto-bytevector
			(type procedure))
		
		(crypto-bytevector-append!
			(type procedure))
		
		(crypto-bytevector-fill!
			(type procedure))
		
		(crypto-blake2b-128
			(type procedure))
		
		(crypto-blake2b-192
			(type procedure))
		
		(crypto-blake2b-224
			(type procedure))
		
		(crypto-blake2b-256
			(type procedure))
		
		(crypto-blake2b-320
			(type procedure))
		
		(crypto-blake2b-384
			(type procedure))
		
		(crypto-blake2b-448
			(type procedure))
		
		(crypto-blake2b-512
			(type procedure))
		
		(crypto-blake2b-64
			(type procedure))
		
		(crypto-blake2s-128
			(type procedure))
		
		(crypto-blake2s-192
			(type procedure))
		
		(crypto-blake2s-224
			(type procedure))
		
		(crypto-blake2s-256
			(type procedure))
		
		(crypto-blake2s-64
			(type procedure))
		
		(crypto-md5
			(type procedure))
		
		(crypto-sha1
			(type procedure))
		
		(crypto-sha2-256
			(type procedure))
		
		(crypto-sha2-256-224
			(type procedure)
			(alias crypto-sha2-224))
		
		(crypto-sha2-512
			(type procedure))
		
		(crypto-sha2-512-224
			(type procedure))
		
		(crypto-sha2-512-256
			(type procedure))
		
		(crypto-sha2-512-384
			(type procedure)
			(alias crypto-sha2-384))
		
		(crypto-sha3-224
			(type procedure))
		
		(crypto-sha3-256
			(type procedure))
		
		(crypto-sha3-384
			(type procedure))
		
		(crypto-sha3-512
			(type procedure))
		
		
		
		
		;; ---- encoding
		
		
		(base32-decode
			(type procedure))
		
		(base32-decode-append!
			(type procedure))
		
		(base32-decode-fill!
			(type procedure))
		
		(base32-hex-decode
			(type procedure))
		
		(base32-hex-decode-append!
			(type procedure))
		
		(base32-hex-decode-fill!
			(type procedure))
		
		(base32-hex-nopad-decode
			(type procedure))
		
		(base32-hex-nopad-decode-append!
			(type procedure))
		
		(base32-hex-nopad-decode-fill!
			(type procedure))
		
		(base32-nopad-decode
			(type procedure))
		
		(base32-nopad-decode-append!
			(type procedure))
		
		(base32-nopad-decode-fill!
			(type procedure))
		
		(base64-decode
			(type procedure))
		
		(base64-decode-append!
			(type procedure))
		
		(base64-decode-fill!
			(type procedure))
		
		(base64-mime-decode
			(type procedure))
		
		(base64-mime-decode-append!
			(type procedure))
		
		(base64-mime-decode-fill!
			(type procedure))
		
		(base64-nopad-decode
			(type procedure))
		
		(base64-nopad-decode-append!
			(type procedure))
		
		(base64-nopad-decode-fill!
			(type procedure))
		
		(base64-url-decode
			(type procedure))
		
		(base64-url-decode-append!
			(type procedure))
		
		(base64-url-decode-fill!
			(type procedure))
		
		(base64-url-nopad-decode
			(type procedure))
		
		(base64-url-nopad-decode-append!
			(type procedure))
		
		(base64-url-nopad-decode-fill!
			(type procedure))
		
		(hex-decode
			(type procedure))
		
		(hex-decode-append!
			(type procedure))
		
		(hex-decode-fill!
			(type procedure))
		
		(base32-encode
			(type procedure))
		
		(base32-encode-append!
			(type procedure))
		
		(base32-encode-fill!
			(type procedure))
		
		(base32-hex-encode
			(type procedure))
		
		(base32-hex-encode-append!
			(type procedure))
		
		(base32-hex-encode-fill!
			(type procedure))
		
		(base32-hex-nopad-encode
			(type procedure))
		
		(base32-hex-nopad-encode-append!
			(type procedure))
		
		(base32-hex-nopad-encode-fill!
			(type procedure))
		
		(base32-nopad-encode
			(type procedure))
		
		(base32-nopad-encode-append!
			(type procedure))
		
		(base32-nopad-encode-fill!
			(type procedure))
		
		(base64-encode
			(type procedure))
		
		(base64-encode-append!
			(type procedure))
		
		(base64-encode-fill!
			(type procedure))
		
		(base64-mime-encode
			(type procedure))
		
		(base64-mime-encode-append!
			(type procedure))
		
		(base64-mime-encode-fill!
			(type procedure))
		
		(base64-nopad-encode
			(type procedure))
		
		(base64-nopad-encode-append!
			(type procedure))
		
		(base64-nopad-encode-fill!
			(type procedure))
		
		(base64-url-encode
			(type procedure))
		
		(base64-url-encode-append!
			(type procedure))
		
		(base64-url-encode-fill!
			(type procedure))
		
		(base64-url-nopad-encode
			(type procedure))
		
		(base64-url-nopad-encode-append!
			(type procedure))
		
		(base64-url-nopad-encode-fill!
			(type procedure))
		
		(hex-lower-encode
			(type procedure)
			(alias hex-encode))
		
		(hex-lower-encode-append!
			(type procedure)
			(alias hex-encode-append!))
		
		(hex-lower-encode-fill!
			(type procedure)
			(alias hex-encode-fill!))
		
		(hex-upper-encode
			(type procedure))
		
		(hex-upper-encode-append!
			(type procedure))
		
		(hex-upper-encode-fill!
			(type procedure))
		
		
		
		
		;; ---- random
		
		
		(random-boolean
			(type procedure))
		
		(random-boolean-weighted
			(type procedure))
		
		(random-bytevector
			(type procedure))
		
		(random-bytevector-append!
			(type procedure))
		
		(random-bytevector-fill!
			(type procedure))
		
		(random-bytevector-permutation
			(type procedure))
		
		(random-bytevector-shuffle!
			(type procedure))
		
		(random-char
			(type procedure))
		
		(random-char*
			(type procedure))
		
		(random-char-ascii
			(type procedure))
		
		(random-char-ascii-alphabetic
			(type procedure))
		
		(random-char-ascii-lower-case
			(type procedure))
		
		(random-char-ascii-upper-case
			(type procedure))
		
		(random-char-ascii-numeric
			(type procedure))
		
		(random-char-ascii-numeric-8
			(type procedure))
		
		(random-char-ascii-numeric-16
			(type procedure))
		
		(random-char-ascii-alphabetic-or-numeric
			(type procedure))
		
		(random-char-ascii-control
			(type procedure))
		
		(random-char-ascii-graphic
			(type procedure))
		
		(random-char-ascii-punctuation
			(type procedure))
		
		(random-char-ascii-whitespace
			(type procedure))
		
		(random-string-ascii
			(type procedure))
		
		(random-string-ascii-alphabetic
			(type procedure))
		
		(random-string-ascii-lower-case
			(type procedure))
		
		(random-string-ascii-upper-case
			(type procedure))
		
		(random-string-ascii-numeric
			(type procedure))
		
		(random-string-ascii-numeric-8
			(type procedure))
		
		(random-string-ascii-numeric-16
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric
			(type procedure))
		
		(random-string-ascii-control
			(type procedure))
		
		(random-string-ascii-graphic
			(type procedure))
		
		(random-string-ascii-punctuation
			(type procedure))
		
		(random-string-ascii-whitespace
			(type procedure))
		
		(random-string-ascii-append!
			(type procedure))
		
		(random-string-ascii-alphabetic-append!
			(type procedure))
		
		(random-string-ascii-lower-case-append!
			(type procedure))
		
		(random-string-ascii-upper-case-append!
			(type procedure))
		
		(random-string-ascii-numeric-append!
			(type procedure))
		
		(random-string-ascii-numeric-8-append!
			(type procedure))
		
		(random-string-ascii-numeric-16-append!
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric-append!
			(type procedure))
		
		(random-string-ascii-control-append!
			(type procedure))
		
		(random-string-ascii-graphic-append!
			(type procedure))
		
		(random-string-ascii-punctuation-append!
			(type procedure))
		
		(random-string-ascii-whitespace-append!
			(type procedure))
		
		(random-string-ascii-permutation
			(type procedure))
		
		(random-string-ascii-alphabetic-permutation
			(type procedure))
		
		(random-string-ascii-lower-case-permutation
			(type procedure))
		
		(random-string-ascii-upper-case-permutation
			(type procedure))
		
		(random-string-ascii-numeric-permutation
			(type procedure))
		
		(random-string-ascii-numeric-8-permutation
			(type procedure))
		
		(random-string-ascii-numeric-16-permutation
			(type procedure))
		
		(random-string-ascii-alphabetic-or-numeric-permutation
			(type procedure))
		
		(random-string-ascii-control-permutation
			(type procedure))
		
		(random-string-ascii-graphic-permutation
			(type procedure))
		
		(random-string-ascii-punctuation-permutation
			(type procedure))
		
		(random-string-ascii-whitespace-permutation
			(type procedure))
		
		(random-f64
			(type procedure))
		
		(random-f64*
			(type procedure))
		
		(random-i8
			(type procedure))
		
		(random-i16
			(type procedure))
		
		(random-i32
			(type procedure))
		
		(random-i64
			(type procedure))
		
		(random-i64*
			(type procedure))
		
		(random-u1
			(type procedure))
		
		(random-u2
			(type procedure))
		
		(random-u3
			(type procedure))
		
		(random-u4
			(type procedure))
		
		(random-u5
			(type procedure))
		
		(random-u6
			(type procedure))
		
		(random-u63
			(type procedure))
		
		(random-u7
			(type procedure))
		
		(random-u8
			(type procedure))
		
		(random-u15
			(type procedure))
		
		(random-u16
			(type procedure))
		
		(random-u31
			(type procedure))
		
		(random-u32
			(type procedure))
		
		
		
		
		;; ---- extended
		
		
		(caaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caaar)))
		
		(caaaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caaaar)))
		
		(caaadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caaadr)))
		
		(caadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caadr)))
		
		(caadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caadar)))
		
		(caaddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caaddr)))
		
		(cadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cadar)))
		
		(cadaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cadaar)))
		
		(cadadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cadadr)))
		
		(caddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caddr)))
		
		(caddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs caddar)))
		
		(cadddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cadddr)))
		
		(cdaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdaar)))
		
		(cdaaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdaaar)))
		
		(cdaadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdaadr)))
		
		(cdadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdadr)))
		
		(cdadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdadar)))
		
		(cdaddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdaddr)))
		
		(cddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cddar)))
		
		(cddaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cddaar)))
		
		(cddadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cddadr)))
		
		(cdddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdddr)))
		
		(cdddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cdddar)))
		
		(cddddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(implements (r7rs cddddr)))
		
		
		
		
		;; ---- parameters
		
		
		(process-spawn:stdin
			(type parameter))
		
		(process-spawn:stdout
			(type parameter))
		
		(process-spawn:stderr
			(type parameter))
		
		(process-spawn:directory
			(type parameter))
		
		(process-spawn:env-empty
			(type parameter))
		
		(process-spawn:env-include
			(type parameter))
		
		(process-spawn:env-exclude
			(type parameter))
		
		
		
		
		;; ---- R7RS unimplemented syntaxes
		
		
		(delay
			(category vs:r7rs vs:promises vs:evaluator)
			(type syntax)
			(implements (r7rs delay)))
		
		(delay-force
			(category vs:r7rs vs:promises vs:evaluator)
			(type syntax)
			(implements (r7rs delay-force)))
		
		
		
		
		;; ---- R7RS unsupported syntaxes
		
		
		(case-lambda
			(category vs:r7rs vs:lambda)
			(type syntax)
			(implements (r7rs case-lambda)))
		
		(cond-expand
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(implements (r7rs cond-expand)))
		
		(define-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(implements (r7rs define-syntax)))
		
		(let-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(implements (r7rs let-syntax)))
		
		(letrec-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(implements (r7rs letrec-syntax)))
		
		(syntax-rules
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(implements (r7rs syntax-rules)))
		
		(syntax-error
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(implements (r7rs syntax-error)))
		
		(import
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(implements (r7rs import)))
		
		(include
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(implements (r7rs include)))
		
		(include-ci
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(implements (r7rs include-ci)))
		
		
		
		
		;; ---- R7RS unimplemented procedures
		
		
		(make-promise
			(category vs:r7rs vs:promises vs:evaluator)
			(type constructor)
			(implements (r7rs make-promise)))
		
		(force
			(category vs:r7rs vs:promises vs:evaluator)
			(type procedure)
			(implements (r7rs force)))
		
		(eval
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs eval)))
		
		(environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs environment)))
		
		(null-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs null-environment)))
		
		(interaction-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs interaction-environment)))
		
		(scheme-report-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs scheme-report-environment)))
		
		(load
			(category vs:r7rs vs:compiler vs:unsupported)
			(type procedure)
			(implements (r7rs load)))
		
		
		
		
		;; ---- miscellaneous unimplemented procedures
		
		;! +fn
		;! -fn
		;! -fn-last
		;! *fn
		;! /fn
		;! /fn-last
		;! <=fn
		;! <fn
		;! =fn
		;! >=fn
		;! >fn
		
		;! begin-fn
		
		;! or-fn
		;! and-fn
		;! xor-fn
		
		;! nand-fn
		;! nor-fn
		;! nxor-fn
		
		;! if-fn
		;! when-fn
		;! unless-fn
		;! cond-fn
		;! case-fn
		
		;! do-fn
		;! while-fn
		;! until-fn
		
		;! eq-fn
		;! equal-fn
		;! eqv-fn
		
		;! string-for-each-fn
		;! string-map-fn
		
		;! bytevector-u8-for-each-fn
		;! bytevector-u8-map-fn
		;! bytevector-u8-ref-fn
		;! bytevector-u8-set-fn
		
		;! vector-for-each-fn
		;! vector-map-fn
		;! vector-ref-fn
		;! vector-set-fn
		
		;! cons-car-fn
		;! cons-cdr-fn
		
		;! map-fn
		;! for-each-fn
		
		;! member-fn
		;! assoc-fn
		
		
		;! alist-cons
		;! alist-copy
		;! alist-delete
		;! any
		;! append-map
		;! append-reverse
		;! break
		;! circular-list
		;! concatenate
		;! cons*
		;! count
		;! delete
		;! delete-duplicates
		;! drop
		;! drop-right
		;! drop-while
		;! every
		;! filter-map
		;! find-pair
		;! find-tail
		;! fold
		;! fold-right
		;! iota
		;! last
		;! last-pair
		;! list-index
		;! list-tabulate
		;! list<
		;! list<=
		;! list=
		;! list>
		;! list>=
		;! pair-fold
		;! pair-fold-right
		;! pair-for-each
		;! partition
		;! reduce
		;! reduce-right
		;! remove
		;! span
		;! split-at
		;! take
		;! take-right
		;! take-while
		;! unfold
		;! unfold-right
		;! unzip*
		;! unzip1
		;! unzip2
		;! unzip3
		;! unzip4
		;! unzip5
		;! zip
		
		
		
		
		;; ---- R7RS unsupported procedures
		
		
		(features
			(category vs:r7rs vs:evaluator vs:compiler vs:unsupported)
			(type procedure)
			(implements (r7rs features)))
		
		
		(rationalize
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs rationalize)))
		
		(numerator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs numerator)))
		
		(denominator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs denominator)))
		
		(make-rectangular
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs make-rectangular)))
		
		(real-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs real-part)))
		
		(imag-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs imag-part)))
		
		(make-polar
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs make-polar)))
		
		(angle
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs angle)))
		
		(magnitude
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(implements (r7rs magnitude)))
		
		
		(call-with-current-continuation
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure)
			(alias call/cc)
			(implements (r7rs call-with-current-continuation)))
		
		(dynamic-wind
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure)
			(implements (r7rs dynamic-wind)))
		
		(with-exception-handler
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure)
			(implements (r7rs with-exception-handler)))
		
		(raise-continuable
			(category vs:r7rs vs:errors vs:evaluator vs:unsupported)
			(type procedure)
			(implements (r7rs raise-continuable)))
		
		
		
		
		;; ---- miscellaneous unsupported procedures
		
		;! car+cdr
		;! length+
		
		;! alist-delete!
		;! append!
		;! append-map!
		;! append-reverse!
		;! break!
		;! concatenate!
		;! delete!
		;! delete-duplicates!
		;! drop-right!
		;! drop-while!
		;! filter!
		;! map!
		;! partition!
		;! remove!
		;! reverse!
		;! span!
		;! split-at!
		;! take!
		;! take-right!
		;! take-while!
		
		;! lset-adjoinlset-diff+intersection
		;! lset-diff+intersection!
		;! lset-difference
		;! lset-difference!
		;! lset-intersection
		;! lset-intersection!
		;! lset-union
		;! lset-union!
		;! lset-xor
		;! lset-xor!
		;! lset<
		;! lset<=
		;! lset=
		;! lset>
		;! lset>=
		
		
		
		
	)
	
	
	
	
)

