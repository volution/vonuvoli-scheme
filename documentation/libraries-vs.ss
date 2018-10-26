
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
	
	
	
	
	(exports
		
		
		
		
		(vs:base
			(descriptor (vonuvoli base))
		)
		
		(vs:core
			(parent vs:base)
			(descriptor (vonuvoli core))
		)
		
		(vs:core-procedures
			(parent vs:core)
			(descriptor (vonuvoli core syntaxes))
		)
		
		(vs:core-syntaxes
			(parent vs:core)
			(descriptor (vonuvoli core syntaxes))
		)
		
		(vs:types*
			(parent vs:base)
			(descriptor (vonuvoli types))
		)
		
		(vs:types
			(parent vs:types*)
			(descriptor (vonuvoli types positive))
		)
		
		(vs:types-negated
			(parent vs:types*)
			(descriptor (vonuvoli types negated))
		)
		
		(vs:booleans
			(parent vs:base)
			(descriptor (vonuvoli booleans))
		)
		
		(vs:arithmetic
			(parent vs:base)
			(descriptor (vonuvoli arithmetic))
		)
		
		(vs:symbols
			(parent vs:base)
			(descriptor (vonuvoli symbols))
		)
		
		(vs:keywords
			(parent vs:base)
			(descriptor (vonuvoli keywords))
		)
		
		(vs:uniques
			(parent vs:base)
			(descriptor (vonuvoli uniques))
		)
		
		(vs:characters
			(parent vs:base)
			(descriptor (vonuvoli characters))
		)
		
		(vs:strings
			(parent vs:base)
			(descriptor (vonuvoli strings))
		)
		
		(vs:bytes
			(parent vs:base)
			(descriptor (vonuvoli bytes))
		)
		
		(vs:pairs
			(parent vs:base)
			(descriptor (vonuvoli pairs))
		)
		
		(vs:lists
			(parent vs:base)
			(descriptor (vonuvoli pairs))
		)
		
		(vs:arrays
			(parent vs:base)
			(descriptor (vonuvoli arrays))
		)
		
		(vs:values
			(parent vs:base)
			(descriptor (vonuvoli values))
		)
		
		(vs:records
			(parent vs:base)
			(descriptor (vonuvoli records))
		)
		
		(vs:errors
			(parent vs:base)
			(descriptor (vonuvoli errors))
		)
		
		(vs:functional
			(parent vs:base)
			(descriptor (vonuvoli functional))
		)
		
		
		
		
		(vs:comparisons*
			(descriptor (vonuvoli comparisons))
		)
		
		(vs:comparisons
			(parent vs:comparisons*)
			(descriptor (vonuvoli comparisons positive))
		)
		
		(vs:comparisons-negated
			(parent vs:comparisons*)
			(descriptor (vonuvoli comparisons negated))
		)
		
		
		
		
		(vs:regex
			(descriptor (vonuvoli regex))
		)
		
		(vs:regex-strings
			(parent vs:regex)
			(descriptor (vonuvoli regex strings))
		)
		
		(vs:regex-bytes
			(parent vs:regex)
			(descriptor (vonuvoli regex bytes))
		)
		
		
		
		
		(vs:io
			(descriptor (vonuvoli io))
		)
		
		(vs:io-open
			(parent vs:io)
			(descriptor (vonuvoli io open))
		)
		
		(vs:io-strings
			(parent vs:io)
			(descriptor (vonuvoli io strings))
		)
		
		(vs:io-bytes
			(parent vs:io)
			(descriptor (vonuvoli io bytes))
		)
		
		(vs:io-values
			(parent vs:io)
			(descriptor (vonuvoli io values))
		)
		
		(vs:io-descriptors
			(parent vs:io)
			(descriptor (vonuvoli io descriptors))
		)
		
		(vs:io-temporary
			(parent vs:io)
			(descriptor (vonuvoli io temporary))
		)
		
		
		
		
		(vs:paths
			(descriptor (vonuvoli paths))
		)
		
		(vs:os
			(descriptor (vonuvoli os))
		)
		
		(vs:processes
			(descriptor (vonuvoli processes))
		)
		
		(vs:time
			(descriptor (vonuvoli time))
		)
		
		
		
		
		(vs:parameters
			(descriptor (vonuvoli parameters))
		)
		
		(vs:contexts
			(descriptor (vonuvoli contexts))
		)
		
		(vs:promises
			(descriptor (vonuvoli promises))
		)
		
		(vs:evaluator
			(descriptor (vonuvoli evaluator))
		)
		
		
		
		
		(vs:hashes
			(descriptor (vonuvoli hashes))
		)
		
		(vs:serialization
			(descriptor (vonuvoli serialization))
		)
		
		(vs:cache
			(descriptor (vonuvoli cache))
		)
		
		
		
		
		(vs:dynamic
			(descriptor (vonuvoli dynamic))
		)
		
		
		
		
	)
	
	
	
	
	(definitions
		
		
		
		
		;; ---- syntaxes
		
		
		(quote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs quote)))
		
		(quasiquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs quasiquote)))
		
		(unquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs unquote)))
		
		(unquote-splicing
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs unquote-splicing)))
		
		
		(_
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(export vs:core-syntaxes)
			(implements (r7rs _)))
		
		(...
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(export vs:core-syntaxes)
			(implements (r7rs ...)))
		
		(=>
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(export vs:core-syntaxes)
			(implements (r7rs =>)))
		
		(else
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax)
			(export vs:core-syntaxes)
			(implements (r7rs else)))
		
		
		(begin
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs begin)))
		
		(and
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs and)))
		
		(or
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs or)))
		
		
		(if
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs if)))
		
		(when
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs when)))
		
		(unless
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs unless)))
		
		(cond
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs cond)))
		
		(case
			(category vs:r7rs vs:control)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs case)))
		
		
		(do
			(category vs:r7rs vs:control vs:loops)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs do)))
		
		(while
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		(until
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		
		(do-cond
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		(while-cond
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		(until-cond
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		
		(loop
			(category vs:loops vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		
		(guard
			(category vs:r7rs vs:errors vs:evaluator)
			(type syntax)
			(implements (r7rs guard))
			(export vs:errors)
		)
		
		(guard*
			(category vs:syntaxes)
			(type syntax)
			(export vs:errors)
		)
		
		
		(locals
			(category vs:contexts vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		(let
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs let)))
		
		(let*
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs let*)))
		
		(letrec
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs letrec)))
		
		(letrec*
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs letrec*)))
		
		(let-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(export vs:values)
			(implements (r7rs let-values)))
		
		(let*-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(export vs:values)
			(implements (r7rs let*-values)))
		
		
		(define
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs define)))
		
		(redefine
			(category vs:contexts vs:syntaxes)
			(type syntax)
			(export vs:core-syntaxes)
		)
		
		(define-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax)
			(export vs:values)
			(implements (r7rs define-values)))
		
		(redefine-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax)
			(export vs:values)
		)
		
		(set!
			(category vs:r7rs vs:contexts)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs set!)))
		
		(set!-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax)
			(export vs:values)
		)
		
		
		(define-record-type
			(category vs:r7rs vs:contexts vs:records)
			(type syntax)
			(export vs:records)
			(implements (r7rs define-record-type)))
		
		
		(lambda
			(category vs:r7rs vs:lambda)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs lambda)))
		
		
		(parameterize
			(category vs:r7rs vs:parameters)
			(type syntax)
			(export vs:parameters)
			(implements (r7rs parameterize)))
		
		
		
		
		;; ---- types
		
		
		(not
			(category vs:r7rs vs:booleans)
			(type predicate)
			(export vs:booleans)
			(implements (r7rs not)))
		
		
		(null?
			(category vs:r7rs vs:lists vs:types)
			(type type-predicate)
			(export vs:types vs:lists)
			(extends (r7rs null?))
			(signature
				((null 1...) -> true)
				((pair 1...) -> false)
				((any 1...) -> false))
		)
		
		(void?
			(type type-predicate)
			(export vs:types)
		)
		
		(undefined?
			(type type-predicate)
			(export vs:types)
		)
		
		
		(boolean?
			(category vs:r7rs vs:booleans vs:types)
			(type type-predicate)
			(export vs:types vs:booleans)
			(extends (r7rs boolean?))
			(signature
				((boolean 1...) -> true)
				((any 1...) -> false))
		)
		
		(true?
			(type predicate)
			(export vs:types vs:booleans)
		)
		
		(false?
			(type predicate)
			(export vs:types vs:booleans)
		)
		
		(true-or-equivalent?
			(type predicate)
			(export vs:booleans)
		)
		
		(false-or-equivalent?
			(type predicate)
			(export vs:booleans)
		)
		
		
		(number?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(export vs:types vs:arithmetic)
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
			(export vs:types vs:arithmetic)
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
			(export vs:types vs:arithmetic)
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
			(export vs:types vs:arithmetic)
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
			(export vs:types vs:arithmetic)
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
			(export vs:arithmetic)
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
			(export vs:arithmetic)
			(extends (r7rs exact?))
			(signature
				((exact-number 1...) -> true)
				((inexact-number 1...) -> false)
				((number 1...) -> false))
		)
		
		(inexact?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate)
			(export vs:arithmetic)
			(extends (r7rs inexact?))
			(signature
				((inexact-number 1...) -> true)
				((exact-number 1...) -> false)
				((number 1...) -> false))
		)
		
		
		(char?
			(category vs:r7rs vs:characters vs:types)
			(type type-predicate)
			(export vs:types vs:characters)
			(extends (r7rs char?))
			(signature
				((character 1...) -> true)
				((any 1...) -> false))
		)
		
		(symbol?
			(category vs:r7rs vs:symbols vs:types)
			(type type-predicate)
			(export vs:types vs:symbols)
			(extends (r7rs symbol?))
			(signature
				((symbol 1...) -> true)
				((any 1...) -> false))
		)
		
		(keyword?
			(type type-predicate)
			(export vs:types vs:keywords)
		)
		
		(unique?
			(type type-predicate)
			(export vs:types vs:uniques)
		)
		
		
		(string?
			(category vs:r7rs vs:strings vs:types)
			(type type-predicate)
			(export vs:types vs:strings)
			(extends (r7rs string?))
			(signature
				((string 1...) -> true)
				((any 1...) -> false))
		)
		
		(string-immutable?
			(type type-predicate)
			(export vs:types vs:strings)
		)
		
		(string-mutable?
			(type type-predicate)
			(export vs:types vs:strings)
		)
		
		(string-empty?
			(type predicate)
			(export vs:strings)
		)
		
		(string-immutable-empty?
			(type predicate)
			(export vs:strings)
		)
		
		(string-mutable-empty?
			(type predicate)
			(export vs:strings)
		)
		
		
		(bytevector?
			(category vs:r7rs vs:bytes)
			(type type-predicate)
			(export vs:types vs:bytes)
			(extends (r7rs bytevector?))
			(signature
				((bytevector 1...) -> true)
				((any 1...) -> false))
		)
		
		(bytevector-immutable?
			(type type-predicate)
			(export vs:types vs:bytes)
		)
		
		(bytevector-mutable?
			(type type-predicate)
			(export vs:types vs:bytes)
		)
		
		(bytevector-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		(bytevector-immutable-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		(bytevector-mutable-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		
		(string-regex?
			(type type-predicate)
			(export vs:types vs:regex-strings)
		)
		
		(bytevector-regex?
			(type type-predicate)
			(export vs:types vs:regex-bytes)
		)
		
		
		(pair?
			(category vs:r7rs vs:pairs vs:lists vs:types)
			(type type-predicate)
			(export vs:types vs:pairs)
			(extends (r7rs pair?))
			(signature
				((pair 1...) -> true)
				((null 1...) -> false)
				((any 1...) -> false))
		)
		
		(pair-immutable?
			(type type-predicate)
			(export vs:types vs:pairs)
		)
		
		(pair-mutable?
			(type type-predicate)
			(export vs:types vs:pairs)
		)
		
		
		(vector?
			(category vs:r7rs vs:vectors vs:types)
			(type type-predicate)
			(export vs:types vs:arrays)
			(extends (r7rs vector?))
			(signature
				((vector 1...) -> true)
				((any 1...) -> false))
		)
		
		(vector-immutable?
			(type type-predicate)
			(export vs:types vs:arrays)
		)
		
		(vector-mutable?
			(type type-predicate)
			(export vs:types vs:arrays)
		)
		
		(vector-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		(vector-immutable-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		(vector-mutable-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		
		(values?
			(type type-predicate)
			(export vs:types vs:values)
		)
		
		(values-empty?
			(type predicate)
			(export vs:values)
		)
		
		
		(record-type?
			(type type-predicate)
			(export vs:types vs:records)
		)
		
		(record?
			(type type-predicate)
			(export vs:types vs:records)
		)
		
		(record-immutable?
			(type type-predicate)
			(export vs:types vs:records)
		)
		
		(record-mutable?
			(type type-predicate)
			(export vs:types vs:records)
		)
		
		
		(error-object?
			(category vs:r7rs vs:errors)
			(type type-predicate)
			(export vs:errors)
			(extends (r7rs error-object?))
			(signature
				((error-object 1...) -> true)
				((any 1...) -> false))
		)
		
		(syntax-error?
			(type predicate)
			(export vs:errors)
		)
		
		(file-error?
			(category vs:r7rs vs:errors)
			(type predicate)
			(export vs:errors)
			(extends (r7rs file-error?))
			(signature
				((any 1...) -> boolean))
		)
		
		(port-error?
			(type type-predicate)
			(export vs:errors)
		)
		
		(read-error?
			(category vs:r7rs vs:errors)
			(type predicate)
			(export vs:errors)
			(extends (r7rs read-error?))
			(signature
				((any 1...) -> boolean))
		)
		
		(write-error?
			(type type-predicate)
			(export vs:errors)
		)
		
		
		
		
		(any-list?
			(type predicate)
			(export vs:lists)
		)
		
		(empty-list?
			(type predicate)
			(export vs:lists)
			(alias null-list?))
		
		(any-or-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		
		(proper-list?
			(type predicate)
			(export vs:lists)
		)
		
		(proper-or-empty-list?
			(category vs:r7rs vs:lists vs:types)
			(type predicate)
			(export vs:lists)
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
			(type predicate)
			(export vs:lists)
		)
		
		(dotted-or-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		
		(circular-list?
			(type predicate)
			(export vs:lists)
		)
		
		(circular-or-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		
		(procedure?
			(category vs:r7rs vs:functions vs:types)
			(type type-predicate)
			(export vs:types)
			(extends (r7rs procedure?))
			(signature
				((procedure 1...) -> true)
				((any 1...) -> false))
		)
		
		(syntax?
			(type predicate)
			(export vs:types)
		)
		
		
		(port?
			(category vs:r7rs vs:ports vs:types)
			(type type-predicate)
			(export vs:types vs:io)
			(extends (r7rs port?))
			(signature
				((port 1...) -> true)
				((any 1...) -> false))
		)
		
		(input-port?
			(category vs:r7rs vs:ports:input)
			(type predicate)
			(export vs:io)
			(extends (r7rs input-port?))
			(signature
				((input-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(output-port?
			(category vs:r7rs vs:ports:output)
			(type predicate)
			(export vs:io)
			(extends (r7rs output-port?))
			(signature
				((output-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(binary-port?
			(category vs:r7rs vs:ports)
			(type predicate)
			(export vs:io)
			(extends (r7rs binary-port?))
			(signature
				((binary-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		(textual-port?
			(category vs:r7rs vs:ports)
			(type predicate)
			(export vs:io)
			(extends (r7rs textual-port?))
			(signature
				((textual-port 1...) -> true)
				((port 1...) -> false)
				((any 1...) -> false))
		)
		
		
		(binary-input-port?
			(type predicate)
			(export vs:io)
		)
		
		(textual-input-port?
			(type predicate)
			(export vs:io)
		)
		
		(binary-output-port?
			(type predicate)
			(export vs:io)
		)
		
		(textual-output-port?
			(type predicate)
			(export vs:io)
		)
		
		
		(eof-object?
			(category vs:r7rs vs:ports vs:globals)
			(type predicate)
			(export vs:types vs:io)
			(extends (r7rs eof-object?))
			(signature
				((eof-object 1...) -> true)
				((any 1...) -> false))
		)
		
		
		(path?
			(type type-predicate)
			(export vs:types vs:paths)
		)
		
		(path-absolute?
			(type predicate)
			(export vs:paths)
		)
		
		(path-relative?
			(type predicate)
			(export vs:paths)
		)
		
		
		(process?
			(type type-predicate)
			(export vs:types vs:processes)
		)
		
		
		(context?
			(type type-predicate)
			(export vs:types vs:contexts)
		)
		
		(binding?
			(type type-predicate)
			(export vs:types vs:contexts)
		)
		
		(parameters?
			(type type-predicate)
			(export vs:types vs:parameters)
		)
		
		(parameter?
			(type type-predicate)
			(export vs:types vs:parameters)
		)
		
		(promise?
			(category vs:r7rs vs:promises vs:evaluator)
			(type type-predicate)
			(export vs:types vs:promises)
			(extends (r7rs promise?))
			(signature
				((promise 1...) -> true)
				((any 1...) -> false))
		)
		
		(resource?
			(type type-predicate)
			(export vs:types)
		)
		
		(internal?
			(type type-predicate)
			(export vs:types)
		)
		
		(opaque?
			(type predicate)
			(export vs:types)
		)
		
		
		(zero?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(export vs:arithmetic)
			(extends (r7rs zero?))
			(signature
				((number-zero 1...) -> true)
				((number 1...) -> false))
		)
		
		(positive?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(export vs:arithmetic)
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
			(export vs:arithmetic)
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
			(export vs:arithmetic)
			(implements (r7rs finite?)))
		
		(infinite?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(export vs:arithmetic)
			(implements (r7rs infinite?)))
		
		(nan?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(export vs:arithmetic)
			(implements (r7rs nan?)))
		
		(even?
			(category vs:r7rs vs:arithmetic)
			(type predicate)
			(export vs:arithmetic)
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
			(export vs:arithmetic)
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
			(export vs:characters)
			(implements (r7rs char-numeric?)))
		
		(char-alphabetic?
			(category vs:r7rs vs:characters)
			(type predicate)
			(export vs:characters)
			(implements (r7rs char-alphabetic?)))
		
		(char-upper-case?
			(category vs:r7rs vs:characters)
			(type predicate)
			(export vs:characters)
			(implements (r7rs char-upper-case?)))
		
		(char-lower-case?
			(category vs:r7rs vs:characters)
			(type predicate)
			(export vs:characters)
			(implements (r7rs char-lower-case?)))
		
		(char-alphabetic-or-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(char-whitespace?
			(category vs:r7rs vs:characters)
			(type predicate)
			(export vs:characters)
			(implements (r7rs char-whitespace?)))
		
		(char-control?
			(type predicate)
			(export vs:characters)
		)
		
		
		(char-ascii?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-numeric-8?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-numeric-16?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-alphabetic?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-upper-case?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-lower-case?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-alphabetic-or-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-whitespace?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-control?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-punctuation?
			(type predicate)
			(export vs:characters)
		)
		
		(char-ascii-graphic?
			(type predicate)
			(export vs:characters)
		)
		
		
		(fs-metadata?
			(type type-predicate))
		
		(cache?
			(type type-predicate)
			(export vs:cache)
		)
		
		
		
		
		;; ---- types (negated)
		
		
		(not-null?
			(type predicate)
			(export vs:types-negated vs:lists)
		)
		
		(not-void?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-undefined?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-boolean?
			(type predicate)
			(export vs:types-negated vs:booleans)
		)
		
		(not-true?
			(type predicate)
			(export vs:types-negated vs:booleans)
		)
		
		(not-false?
			(type predicate)
			(export vs:types-negated vs:booleans)
		)
		
		(not-true-or-equivalent?
			(type predicate)
			(export vs:booleans)
		)
		
		(not-false-or-equivalent?
			(type predicate)
			(export vs:booleans)
		)
		
		(not-number?
			(type predicate)
			(export vs:types-negated vs:arithmetic)
		)
		
		(not-complex?
			(type predicate)
			(export vs:types-negated vs:arithmetic)
		)
		
		(not-real?
			(type predicate)
			(export vs:types-negated vs:arithmetic)
		)
		
		(not-rational?
			(type predicate)
			(export vs:types-negated vs:arithmetic)
		)
		
		(not-integer?
			(type predicate)
			(export vs:types-negated vs:arithmetic)
		)
		
		(not-exact-integer?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-exact?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-inexact?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-char?
			(type predicate)
			(export vs:types-negated vs:characters)
		)
		
		(not-symbol?
			(type predicate)
			(export vs:types-negated vs:symbols)
		)
		
		(not-keyword?
			(type predicate)
			(export vs:types-negated vs:keywords)
		)
		
		(not-unique?
			(type predicate)
			(export vs:types-negated vs:uniques)
		)
		
		(not-string?
			(type predicate)
			(export vs:types-negated vs:strings)
		)
		
		(not-string-immutable?
			(type predicate)
			(export vs:types-negated vs:strings)
		)
		
		(not-string-mutable?
			(type predicate)
			(export vs:types-negated vs:strings)
		)
		
		(not-string-empty?
			(type predicate)
			(export vs:strings)
		)
		
		(not-string-immutable-empty?
			(type predicate)
			(export vs:strings)
		)
		
		(not-string-mutable-empty?
			(type predicate)
			(export vs:strings)
		)
		
		(not-bytevector?
			(type predicate)
			(export vs:types-negated vs:bytes)
		)
		
		(not-bytevector-immutable?
			(type predicate)
			(export vs:types-negated vs:bytes)
		)
		
		(not-bytevector-mutable?
			(type predicate)
			(export vs:types-negated vs:bytes)
		)
		
		(not-bytevector-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		(not-bytevector-immutable-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		(not-bytevector-mutable-empty?
			(type predicate)
			(export vs:bytes)
		)
		
		(not-string-regex?
			(type predicate)
			(export vs:types-negated vs:regex-strings)
		)
		
		(not-bytevector-regex?
			(type predicate)
			(export vs:types-negated vs:regex-bytes)
		)
		
		(not-pair?
			(type predicate)
			(export vs:types-negated vs:pairs)
		)
		
		(not-pair-immutable?
			(type predicate)
			(export vs:types-negated vs:pairs)
		)
		
		(not-pair-mutable?
			(type predicate)
			(export vs:types-negated vs:pairs)
		)
		
		(not-vector?
			(type predicate)
			(export vs:types-negated vs:arrays)
		)
		
		(not-vector-immutable?
			(type predicate)
			(export vs:types-negated vs:arrays)
		)
		
		(not-vector-mutable?
			(type predicate)
			(export vs:types-negated vs:arrays)
		)
		
		(not-vector-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		(not-vector-immutable-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		(not-vector-mutable-empty?
			(type predicate)
			(export vs:arrays)
		)
		
		(not-values?
			(type predicate)
			(export vs:types-negated vs:values)
		)
		
		(not-values-empty?
			(type predicate)
			(export vs:values)
		)
		
		(not-record-type?
			(type predicate)
			(export vs:types-negated vs:records)
		)
		
		(not-record?
			(type predicate)
			(export vs:types-negated vs:records)
		)
		
		(not-record-immutable?
			(type predicate)
			(export vs:types-negated vs:records)
		)
		
		(not-record-mutable?
			(type predicate)
			(export vs:types-negated vs:records)
		)
		
		(not-error-object?
			(type predicate)
			(extends (r7rs error-object?))
		)
		
		(not-syntax-error?
			(type predicate)
			(export vs:errors)
		)
		
		(not-file-error?
			(type predicate)
			(export vs:errors)
		)
		
		(not-port-error?
			(type predicate)
			(export vs:errors)
		)
		
		(not-read-error?
			(type predicate)
			(export vs:errors)
		)
		
		(not-write-error?
			(type predicate)
			(export vs:errors)
		)
		
		(not-any-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-any-or-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-proper-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-proper-or-empty-list?
			(type predicate)
			(export vs:lists)
			(alias not-list?))
		
		(not-dotted-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-dotted-list-or-emtpy?
			(type predicate)
			(export vs:lists)
		)
		
		(not-circular-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-circular-or-empty-list?
			(type predicate)
			(export vs:lists)
		)
		
		(not-procedure?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-syntax?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-port?
			(type predicate)
			(export vs:types-negated vs:io)
		)
		
		(not-input-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-output-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-binary-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-textual-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-binary-input-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-textual-input-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-binary-output-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-textual-output-port?
			(type predicate)
			(export vs:io)
		)
		
		(not-eof-object?
			(type predicate)
			(export vs:types-negated vs:io)
		)
		
		(not-path?
			(type predicate)
			(export vs:types-negated vs:paths)
		)
		
		(not-path-absolute?
			(type predicate)
			(export vs:paths)
		)
		
		(not-path-relative?
			(type predicate)
			(export vs:paths)
		)
		
		(not-process?
			(type predicate)
			(export vs:types-negated vs:processes)
		)
		
		(not-context?
			(type predicate)
			(export vs:types-negated vs:contexts)
		)
		
		(not-binding?
			(type predicate)
			(export vs:types-negated vs:contexts)
		)
		
		(not-parameters?
			(type predicate)
			(export vs:types-negated vs:parameters)
		)
		
		(not-parameter?
			(type predicate)
			(export vs:types-negated vs:parameters)
		)
		
		(not-promise?
			(type predicate)
			(export vs:types-negated vs:promises)
		)
		
		(not-resource?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-internal?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-opaque?
			(type predicate)
			(export vs:types-negated)
		)
		
		(not-zero?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-positive?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-negative?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-finite?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-infinite?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-nan?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-even?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-odd?
			(type predicate)
			(export vs:arithmetic)
		)
		
		(not-char-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-alphabetic?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-upper-case?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-lower-case?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-alphabetic-or-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-whitespace?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-control?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-numeric-8?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-numeric-16?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-alphabetic?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-upper-case?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-lower-case?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-alphabetic-or-numeric?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-whitespace?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-control?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-punctuation?
			(type predicate)
			(export vs:characters)
		)
		
		(not-char-ascii-graphic?
			(type predicate)
			(export vs:characters)
		)
		
		(not-fs-metadata?
			(type predicate))
		
		(not-cache?
			(type predicate)
			(export vs:cache)
		)
		
		
		
		
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
			(export vs:comparisons vs:core-procedures)
			(alias eq?)
			(extends (r7rs eq?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		(equivalent-by-value-strict?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:core-procedures)
			(alias eqv?)
			(extends (r7rs eqv?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		(equivalent-by-value-strict-recursive?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:core-procedures)
			(alias equal?)
			(extends (r7rs equal?))
			(signature
				((any) -> true)
				((any 2...) -> boolean))
		)
		
		
		(equivalent-by-value-coerced?
			(type procedure)
			(export vs:comparisons vs:core-procedures)
		)
		
		(equivalent-by-value-coerced-recursive?
			(type procedure)
			(export vs:comparisons vs:core-procedures)
		)
		
		
		(generic<?
			(type procedure)
			(export vs:comparisons)
		)
		
		(generic<=?
			(type procedure)
			(export vs:comparisons)
		)
		
		(generic=?
			(type procedure)
			(export vs:comparisons)
		)
		
		(generic>=?
			(type procedure)
			(export vs:comparisons)
		)
		
		(generic>?
			(type procedure)
			(export vs:comparisons)
		)
		
		
		(boolean<?
			(type procedure)
			(export vs:comparisons vs:booleans)
		)
		
		(boolean<=?
			(type procedure)
			(export vs:comparisons vs:booleans)
		)
		
		(boolean=?
			(category vs:r7rs vs:booleans vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:booleans)
			(extends (r7rs boolean=?))
			(signature
				((boolean) -> true)
				((boolean 2...) -> boolean))
		)
		
		(boolean>=?
			(type procedure)
			(export vs:comparisons vs:booleans)
		)
		
		(boolean>?
			(type procedure)
			(export vs:comparisons vs:booleans)
		)
		
		
		(<
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<)
			(export vs:comparisons vs:arithmetic)
			(extends (r7rs <))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(<=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<=)
			(export vs:comparisons vs:arithmetic)
			(extends (r7rs <=))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(=
			(category vs:r7rs vs:arithmetic vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:arithmetic)
			(extends (r7rs =))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(>=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>=)
			(export vs:comparisons vs:arithmetic)
			(extends (r7rs >=))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		(>
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>)
			(export vs:comparisons vs:arithmetic)
			(extends (r7rs >))
			(signature
				((number-not-nan) -> true)
				((number-not-nan 2...) -> boolean)
				((number 1...) -> false))
		)
		
		
		(char<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<)
			(export vs:comparisons vs:characters)
			(implements (r7rs char<?)))
		
		(char<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char<=?)))
		
		(char=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char=?)))
		
		(char>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char>=?)))
		
		(char>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>)
			(export vs:comparisons vs:characters)
			(implements (r7rs char>?)))
		
		
		(char-ci<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<)
			(export vs:comparisons vs:characters)
			(implements (r7rs char-ci<?)))
		
		(char-ci<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char-ci<=?)))
		
		(char-ci=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char-ci=?)))
		
		(char-ci>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=)
			(export vs:comparisons vs:characters)
			(implements (r7rs char-ci>=?)))
		
		(char-ci>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>)
			(export vs:comparisons vs:characters)
			(implements (r7rs char-ci>?)))
		
		
		(string<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<)
			(export vs:comparisons vs:strings)
			(implements (r7rs string<?)))
		
		(string<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string<=?)))
		
		(string=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string=?)))
		
		(string>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string>=?)))
		
		(string>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>)
			(export vs:comparisons vs:strings)
			(implements (r7rs string>?)))
		
		
		(string-ci<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<)
			(export vs:comparisons vs:strings)
			(implements (r7rs string-ci<?)))
		
		(string-ci<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string-ci<=?)))
		
		(string-ci=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string-ci=?)))
		
		(string-ci>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=)
			(export vs:comparisons vs:strings)
			(implements (r7rs string-ci>=?)))
		
		(string-ci>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>)
			(export vs:comparisons vs:strings)
			(implements (r7rs string-ci>?)))
		
		
		(symbol<?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol<=?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol=?
			(category vs:r7rs vs:symbols vs:comparisons vs:equivalence)
			(type comparator=)
			(export vs:comparisons vs:symbols)
			(extends (r7rs symbol=?))
			(signature
				((symbol) -> true)
				((symbol 2...) -> boolean))
		)
		
		(symbol>=?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol>?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		
		(symbol-ci<?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol-ci<=?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol-ci=?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol-ci>=?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		(symbol-ci>?
			(type procedure)
			(export vs:comparisons vs:symbols)
		)
		
		
		(keyword<?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword<=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword>=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword>?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		
		(keyword-ci<?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword-ci<=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword-ci=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword-ci>=?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		(keyword-ci>?
			(type procedure)
			(export vs:comparisons vs:keywords)
		)
		
		
		(unique<?
			(type procedure)
			(export vs:comparisons vs:uniques)
		)
		
		(unique<=?
			(type procedure)
			(export vs:comparisons vs:uniques)
		)
		
		(unique=?
			(type procedure)
			(export vs:comparisons vs:uniques)
		)
		
		(unique>=?
			(type procedure)
			(export vs:comparisons vs:uniques)
		)
		
		(unique>?
			(type procedure)
			(export vs:comparisons vs:uniques)
		)
		
		
		(bytevector<?
			(type procedure)
			(export vs:comparisons vs:bytes)
		)
		
		(bytevector<=?
			(type procedure)
			(export vs:comparisons vs:bytes)
		)
		
		(bytevector=?
			(type procedure)
			(export vs:comparisons vs:bytes)
		)
		
		(bytevector>=?
			(type procedure)
			(export vs:comparisons vs:bytes)
		)
		
		(bytevector>?
			(type procedure)
			(export vs:comparisons vs:bytes)
		)
		
		
		(pair<?
			(type procedure)
			(export vs:comparisons vs:pairs)
		)
		
		(pair<=?
			(type procedure)
			(export vs:comparisons vs:pairs)
		)
		
		(pair=?
			(type procedure)
			(export vs:comparisons vs:pairs)
		)
		
		(pair>=?
			(type procedure)
			(export vs:comparisons vs:pairs)
		)
		
		(pair>?
			(type procedure)
			(export vs:comparisons vs:pairs)
		)
		
		
		(vector<?
			(type procedure)
			(export vs:comparisons vs:arrays)
		)
		
		(vector<=?
			(type procedure)
			(export vs:comparisons vs:arrays)
		)
		
		(vector=?
			(type procedure)
			(export vs:comparisons vs:arrays)
		)
		
		(vector>=?
			(type procedure)
			(export vs:comparisons vs:arrays)
		)
		
		(vector>?
			(type procedure)
			(export vs:comparisons vs:arrays)
		)
		
		
		(values<?
			(type procedure)
			(export vs:comparisons vs:values)
		)
		
		(values<=?
			(type procedure)
			(export vs:comparisons vs:values)
		)
		
		(values=?
			(type procedure)
			(export vs:comparisons vs:values)
		)
		
		(values>=?
			(type procedure)
			(export vs:comparisons vs:values)
		)
		
		(values>?
			(type procedure)
			(export vs:comparisons vs:values)
		)
		
		
		(record<?
			(type procedure)
			(export vs:comparisons vs:records)
		)
		
		(record<=?
			(type procedure)
			(export vs:comparisons vs:records)
		)
		
		(record=?
			(type procedure)
			(export vs:comparisons vs:records)
		)
		
		(record>=?
			(type procedure)
			(export vs:comparisons vs:records)
		)
		
		(record>?
			(type procedure)
			(export vs:comparisons vs:records)
		)
		
		
		(path<?
			(type procedure)
			(export vs:comparisons vs:paths)
		)
		
		(path<=?
			(type procedure)
			(export vs:comparisons vs:paths)
		)
		
		(path=?
			(type procedure)
			(export vs:comparisons vs:paths)
		)
		
		(path>=?
			(type procedure)
			(export vs:comparisons vs:paths)
		)
		
		(path>?
			(type procedure)
			(export vs:comparisons vs:paths)
		)
		
		
		
		
		;; ---- comparisons (negated)
		
		
		(not-equivalent-by-identity?
			(type procedure)
			(export vs:comparisons-negated vs:core-procedures)
			(alias not-eq?))
		
		(not-equivalent-by-value-strict?
			(type procedure)
			(export vs:comparisons-negated vs:core-procedures)
			(alias not-eqv?))
		
		(not-equivalent-by-value-strict-recursive?
			(type procedure)
			(export vs:comparisons-negated vs:core-procedures)
			(alias not-equal?))
		
		
		(not-equivalent-by-value-coerced?
			(type procedure)
			(export vs:comparisons-negated vs:core-procedures)
		)
		
		(not-equivalent-by-value-coerced-recursive?
			(type procedure)
			(export vs:comparisons-negated vs:core-procedures)
		)
		
		
		(not-generic<?
			(type procedure)
			(export vs:comparisons-negated)
		)
		
		(not-generic<=?
			(type procedure)
			(export vs:comparisons-negated)
		)
		
		(not-generic=?
			(type procedure)
			(export vs:comparisons-negated)
		)
		
		(not-generic>=?
			(type procedure)
			(export vs:comparisons-negated)
		)
		
		(not-generic>?
			(type procedure)
			(export vs:comparisons-negated)
		)
		
		
		(not-boolean<?
			(type procedure)
			(export vs:comparisons-negated vs:booleans)
		)
		
		(not-boolean<=?
			(type procedure)
			(export vs:comparisons-negated vs:booleans)
		)
		
		(not-boolean=?
			(type procedure)
			(export vs:comparisons-negated vs:booleans)
		)
		
		(not-boolean>=?
			(type procedure)
			(export vs:comparisons-negated vs:booleans)
		)
		
		(not-boolean>?
			(type procedure)
			(export vs:comparisons-negated vs:booleans)
		)
		
		
		(not-<?
			(type procedure)
			(export vs:comparisons-negated vs:arithmetic)
		)
		
		(not-<=?
			(type procedure)
			(export vs:comparisons-negated vs:arithmetic)
		)
		
		(not-=?
			(type procedure)
			(export vs:comparisons-negated vs:arithmetic)
		)
		
		(not->=?
			(type procedure)
			(export vs:comparisons-negated vs:arithmetic)
		)
		
		(not->?
			(type procedure)
			(export vs:comparisons-negated vs:arithmetic)
		)
		
		
		(not-char<?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char<=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char>=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char>?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		
		(not-char-ci<?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char-ci<=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char-ci=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char-ci>=?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		(not-char-ci>?
			(type procedure)
			(export vs:comparisons-negated vs:characters)
		)
		
		
		(not-string<?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string<=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string>=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string>?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		
		(not-string-ci<?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string-ci<=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string-ci=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string-ci>=?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		(not-string-ci>?
			(type procedure)
			(export vs:comparisons-negated vs:strings)
		)
		
		
		(not-symbol<?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol<=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol>=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol>?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		
		(not-symbol-ci<?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol-ci<=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol-ci=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol-ci>=?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		(not-symbol-ci>?
			(type procedure)
			(export vs:comparisons-negated vs:symbols)
		)
		
		
		(not-keyword<?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword<=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword>=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword>?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		
		(not-keyword-ci<?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword-ci<=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword-ci=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword-ci>=?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		(not-keyword-ci>?
			(type procedure)
			(export vs:comparisons-negated vs:keywords)
		)
		
		
		(not-unique<?
			(type procedure)
			(export vs:comparisons-negated vs:uniques)
		)
		
		(not-unique<=?
			(type procedure)
			(export vs:comparisons-negated vs:uniques)
		)
		
		(not-unique=?
			(type procedure)
			(export vs:comparisons-negated vs:uniques)
		)
		
		(not-unique>=?
			(type procedure)
			(export vs:comparisons-negated vs:uniques)
		)
		
		(not-unique>?
			(type procedure)
			(export vs:comparisons-negated vs:uniques)
		)
		
		
		(not-bytevector<?
			(type procedure)
			(export vs:comparisons-negated vs:bytes)
		)
		
		(not-bytevector<=?
			(type procedure)
			(export vs:comparisons-negated vs:bytes)
		)
		
		(not-bytevector=?
			(type procedure)
			(export vs:comparisons-negated vs:bytes)
		)
		
		(not-bytevector>=?
			(type procedure)
			(export vs:comparisons-negated vs:bytes)
		)
		
		(not-bytevector>?
			(type procedure)
			(export vs:comparisons-negated vs:bytes)
		)
		
		
		(not-pair<?
			(type procedure)
			(export vs:comparisons-negated vs:pairs)
		)
		
		(not-pair<=?
			(type procedure)
			(export vs:comparisons-negated vs:pairs)
		)
		
		(not-pair=?
			(type procedure)
			(export vs:comparisons-negated vs:pairs)
		)
		
		(not-pair>=?
			(type procedure)
			(export vs:comparisons-negated vs:pairs)
		)
		
		(not-pair>?
			(type procedure)
			(export vs:comparisons-negated vs:pairs)
		)
		
		
		(not-vector<?
			(type procedure)
			(export vs:comparisons-negated vs:arrays)
		)
		
		(not-vector<=?
			(type procedure)
			(export vs:comparisons-negated vs:arrays)
		)
		
		(not-vector=?
			(type procedure)
			(export vs:comparisons-negated vs:arrays)
		)
		
		(not-vector>=?
			(type procedure)
			(export vs:comparisons-negated vs:arrays)
		)
		
		(not-vector>?
			(type procedure)
			(export vs:comparisons-negated vs:arrays)
		)
		
		
		(not-values<?
			(type procedure)
			(export vs:comparisons-negated vs:values)
		)
		
		(not-values<=?
			(type procedure)
			(export vs:comparisons-negated vs:values)
		)
		
		(not-values=?
			(type procedure)
			(export vs:comparisons-negated vs:values)
		)
		
		(not-values>=?
			(type procedure)
			(export vs:comparisons-negated vs:values)
		)
		
		(not-values>?
			(type procedure)
			(export vs:comparisons-negated vs:values)
		)
		
		
		(not-record<?
			(type procedure)
			(export vs:comparisons-negated vs:records)
		)
		
		(not-record<=?
			(type procedure)
			(export vs:comparisons-negated vs:records)
		)
		
		(not-record=?
			(type procedure)
			(export vs:comparisons-negated vs:records)
		)
		
		(not-record>=?
			(type procedure)
			(export vs:comparisons-negated vs:records)
		)
		
		(not-record>?
			(type procedure)
			(export vs:comparisons-negated vs:records)
		)
		
		
		(not-path<?
			(type procedure)
			(export vs:comparisons-negated vs:paths)
		)
		
		(not-path<=?
			(type procedure)
			(export vs:comparisons-negated vs:paths)
		)
		
		(not-path=?
			(type procedure)
			(export vs:comparisons-negated vs:paths)
		)
		
		(not-path>=?
			(type procedure)
			(export vs:comparisons-negated vs:paths)
		)
		
		(not-path>?
			(type procedure)
			(export vs:comparisons-negated vs:paths)
		)
		
		
		
		
		;; ---- lists
		
		
		(car
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(export vs:pairs vs:lists)
			(implements (r7rs car)))
		
		(cdr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(export vs:pairs vs:lists)
			(implements (r7rs cdr)))
		
		(caar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(export vs:lists)
			(implements (r7rs caar)))
		
		(cdar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor)
			(export vs:lists)
			(implements (r7rs cdar)))
		
		
		(first-pair
			(type procedure)
			(export vs:lists)
		)
		
		(second-pair
			(type procedure)
			(export vs:lists)
		)
		
		(third-pair
			(type procedure)
			(export vs:lists)
		)
		
		(fourth-pair
			(type procedure)
			(export vs:lists)
		)
		
		(fifth-pair
			(type procedure)
			(export vs:lists)
		)
		
		(sixth-pair
			(type procedure)
			(export vs:lists)
		)
		
		(seventh-pair
			(type procedure)
			(export vs:lists)
		)
		
		(eighth-pair
			(type procedure)
			(export vs:lists)
		)
		
		(ninth-pair
			(type procedure)
			(export vs:lists)
		)
		
		(tenth-pair
			(type procedure)
			(export vs:lists)
		)
		
		
		(first
			(type procedure)
			(export vs:lists)
		)
		
		(second
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(export vs:lists)
			(alias cadr)
			(implements (r7rs cadr)))
		
		(third
			(type procedure)
			(export vs:lists)
		)
		
		(fourth
			(type procedure)
			(export vs:lists)
		)
		
		(fifth
			(type procedure)
			(export vs:lists)
		)
		
		(sixth
			(type procedure)
			(export vs:lists)
		)
		
		(seventh
			(type procedure)
			(export vs:lists)
		)
		
		(eighth
			(type procedure)
			(export vs:lists)
		)
		
		(ninth
			(type procedure)
			(export vs:lists)
		)
		
		(tenth
			(type procedure)
			(export vs:lists)
		)
		
		
		(first-tail
			(type procedure)
			(export vs:lists)
		)
		
		(second-tail
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(export vs:lists)
			(alias cddr)
			(implements (r7rs cddr)))
		
		(third-tail
			(type procedure)
			(export vs:lists)
		)
		
		(fourth-tail
			(type procedure)
			(export vs:lists)
		)
		
		(fifth-tail
			(type procedure)
			(export vs:lists)
		)
		
		(sixth-tail
			(type procedure)
			(export vs:lists)
		)
		
		(seventh-tail
			(type procedure)
			(export vs:lists)
		)
		
		(eighth-tail
			(type procedure)
			(export vs:lists)
		)
		
		(ninth-tail
			(type procedure)
			(export vs:lists)
		)
		
		(tenth-tail
			(type procedure)
			(export vs:lists)
		)
		
		
		(length
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs length)))
		
		(reverse
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs reverse)))
		
		(pair->immutable
			(type procedure)
			(export vs:pairs)
		)
		
		(pair->mutable
			(type procedure)
			(export vs:pairs)
		)
		
		(list->immutable
			(type procedure)
			(export vs:lists)
		)
		
		(list->mutable
			(type procedure)
			(export vs:lists)
		)
		
		
		(cons
			(category vs:r7rs vs:pairs vs:lists)
			(type constructor)
			(export vs:pairs vs:lists)
			(implements (r7rs cons)))
		
		(xcons
			(type procedure)
			(export vs:pairs)
		)
		
		(set-car!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!)
			(export vs:pairs vs:lists)
			(extends (r7rs set-car!))
			(signature
				((pair any) -> any))
		)
		
		(set-cdr!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!)
			(export vs:pairs vs:lists)
			(extends (r7rs set-cdr!))
			(signature
				((pair any) -> any))
		)
		
		(list-ref-cons
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(alias list-tail)
			(implements (r7rs list-tail)))
		
		(list-ref-car
			(category vs:r7rs vs:lists)
			(type accessor)
			(export vs:lists)
			(alias list-ref)
			(implements (r7rs list-ref)))
		
		(list-ref-cdr
			(type procedure)
			(export vs:lists)
		)
		
		
		(memq
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs memq)))
		
		(memv
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs memv)))
		
		(assq
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(export vs:lists)
			(implements (r7rs assq)))
		
		(assv
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(export vs:lists)
			(implements (r7rs assv)))
		
		(find
			(type procedure)
			(export vs:lists)
		)
		
		(list-set-car!
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(alias list-set!)
			(extends (r7rs list-set!))
			(signature
				((list-not-null range-offset any) -> any))
		)
		
		(list-set-cdr!
			(type procedure)
			(export vs:lists)
		)
		
		
		(make-pair
			(type procedure)
			(export vs:pairs)
		)
		
		(make-list
			(category vs:r7rs vs:lists)
			(type constructor)
			(export vs:lists)
			(implements (r7rs make-list)))
		
		(list
			(category vs:r7rs vs:lists)
			(type constructor)
			(export vs:lists)
			(implements (r7rs list)))
		
		(list*
			(type procedure)
			(export vs:lists)
		)
		
		(append
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs append)))
		
		(list-fill!
			(type procedure)
			(export vs:lists)
		)
		
		(list-copy!
			(type procedure)
			(export vs:lists)
		)
		
		(list-copy
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs list-copy)))
		
		(list-reverse!
			(type procedure)
			(export vs:lists)
		)
		
		(member
			(category vs:r7rs vs:lists)
			(type procedure)
			(export vs:lists)
			(implements (r7rs member)))
		
		(assoc
			(category vs:r7rs vs:lists vs:associations)
			(type procedure)
			(export vs:lists)
			(implements (r7rs assoc)))
		
		
		
		
		;; ---- arrays
		
		
		(vector-length
			(category vs:r7rs vs:vectors)
			(type procedure)
			(export vs:arrays)
			(implements (r7rs vector-length)))
		
		(vector-reverse
			(type procedure)
			(export vs:arrays)
		)
		
		(vector->immutable
			(type procedure)
			(export vs:arrays)
		)
		
		(vector->mutable
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-clear!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-ref
			(category vs:r7rs vs:vectors)
			(type accessor)
			(export vs:arrays)
			(implements (r7rs vector-ref)))
		
		(vector-push-from!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-find
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-set!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(export vs:arrays)
			(extends (r7rs vector-set!))
			(signature
				((vector-not-empty range-offset any) -> any))
		)
		
		(vector-insert-from!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-swap!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-clear-at!
			(type procedure)
			(export vs:arrays)
		)
		
		(make-vector
			(category vs:r7rs vs:vectors)
			(type constructor)
			(export vs:arrays)
			(implements (r7rs make-vector)))
		
		(vector
			(category vs:r7rs vs:vectors)
			(type constructor)
			(export vs:arrays)
			(implements (r7rs vector)))
		
		(vector-append
			(category vs:r7rs vs:vectors)
			(type procedure)
			(export vs:arrays)
			(implements (r7rs vector-append)))
		
		(vector-fill!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(export vs:arrays)
			(implements (r7rs vector-fill!)))
		
		(vector-copy!
			(category vs:r7rs vs:vectors)
			(type mutator!)
			(export vs:arrays)
			(implements (r7rs vector-copy!)))
		
		(vector-append!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-copy
			(category vs:r7rs vs:vectors)
			(type accessor)
			(export vs:arrays)
			(implements (r7rs vector-copy)))
		
		(vector-reverse!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector->list
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter)
			(export vs:arrays)
			(implements (r7rs vector->list)))
		
		(list->vector
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter)
			(export vs:arrays)
			(implements (r7rs list->vector)))
		
		(vector-push!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-pop!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-insert!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-remove!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-resize!
			(type procedure)
			(export vs:arrays)
		)
		
		(vector-resize-at!
			(type procedure)
			(export vs:arrays)
		)
		
		
		
		
		;; ---- bytes
		
		
		(bytevector-length
			(category vs:r7rs vs:bytes)
			(type procedure)
			(export vs:bytes)
			(implements (r7rs bytevector-length)))
		
		(bytevector-reverse
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector->immutable
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector->mutable
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector-u8-ref
			(category vs:r7rs vs:bytes)
			(type accessor)
			(export vs:bytes)
			(implements (r7rs bytevector-u8-ref)))
		
		(bytevector-u8-set!
			(category vs:r7rs vs:bytes)
			(type mutator!)
			(export vs:bytes)
			(extends (r7rs bytevector-u8-set!))
			(signature
				((bytevector-not-empty range-offset byte) -> byte))
		)
		
		(make-bytevector
			(category vs:r7rs vs:bytes)
			(type constructor)
			(export vs:bytes)
			(implements (r7rs make-bytevector)))
		
		(bytevector
			(category vs:r7rs vs:bytes)
			(type constructor)
			(export vs:bytes)
			(implements (r7rs bytevector)))
		
		(bytevector-append
			(category vs:r7rs vs:bytes)
			(type procedure)
			(export vs:bytes)
			(implements (r7rs bytevector-append)))
		
		(bytevector-u8-fill!
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector-copy!
			(category vs:r7rs vs:bytes)
			(type procedure!)
			(export vs:bytes)
			(implements (r7rs bytevector-copy!)))
		
		(bytevector-copy
			(category vs:r7rs vs:bytes)
			(type procedure)
			(export vs:bytes)
			(implements (r7rs bytevector-copy)))
		
		(bytevector-reverse!
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector->list
			(type procedure)
			(export vs:bytes)
		)
		
		(list->bytevector
			(type procedure)
			(export vs:bytes)
		)
		
		(bytevector->vector
			(type procedure)
			(export vs:bytes)
		)
		
		(vector->bytevector
			(type procedure)
			(export vs:bytes)
		)
		
		
		
		
		;; ---- bytes regular-expressions
		
		
		(make-bytevector-regex
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match?
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-all
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-all->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-position
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-position-all
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-position-all->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures->assoc
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-all
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-all->assoc
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-all->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position->assoc
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position-all
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position-all->assoc
			(type procedure)
			(export vs:regex-bytes)
		)
		
		(bytevector-regex-match-captures-position-all->vector
			(type procedure)
			(export vs:regex-bytes)
		)
		
		
		
		
		;; ---- string
		
		
		(string-length
			(category vs:r7rs vs:strings)
			(type procedure)
			(export vs:strings)
			(implements (r7rs string-length)))
		
		(string-reverse
			(type procedure)
			(export vs:strings)
		)
		
		(string->symbol
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter)
			(export vs:strings vs:symbols)
			(implements (r7rs string->symbol)))
		
		(symbol->string
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter)
			(export vs:strings vs:symbols)
			(implements (r7rs symbol->string)))
		
		(char->integer
			(category vs:r7rs vs:characters)
			(type converter)
			(export vs:characters)
			(implements (r7rs char->integer)))
		
		(integer->char
			(category vs:r7rs vs:characters)
			(type converter)
			(export vs:characters)
			(implements (r7rs integer->char)))
		
		(string->keyword
			(type procedure)
			(export vs:strings vs:keywords)
		)
		
		(keyword->string
			(type procedure)
			(export vs:strings vs:keywords)
		)
		
		(symbol->keyword
			(type procedure)
			(export vs:strings vs:symbols vs:keywords)
		)
		
		(keyword->symbol
			(type procedure)
			(export vs:strings vs:symbols vs:keywords)
		)
		
		(string-upcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(export vs:strings)
			(implements (r7rs string-upcase)))
		
		(string-downcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(export vs:strings)
			(implements (r7rs string-downcase)))
		
		(string-foldcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure)
			(export vs:strings)
			(implements (r7rs string-foldcase)))
		
		(symbol-upcase
			(type procedure)
			(export vs:strings vs:symbols)
		)
		
		(symbol-downcase
			(type procedure)
			(export vs:strings vs:symbols)
		)
		
		(symbol-foldcase
			(type procedure)
			(export vs:strings vs:symbols)
		)
		
		(char-upcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(export vs:characters)
			(implements (r7rs char-upcase)))
		
		(char-downcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(export vs:characters)
			(implements (r7rs char-downcase)))
		
		(char-foldcase
			(category vs:r7rs vs:characters)
			(type procedure)
			(export vs:characters)
			(implements (r7rs char-foldcase)))
		
		(keyword-upcase
			(type procedure)
			(export vs:strings vs:keywords)
		)
		
		(keyword-downcase
			(type procedure)
			(export vs:strings vs:keywords)
		)
		
		(keyword-foldcase
			(type procedure)
			(export vs:strings vs:keywords)
		)
		
		(string->immutable
			(type procedure)
			(export vs:strings)
		)
		
		(string->mutable
			(type procedure)
			(export vs:strings)
		)
		
		(string-ref
			(category vs:r7rs vs:strings)
			(type accessor)
			(export vs:strings)
			(implements (r7rs string-ref)))
		
		(string-set!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(export vs:strings)
			(extends (r7rs string-set!))
			(signature
				((string-not-empty range-offset character) -> character))
		)
		
		(make-string
			(category vs:r7rs vs:strings)
			(type constructor)
			(export vs:strings)
			(implements (r7rs make-string)))
		
		(string
			(category vs:r7rs vs:strings)
			(type constructor)
			(export vs:strings)
			(implements (r7rs string)))
		
		(string-append
			(category vs:r7rs vs:strings)
			(type constructor)
			(export vs:strings)
			(implements (r7rs string-append)))
		
		(string-fill!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(export vs:strings)
			(implements (r7rs string-fill!)))
		
		(string-copy!
			(category vs:r7rs vs:strings)
			(type mutator!)
			(export vs:strings)
			(implements (r7rs string-copy!)))
		
		(string-copy
			(category vs:r7rs vs:strings)
			(type accessor)
			(export vs:strings)
			(alias substring)
			(implements (r7rs string-copy) (r7rs substring)))
		
		(string-reverse!
			(type procedure)
			(export vs:strings)
		)
		
		(string->list
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs string->list)))
		
		(list->string
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs list->string)))
		
		(string->vector
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs string->vector)))
		
		(vector->string
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs vector->string)))
		
		(string->utf8
			(category vs:r7rs vs:bytes vs:strings)
			(type converter)
			(export vs:strings)
			(implements (r7rs string->utf8)))
		
		(utf8->string
			(category vs:r7rs vs:bytes vs:strings)
			(type converter)
			(export vs:strings)
			(implements (r7rs utf8->string)))
		
		(string->number
			(category vs:r7rs vs:strings vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs string->number)))
		
		(number->string
			(category vs:r7rs vs:strings vs:conversions)
			(type converter)
			(export vs:strings)
			(implements (r7rs number->string)))
		
		(digit-value
			(category vs:r7rs vs:characters)
			(type converter)
			(export vs:characters)
			(implements (r7rs digit-value)))
		
		
		
		
		;; ---- string regular-expressions
		
		
		(make-string-regex
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match?
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-all
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-all->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-position
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-position-all
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-position-all->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures->assoc
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-all
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-all->assoc
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-all->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position->assoc
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position-all
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position-all->assoc
			(type procedure)
			(export vs:regex-strings)
		)
		
		(string-regex-match-captures-position-all->vector
			(type procedure)
			(export vs:regex-strings)
		)
		
		
		
		
		;; ---- functional
		
		
		(identity
			(type procedure)
			(export vs:functional)
		)
		
		(constant-fn
			(type procedure)
			(export vs:functional)
		)
		
		(constant-fn*
			(type procedure)
			(export vs:functional)
		)
		
		(not-fn
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-list*
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-list
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-vector*
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-vector
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-values*
			(type procedure)
			(export vs:functional)
		)
		
		(call-with-values
			(category vs:r7rs vs:functions vs:values)
			(type procedure)
			(export vs:functional)
			(implements (r7rs call-with-values)))
		
		(map-in-order
			(type procedure)
			(export vs:functional vs:lists)
		)
		
		(call
			(type procedure)
			(export vs:core-procedures)
		)
		
		(apply
			(category vs:r7rs vs:functions)
			(type procedure)
			(export vs:core-procedures)
			(implements (r7rs apply)))
		
		(map
			(category vs:r7rs vs:lists vs:functions vs:conversions vs:loops)
			(type map)
			(export vs:functional vs:lists)
			(implements (r7rs map)))
		
		(for-each
			(category vs:r7rs vs:lists vs:functions vs:loops)
			(type for-each)
			(export vs:functional vs:lists)
			(extends (r7rs for-each))
			(signature
				((for-each-procedure list 1...) -> void))
		)
		
		(vector-map
			(category vs:r7rs vs:vectors vs:functions vs:conversions vs:loops)
			(type map)
			(export vs:functional vs:arrays)
			(implements (r7rs vector-map)))
		
		(vector-for-each
			(category vs:r7rs vs:vectors vs:functions vs:loops)
			(type for-each)
			(export vs:functional vs:arrays)
			(extends (r7rs vector-for-each))
			(signature
				((for-each-procedure vector 1...) -> void))
		)
		
		(bytevector-u8-map
			(type map)
			(export vs:functional vs:bytes)
		)
		
		(bytevector-u8-for-each
			(type for-each)
			(export vs:functional vs:bytes)
		)
		
		(string-map
			(category vs:r7rs vs:strings vs:functions vs:conversions vs:loops)
			(type map)
			(export vs:functional vs:strings)
			(implements (r7rs string-map)))
		
		(string-for-each
			(category vs:r7rs vs:strings vs:functions vs:loops)
			(type for-each)
			(export vs:functional vs:strings)
			(extends (r7rs string-for-each))
			(signature
				((for-each-procedure string 1...) -> void))
		)
		
		(values
			(category vs:r7rs vs:functions vs:values)
			(type constructor)
			(export vs:values)
			(implements (r7rs values)))
		
		(curry
			(type procedure)
			(export vs:functional)
		)
		
		(curry-last
			(type procedure)
			(export vs:functional)
		)
		
		(compose
			(type procedure)
			(export vs:functional)
		)
		
		(compose*
			(type procedure)
			(export vs:functional)
		)
		
		
		
		
		;; ---- records
		
		
		(record-type-identifier
			(type procedure)
			(export vs:records)
		)
		
		(record-type-size
			(type procedure)
			(export vs:records)
		)
		
		(record-type
			(type procedure)
			(export vs:records)
		)
		
		(record->immutable
			(type procedure)
			(export vs:records)
		)
		
		(record->mutable
			(type procedure)
			(export vs:records)
		)
		
		(make-record-type
			(type procedure)
			(export vs:records)
		)
		
		(record-type-predicate
			(type procedure)
			(export vs:records)
		)
		
		(record-type-accessor
			(type procedure)
			(export vs:records)
		)
		
		(record-type-mutator
			(type procedure)
			(export vs:records)
		)
		
		(record-type-constructor
			(type procedure)
			(export vs:records)
		)
		
		(record-type-constructor*
			(type procedure)
			(export vs:records)
		)
		
		(record-of?
			(type procedure)
			(export vs:records)
		)
		
		(record-set!
			(type procedure)
			(export vs:records)
		)
		
		(record-ref
			(type procedure)
			(export vs:records)
		)
		
		(make-record
			(type procedure)
			(export vs:records)
		)
		
		(make-record*
			(type procedure)
			(export vs:records)
		)
		
		(record->vector
			(type procedure)
			(export vs:records)
		)
		
		(vector->record
			(type procedure)
			(export vs:records)
		)
		
		(record->values
			(type procedure)
			(export vs:records)
		)
		
		(values->record
			(type procedure)
			(export vs:records)
		)
		
		(record->list
			(type procedure)
			(export vs:records)
		)
		
		(list->record
			(type procedure)
			(export vs:records)
		)
		
		
		
		
		;; ---- runtime
		
		
		(command-line
			(category vs:r7rs)
			(type procedure)
			(export vs:os)
			(implements (r7rs command-line)))
		
		(command-line->vector
			(type procedure)
			(export vs:os)
		)
		
		(command-line-length
			(type procedure)
			(export vs:os)
		)
		
		(get-environment-variables
			(category vs:r7rs)
			(type procedure)
			(export vs:os)
			(implements (r7rs get-environment-variables)))
		
		(get-environment-variables->vector
			(type procedure)
			(export vs:os)
		)
		
		(get-environment-fingerprint
			(type procedure)
			(export vs:os)
		)
		
		
		(current-second
			(category vs:r7rs)
			(type procedure)
			(export vs:time)
			(implements (r7rs current-second)))
		
		(current-jiffy
			(category vs:r7rs)
			(type procedure)
			(export vs:time)
			(implements (r7rs current-jiffy)))
		
		(jiffies-per-second
			(category vs:r7rs)
			(type procedure)
			(export vs:time)
			(implements (r7rs jiffies-per-second)))
		
		
		(raise
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure)
			(export vs:errors)
			(implements (r7rs raise)))
		
		(error-object-message
			(category vs:r7rs vs:errors)
			(type accessor)
			(export vs:errors)
			(implements (r7rs error-object-message)))
		
		(error-object-irritants
			(category vs:r7rs vs:errors)
			(type accessor)
			(export vs:errors)
			(implements (r7rs error-object-irritants)))
		
		(error-object-irritants->vector
			(type procedure)
			(export vs:errors)
		)
		
		(error-object-irritants->values
			(type procedure)
			(export vs:errors)
		)
		
		(command-line-ref
			(type procedure)
			(export vs:os)
		)
		
		(get-environment-variable
			(category vs:r7rs)
			(type procedure)
			(export vs:os)
			(implements (r7rs get-environment-variable)))
		
		(process-wait-poll
			(type procedure)
			(export vs:processes)
		)
		
		(process-wait-try
			(type procedure)
			(export vs:processes)
		)
		
		(process-wait
			(type procedure)
			(export vs:processes)
		)
		
		(process-stdin
			(type procedure)
			(export vs:processes)
		)
		
		(process-stdout
			(type procedure)
			(export vs:processes)
		)
		
		(process-stderr
			(type procedure)
			(export vs:processes)
		)
		
		
		(cache-close
			(type procedure)
			(export vs:cache)
		)
		
		(serialize-bytevector
			(type procedure)
			(export vs:serialization)
		)
		
		(deserialize-bytevector
			(type procedure)
			(export vs:serialization)
		)
		
		(hash
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-sip-unseeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-sea-unseeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-blake2b-unseeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-blake2s-unseeded
			(type procedure)
			(export vs:hashes)
		)
		
		(process-spawn
			(type procedure)
			(export vs:processes)
		)
		
		(process-exec
			(type procedure)
			(export vs:processes)
		)
		
		(process-run-try
			(type procedure)
			(export vs:processes)
		)
		
		(process-run
			(type procedure)
			(export vs:processes)
		)
		
		(error
			(category vs:r7rs vs:errors)
			(type constructor)
			(implements (r7rs error)))
		
		(make-error
			(type procedure)
		)
		
		(make-parameter
			(category vs:r7rs vs:parameters)
			(type constructor)
			(implements (r7rs make-parameter)))
		
		(parameter-ref
			(type procedure)
		)
		
		(parameter-set!
			(type procedure)
		)
		
		(trace-critical
			(type procedure)
		)
		
		(trace-error
			(type procedure)
		)
		
		(trace-warning
			(type procedure)
		)
		
		(trace-notice
			(type procedure)
		)
		
		(trace-information
			(type procedure)
		)
		
		(trace-internal
			(type procedure)
		)
		
		(trace-debugging
			(type procedure)
		)
		
		(abort
			(type procedure)
		)
		
		(pause
			(type procedure)
		)
		
		(exit
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs exit)))
		
		(emergency-exit
			(category vs:r7rs)
			(type procedure)
			(implements (r7rs emergency-exit)))
		
		(process-spawn*
			(type procedure)
			(export vs:processes)
		)
		
		(process-exec*
			(type procedure)
			(export vs:processes)
		)
		
		(cache-open
			(type procedure)
			(export vs:cache)
		)
		
		(cache-select-bytevector
			(type procedure)
			(export vs:cache)
		)
		
		(cache-include-bytevector
			(type procedure)
			(export vs:cache)
		)
		
		(cache-exclude-bytevector
			(type procedure)
			(export vs:cache)
		)
		
		(cache-resolve-bytevector
			(type procedure)
			(export vs:cache)
		)
		
		(cache-select
			(type procedure)
			(export vs:cache)
		)
		
		(cache-include
			(type procedure)
			(export vs:cache)
		)
		
		(cache-exclude
			(type procedure)
			(export vs:cache)
		)
		
		(cache-resolve
			(type procedure)
			(export vs:cache)
		)
		
		(cache-exclude-all
			(type procedure)
			(export vs:cache)
		)
		
		(cache-prune-all
			(type procedure)
			(export vs:cache)
		)
		
		(hash-sip-seeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-sea-seeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-blake2b-seeded
			(type procedure)
			(export vs:hashes)
		)
		
		(hash-blake2s-seeded
			(type procedure)
			(export vs:hashes)
		)
		
		
		
		
		;; ---- ports
		
		
		(current-input-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(export vs:io vs:parameters)
			(implements (r7rs current-input-port)))
		
		(current-output-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(export vs:io vs:parameters)
			(implements (r7rs current-output-port)))
		
		(current-error-port
			(category vs:r7rs vs:parameters)
			(type parameter)
			(export vs:io vs:parameters)
			(implements (r7rs current-error-port)))
		
		(eof-object
			(category vs:r7rs vs:ports vs:globals)
			(type constructor)
			(export vs:io)
			(implements (r7rs eof-object)))
		
		(open-input-bytevector
			(category vs:r7rs vs:ports:input vs:ports:open vs:bytes)
			(type procedure)
			(export vs:io-bytes vs:io-open)
			(implements (r7rs open-input-bytevector)))
		
		(open-input-string
			(category vs:r7rs vs:ports:input vs:ports:open vs:strings)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs open-input-string)))
		
		(get-output-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(implements (r7rs get-output-bytevector)))
		
		(get-output-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure)
			(export vs:io-strings)
			(implements (r7rs get-output-string)))
		
		(open-binary-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure)
			(export vs:io-bytes vs:io-open)
			(implements (r7rs open-binary-input-file)))
		
		(open-binary-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure)
			(export vs:io-bytes vs:io-open)
			(implements (r7rs open-binary-output-file)))
		
		(open-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs open-input-file)))
		
		(open-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs open-output-file)))
		
		(port-descriptor
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(port-descriptor-clone
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(port-descriptor-ref
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(port-descriptor-path
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(port-temporary-release
			(type procedure)
			(export vs:io-temporary)
		)
		
		(port-temporary-path
			(type procedure)
			(export vs:io-temporary)
		)
		
		(call-with-port
			(category vs:r7rs vs:ports vs:functions)
			(type procedure)
			(export vs:io)
			(implements (r7rs call-with-port)))
		
		(call-with-binary-input-file
			(type procedure)
			(export vs:io-bytes vs:io-open)
		)
		
		(call-with-binary-output-file
			(type procedure)
			(export vs:io-bytes vs:io-open)
		)
		
		(call-with-input-file
			(category vs:r7rs vs:ports:input vs:functions)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs call-with-input-file)))
		
		(call-with-output-file
			(category vs:r7rs vs:ports:output vs:functions)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs call-with-output-file)))
		
		(with-binary-input-file
			(type procedure)
			(export vs:io-bytes vs:io-open)
		)
		
		(with-binary-output-file
			(type procedure)
			(export vs:io-bytes vs:io-open)
		)
		
		(with-input-from-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs with-input-from-file)))
		
		(with-output-to-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs with-output-to-file)))
		
		(port-descriptor-flag-ref
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(port-descriptor-flag-set!
			(type procedure)
			(export vs:io-descriptors)
		)
		
		(open-output-bytevector
			(category vs:r7rs vs:ports:output vs:ports:open vs:bytes)
			(type procedure)
			(export vs:io-bytes vs:io-open)
			(implements (r7rs open-output-bytevector)))
		
		(open-output-string
			(category vs:r7rs vs:ports:output vs:ports:open vs:strings)
			(type procedure)
			(export vs:io-strings vs:io-open)
			(implements (r7rs open-output-string)))
		
		(input-port-open?
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type predicate)
			(export vs:io)
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
			(export vs:io)
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
			(export vs:io)
			(implements (r7rs close-port)))
		
		(close-input-port
			(category vs:r7rs vs:ports:input)
			(type procedure)
			(export vs:io)
			(implements (r7rs close-input-port)))
		
		(close-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure)
			(export vs:io)
			(implements (r7rs close-output-port)))
		
		(u8-ready?
			(category vs:r7rs vs:ports:input vs:bytes)
			(type predicate)
			(export vs:io-bytes)
			(implements (r7rs u8-ready?)))
		
		(peek-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(implements (r7rs peek-u8)))
		
		(read-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(implements (r7rs read-u8)))
		
		(char-ready?
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type predicate)
			(export vs:io-strings)
			(implements (r7rs char-ready?)))
		
		(peek-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure)
			(export vs:io-strings)
			(implements (r7rs peek-char)))
		
		(read-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure)
			(export vs:io-strings)
			(implements (r7rs read-char)))
		
		(read-bytevector!
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure!)
			(export vs:io-bytes)
			(implements (r7rs read-bytevector!)))
		
		(read-bytevector-append!
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(implements (r7rs read-bytevector)))
		
		(read-bytevector-chunk
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector-line
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector-zero
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-string-append!
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-string
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure)
			(export vs:io-strings)
			(implements (r7rs read-string)))
		
		(read-string-chunk
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-string-line
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure)
			(export vs:io-strings)
			(alias read-line)
			(implements (r7rs read-line)))
		
		(read-string-zero
			(type procedure)
			(export vs:io-strings)
		)
		
		(read
			(category vs:r7rs vs:ports:input vs:ports:values)
			(type procedure)
			(export vs:io-values)
			(implements (r7rs read)))
		
		(read-bytevector-fold
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector-chunk-fold
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector-line-fold
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-bytevector-zero-fold
			(type procedure)
			(export vs:io-bytes)
		)
		
		(read-string-fold
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-string-chunk-fold
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-string-line-fold
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-string-zero-fold
			(type procedure)
			(export vs:io-strings)
		)
		
		(read-fold
			(type procedure)
			(export vs:io-values)
		)
		
		(write-u8
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(extends (r7rs write-u8))
			(signature
				((byte) -> void)
				((byte binary-output-port-open) -> void))
		)
		
		(write-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure)
			(export vs:io-bytes)
			(implements (r7rs write-bytevector)))
		
		(write-bytevector-line
			(type procedure)
			(export vs:io-bytes)
		)
		
		(write-bytevector-zero
			(type procedure)
			(export vs:io-bytes)
		)
		
		(write-char
			(category vs:r7rs vs:ports:output vs:strings vs:characters)
			(type procedure)
			(export vs:io-strings)
			(extends (r7rs write-char))
			(signature
				((character) -> void)
				((character textual-output-port-open) -> void))
		)
		
		(write-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure)
			(export vs:io-strings)
			(implements (r7rs write-string)))
		
		(write-string-line
			(type procedure)
			(export vs:io-strings)
		)
		
		(write-string-zero
			(type procedure)
			(export vs:io-strings)
		)
		
		(write
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(export vs:io-values)
			(implements (r7rs write)))
		
		(write-shared
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(export vs:io-values)
			(implements (r7rs write-shared)))
		
		(write-simple
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(export vs:io-values)
			(implements (r7rs write-simple)))
		
		(display
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure)
			(export vs:io-values)
			(implements (r7rs display)))
		
		(write-line
			(type procedure)
			(export vs:io-values)
		)
		
		(display-line
			(type procedure)
			(export vs:io-values)
		)
		
		(newline
			(category vs:r7rs vs:ports:output vs:bytes vs:strings)
			(type procedure)
			(export vs:io)
			(implements (r7rs newline)))
		
		(flush-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure)
			(export vs:io)
			(implements (r7rs flush-output-port)))
		
		(open-binary-temporary
			(type procedure)
			(export vs:io-temporary vs:io-bytes vs:io-open)
		)
		
		(open-temporary
			(type procedure)
			(export vs:io-temporary vs:io-strings vs:io-open)
		)
		
		
		
		
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
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:stdout
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:stderr
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:directory
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:env-empty
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:env-include
			(type parameter)
			(export vs:processes)
		)
		
		(process-spawn:env-exclude
			(type parameter)
			(export vs:processes)
		)
		
		
		
		
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
			(export vs:core-syntaxes)
			(implements (r7rs case-lambda)))
		
		(cond-expand
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs cond-expand)))
		
		(define-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs define-syntax)))
		
		(let-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs let-syntax)))
		
		(letrec-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs letrec-syntax)))
		
		(syntax-rules
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs syntax-rules)))
		
		(syntax-error
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs syntax-error)))
		
		(import
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs import)))
		
		(include
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs include)))
		
		(include-ci
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax)
			(export vs:core-syntaxes)
			(implements (r7rs include-ci)))
		
		
		
		
		;; ---- R7RS unimplemented procedures
		
		
		(make-promise
			(category vs:r7rs vs:promises vs:evaluator)
			(type constructor)
			(export vs:promises)
			(implements (r7rs make-promise)))
		
		(force
			(category vs:r7rs vs:promises vs:evaluator)
			(type procedure)
			(export vs:promises)
			(implements (r7rs force)))
		
		(eval
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:evaluator)
			(implements (r7rs eval)))
		
		(environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:evaluator)
			(implements (r7rs environment)))
		
		(null-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:evaluator)
			(implements (r7rs null-environment)))
		
		(interaction-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:evaluator)
			(implements (r7rs interaction-environment)))
		
		(scheme-report-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:evaluator)
			(implements (r7rs scheme-report-environment)))
		
		(load
			(category vs:r7rs vs:compiler vs:unsupported)
			(type procedure)
			(export vs:core-syntaxes)
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
			(export vs:core-procedures)
			(implements (r7rs features)))
		
		
		(rationalize
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs rationalize)))
		
		(numerator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs numerator)))
		
		(denominator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs denominator)))
		
		(make-rectangular
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs make-rectangular)))
		
		(real-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs real-part)))
		
		(imag-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs imag-part)))
		
		(make-polar
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs make-polar)))
		
		(angle
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs angle)))
		
		(magnitude
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure)
			(export vs:arithmetic)
			(implements (r7rs magnitude)))
		
		
		(call-with-current-continuation
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure)
			(export vs:dynamic)
			(alias call/cc)
			(implements (r7rs call-with-current-continuation)))
		
		(dynamic-wind
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure)
			(export vs:dynamic)
			(implements (r7rs dynamic-wind)))
		
		(with-exception-handler
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure)
			(export vs:dynamic vs:errors)
			(implements (r7rs with-exception-handler)))
		
		(raise-continuable
			(category vs:r7rs vs:errors vs:evaluator vs:unsupported)
			(type procedure)
			(export vs:dynamic vs:errors)
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

