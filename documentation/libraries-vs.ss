
(library
	
	
	
	
	(identifier vonuvoli)
	
	(title "Vonuvoli-Scheme builtin functionality")
	
	(description
		#<<<
					
					**FIXME!**
					
		>>>#)
	
	
	
	
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
		
		
		;+ quote
		(quote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax))
		
		;+ quasiquote
		(quasiquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax))
		
		;+ unquote
		(unquote
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax))
		
		;+ unquote-splicing
		(unquote-splicing
			(category vs:r7rs vs:syntaxes vs:quotation)
			(type syntax))
		
		
		;+ _
		(_
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax))
		
		;+ ...
		(...
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax))
		
		;+ =>
		(=>
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax))
		
		;+ else
		(else
			(category vs:r7rs vs:syntaxes)
			(type auxiliary-syntax))
		
		
		;+ begin
		(begin
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ and
		(and
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ or
		(or
			(category vs:r7rs vs:control)
			(type syntax))
		
		
		;+ if
		(if
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ when
		(when
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ unless
		(unless
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ cond
		(cond
			(category vs:r7rs vs:control)
			(type syntax))
		
		;+ case
		(case
			(category vs:r7rs vs:control)
			(type syntax))
		
		
		;+ do
		(do
			(category vs:r7rs vs:control vs:loops)
			(type syntax))
		
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
		
		
		;+ guard
		(guard
			(category vs:r7rs vs:errors vs:evaluator)
			(type syntax))
		
		(guard*
			(category vs:syntaxes)
			(type syntax))
		
		
		(locals
			(category vs:contexts vs:syntaxes)
			(type syntax))
		
		;+ let
		(let
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		;+ let*
		(let*
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		;+ letrec
		(letrec
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		;+ letrec*
		(letrec*
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		;+ let-values
		(let-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax))
		
		;+ let*-values
		(let*-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax))
		
		
		;+ define
		(define
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		(redefine
			(category vs:contexts vs:syntaxes)
			(type syntax))
		
		;+ define-values
		(define-values
			(category vs:r7rs vs:contexts vs:values)
			(type syntax))
		
		(redefine-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax))
		
		;+ set!
		(set!
			(category vs:r7rs vs:contexts)
			(type syntax))
		
		(set!-values
			(category vs:contexts vs:values vs:syntaxes)
			(type syntax))
		
		
		;+ define-record
		(define-record-type
			(category vs:r7rs vs:contexts vs:records)
			(type syntax))
		
		
		;+ lambda
		(lambda
			(category vs:r7rs vs:lambda)
			(type syntax))
		
		
		;+ parameterize
		(parameterize
			(category vs:r7rs vs:parameters)
			(type syntax))
		
		
		
		
		;; ---- types
		
		
		;+ not
		(not
			(category vs:r7rs vs:booleans)
			(type predicate))
		
		
		;+ null?
		(null?
			(category vs:r7rs vs:lists vs:types)
			(type type-predicate))
		
		(void?
			(type type-predicate))
		
		(undefined?
			(type type-predicate))
		
		
		;+ boolean?
		(boolean?
			(category vs:r7rs vs:booleans vs:types)
			(type type-predicate))
		
		(true?
			(type predicate))
		
		(false?
			(type predicate))
		
		(true-or-equivalent?
			(type predicate))
		
		(false-or-equivalent?
			(type predicate))
		
		
		;+ number?
		(number?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ complex?
		(complex?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ real?
		(real?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ rational?
		(rational?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ integer?
		(integer?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ exact-integer?
		(exact-integer?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ exact?
		(exact?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		;+ inexact?
		(inexact?
			(category vs:r7rs vs:arithmetic vs:types)
			(type type-predicate))
		
		
		;+ char?
		(char?
			(category vs:r7rs vs:characters vs:types)
			(type type-predicate))
		
		;+ symbol?
		(symbol?
			(category vs:r7rs vs:symbols vs:types)
			(type type-predicate))
		
		(keyword?
			(type type-predicate))
		
		(unique?
			(type type-predicate))
		
		
		;+ string?
		(string?
			(category vs:r7rs vs:strings vs:types)
			(type type-predicate))
		
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
		
		
		;+ bytevector?
		(bytevector?
			(category vs:r7rs vs:bytes)
			(type type-predicate))
		
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
		
		
		;+ pair?
		(pair?
			(category vs:r7rs vs:pairs vs:lists vs:types)
			(type type-predicate))
		
		(pair-immutable?
			(type type-predicate))
		
		(pair-mutable?
			(type type-predicate))
		
		
		;+ vector?
		(vector?
			(category vs:r7rs vs:vectors vs:types)
			(type type-predicate))
		
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
		
		
		;+ error-object?
		(error-object?
			(category vs:r7rs vs:errors)
			(type type-predicate))
		
		(syntax-error?
			(type predicate))
		
		;+ file-error?
		(file-error?
			(category vs:r7rs vs:errors)
			(type predicate))
		
		(port-error?
			(type type-predicate))
		
		;+ read-error?
		(read-error?
			(category vs:r7rs vs:errors)
			(type predicate))
		
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
		
		;+ list?
		(proper-or-empty-list?
			(category vs:r7rs vs:lists vs:types)
			(type predicate)
			(alias list?))
		
		
		(dotted-list?
			(type predicate))
		
		(dotted-or-empty-list?
			(type predicate))
		
		
		(circular-list?
			(type predicate))
		
		(circular-or-empty-list?
			(type predicate))
		
		
		;+ procedure?
		(procedure?
			(category vs:r7rs vs:functions vs:types)
			(type type-predicate))
		
		(syntax?
			(type predicate))
		
		
		;+ port?
		(port?
			(category vs:r7rs vs:ports vs:types)
			(type type-predicate))
		
		;+ input-port?
		(input-port?
			(category vs:r7rs vs:ports:input)
			(type predicate))
		
		;+ output-port?
		(output-port?
			(category vs:r7rs vs:ports:output)
			(type predicate))
		
		;+ binary-port?
		(binary-port?
			(category vs:r7rs vs:ports)
			(type predicate))
		
		;+ textual-port?
		(textual-port?
			(category vs:r7rs vs:ports)
			(type predicate))
		
		
		(binary-input-port?
			(type predicate))
		
		(textual-input-port?
			(type predicate))
		
		(binary-output-port?
			(type predicate))
		
		(textual-output-port?
			(type predicate))
		
		
		;+ eof-object?
		(eof-object?
			(category vs:r7rs vs:ports vs:globals)
			(type predicate))
		
		
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
		
		;+ promise?
		(promise?
			(category vs:r7rs vs:promises vs:evaluator)
			(type type-predicate))
		
		
		(resource?
			(type type-predicate))
		
		(internal?
			(type type-predicate))
		
		(opaque?
			(type predicate))
		
		
		;+ zero?
		(zero?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ positive?
		(positive?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ negative?
		(negative?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ finite?
		(finite?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ infinite?
		(infinite?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ nan?
		(nan?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ even?
		(even?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		;+ odd?
		(odd?
			(category vs:r7rs vs:arithmetic)
			(type predicate))
		
		
		;+ char-numeric?
		(char-numeric?
			(category vs:r7rs vs:characters)
			(type predicate))
		
		;+ char-alphabetic?
		(char-alphabetic?
			(category vs:r7rs vs:characters)
			(type predicate))
		
		;+ char-upper-case?
		(char-upper-case?
			(category vs:r7rs vs:characters)
			(type predicate))
		
		;+ char-lower-case?
		(char-lower-case?
			(category vs:r7rs vs:characters)
			(type predicate))
		
		(char-alphabetic-or-numeric?
			(type predicate))
		
		;+ char-whitespace?
		(char-whitespace?
			(category vs:r7rs vs:characters)
			(type predicate))
		
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
		
		;+ abs
		(abs
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		(signum
			(type procedure))
		
		;+ floor
		(floor
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ ceiling
		(ceiling
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ round
		(round
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ truncate
		(truncate
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		(fractional
			(type procedure))
		
		;+ exact
		(exact
			(category vs:r7rs vs:arithmetic)
			(type converter))
		
		;+ inexact
		(inexact
			(category vs:r7rs vs:arithmetic)
			(type converter))
		
		;+ square
		(square
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ sqrt
		(sqrt
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ exact-integer-sqrt
		(exact-integer-sqrt
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ exp
		(exp
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ log
		(log
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ sin
		(sin
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ cos
		(cos
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ tan
		(tan
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ asin
		(asin
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ acos
		(acos
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ atan
		(atan
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ floor/
		(floor/
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ floor-quotient
		(floor-quotient
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ floor-remainder
		;+ modulo
		(floor-remainder
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias modulo))
		
		;+ truncate/
		(truncate/
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ truncate-quotient
		;+ quotient
		(truncate-quotient
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias quotient))
		
		;+ truncate-remainder
		;+ remainder
		(truncate-remainder
			(category vs:r7rs vs:arithmetic)
			(type procedure)
			(alias remainder))
		
		;+ expt
		(expt
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ +
		(+
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ -
		(-
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ *
		(*
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ /
		(/
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ gcd
		(gcd
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ lcm
		(lcm
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ min
		(min
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		;+ max
		(max
			(category vs:r7rs vs:arithmetic)
			(type procedure))
		
		
		
		
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
		
		
		;+ eq?
		(equivalent-by-identity?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias eq?))
		
		;+ eqv?
		(equivalent-by-value-strict?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias eqv?))
		
		;+ equal?
		(equivalent-by-value-strict-recursive?
			(category vs:r7rs vs:equivalence)
			(type comparator=)
			(alias equal?))
		
		
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
		
		;+ boolean=?
		(boolean=?
			(category vs:r7rs vs:booleans vs:comparisons vs:equivalence)
			(type comparator=))
		
		(boolean>=?
			(type procedure))
		
		(boolean>?
			(type procedure))
		
		
		;+ <
		(<
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<))
		
		;+ <=
		(<=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator<=))
		
		;+ =
		(=
			(category vs:r7rs vs:arithmetic vs:comparisons vs:equivalence)
			(type comparator=))
		
		;+ >=
		(>=
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>=))
		
		;+ >
		(>
			(category vs:r7rs vs:arithmetic vs:comparisons)
			(type comparator>))
		
		
		;+ char<?
		(char<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<))
		
		;+ char<=?
		(char<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=))
		
		;+ char=?
		(char=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=))
		
		;+ char>=?
		(char>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=))
		
		;+ char>?
		(char>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>))
		
		
		;+ char-ci<?
		(char-ci<?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<))
		
		;+ char-ci<=?
		(char-ci<=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator<=))
		
		;+ char-ci=?
		(char-ci=?
			(category vs:r7rs vs:characters vs:comparisons vs:equivalence)
			(type comparator=))
		
		;+ char-ci>=?
		(char-ci>=?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>=))
		
		;+ char-ci>?
		(char-ci>?
			(category vs:r7rs vs:characters vs:comparisons)
			(type comparator>))
		
		
		;+ string<?
		(string<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<))
		
		;+ string<=?
		(string<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=))
		
		;+ string=?
		(string=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=))
		
		;+ string>=?
		(string>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=))
		
		;+ string>?
		(string>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>))
		
		
		;+ string-ci<?
		(string-ci<?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<))
		
		;+ string-ci<=?
		(string-ci<=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator<=))
		
		;+ string-ci=?
		(string-ci=?
			(category vs:r7rs vs:strings vs:comparisons vs:equivalence)
			(type comparator=))
		
		;+ string-ci>=?
		(string-ci>=?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>=))
		
		;+ string-ci>?
		(string-ci>?
			(category vs:r7rs vs:strings vs:comparisons)
			(type comparator>))
		
		
		(symbol<?
			(type procedure))
		
		(symbol<=?
			(type procedure))
		
		;+ symbol=?
		(symbol=?
			(category vs:r7rs vs:symbols vs:comparisons vs:equivalence)
			(type comparator=))
		
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
		
		
		;+ car
		(car
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		;+ cdr
		(cdr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		;+ caar
		(caar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		;+ cdar
		(cdar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		
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
		
		;+ cadr
		(second
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(alias cadr))
		
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
		
		;+ cddr
		(second-tail
			(category vs:r7rs vs:pairs vs:lists)
			(type procedure)
			(alias cddr))
		
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
		
		
		;+ length
		(length
			(category vs:r7rs vs:lists)
			(type procedure))
		
		;+ reverse
		(reverse
			(category vs:r7rs vs:lists)
			(type procedure))
		
		(pair->immutable
			(type procedure))
		
		(pair->mutable
			(type procedure))
		
		(list->immutable
			(type procedure))
		
		(list->mutable
			(type procedure))
		
		
		;+ cons
		(cons
			(category vs:r7rs vs:pairs vs:lists)
			(type constructor))
		
		(xcons
			(type procedure))
		
		;+ set-car!
		(set-car!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!))
		
		;+ set-cdr!
		(set-cdr!
			(category vs:r7rs vs:pairs vs:lists)
			(type mutator!))
		
		
		;+ list-tail
		(list-ref-cons
			(category vs:r7rs vs:lists)
			(type procedure)
			(alias list-tail))
		
		;+ list-ref
		(list-ref-car
			(category vs:r7rs vs:lists)
			(type accessor)
			(alias list-ref))
		
		(list-ref-cdr
			(type procedure))
		
		
		;+ memq
		(memq
			(category vs:r7rs vs:lists)
			(type procedure))
		
		;+ memv
		(memv
			(category vs:r7rs vs:lists)
			(type procedure))
		
		;+ assq
		(assq
			(category vs:r7rs vs:lists vs:associations)
			(type procedure))
		
		;+ assv
		(assv
			(category vs:r7rs vs:lists vs:associations)
			(type procedure))
		
		(find
			(type procedure))
		
		;+ list-set!
		(list-set-car!
			(category vs:r7rs vs:lists)
			(type procedure)
			(alias list-set!))
		
		(list-set-cdr!
			(type procedure))
		
		
		(make-pair
			(type procedure))
		
		;+ make-list
		(make-list
			(category vs:r7rs vs:lists)
			(type constructor))
		
		;+ list
		(list
			(category vs:r7rs vs:lists)
			(type constructor))
		
		(list*
			(type procedure))
		
		;+ append
		(append
			(category vs:r7rs vs:lists)
			(type procedure))
		
		(list-fill!
			(type procedure))
		
		(list-copy!
			(type procedure))
		
		;+ list-copy
		(list-copy
			(category vs:r7rs vs:lists)
			(type procedure))
		
		(list-reverse!
			(type procedure))
		
		;+ member
		(member
			(category vs:r7rs vs:lists)
			(type procedure))
		
		;+ assoc
		(assoc
			(category vs:r7rs vs:lists vs:associations)
			(type procedure))
		
		
		
		
		;; ---- arrays
		
		
		;+ vector-length
		(vector-length
			(category vs:r7rs vs:vectors)
			(type procedure))
		
		(vector-reverse
			(type procedure))
		
		(vector->immutable
			(type procedure))
		
		(vector->mutable
			(type procedure))
		
		(vector-clear!
			(type procedure))
		
		;+ vector-ref
		(vector-ref
			(category vs:r7rs vs:vectors)
			(type accessor))
		
		(vector-push-from!
			(type procedure))
		
		(vector-find
			(type procedure))
		
		;+ vector-set!
		(vector-set!
			(category vs:r7rs vs:vectors)
			(type mutator!))
		
		(vector-insert-from!
			(type procedure))
		
		(vector-swap!
			(type procedure))
		
		(vector-clear-at!
			(type procedure))
		
		;+ make-vector
		(make-vector
			(category vs:r7rs vs:vectors)
			(type constructor))
		
		;+ vector
		(vector
			(category vs:r7rs vs:vectors)
			(type constructor))
		
		;+ vector-append
		(vector-append
			(category vs:r7rs vs:vectors)
			(type procedure))
		
		;+ vector-fill!
		(vector-fill!
			(category vs:r7rs vs:vectors)
			(type mutator!))
		
		;+ vector-copy!
		(vector-copy!
			(category vs:r7rs vs:vectors)
			(type mutator!))
		
		(vector-append!
			(type procedure))
		
		;+ vector-copy
		(vector-copy
			(category vs:r7rs vs:vectors)
			(type accessor))
		
		(vector-reverse!
			(type procedure))
		
		;+ vector->list
		(vector->list
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter))
		
		;+ list->vector
		(list->vector
			(category vs:r7rs vs:vectors vs:lists vs:conversions)
			(type converter))
		
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
		
		
		;+ bytevector-length
		(bytevector-length
			(category vs:r7rs vs:bytes)
			(type procedure))
		
		(bytevector-reverse
			(type procedure))
		
		(bytevector->immutable
			(type procedure))
		
		(bytevector->mutable
			(type procedure))
		
		;+ bytevector-u8-ref
		(bytevector-u8-ref
			(category vs:r7rs vs:bytes)
			(type accessor))
		
		;+ bytevector-u8-set!
		(bytevector-u8-set!
			(category vs:r7rs vs:bytes)
			(type mutator!))
		
		;+ make-bytevector
		(make-bytevector
			(category vs:r7rs vs:bytes)
			(type constructor))
		
		;+ bytevector
		(bytevector
			(category vs:r7rs vs:bytes)
			(type constructor))
		
		;+ bytevector-append
		(bytevector-append
			(category vs:r7rs vs:bytes)
			(type procedure))
		
		(bytevector-u8-fill!
			(type procedure))
		
		;+ bytevector-copy!
		(bytevector-copy!
			(category vs:r7rs vs:bytes)
			(type procedure!))
		
		;+ bytevector-copy
		(bytevector-copy
			(category vs:r7rs vs:bytes)
			(type procedure))
		
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
		
		
		;+ string-length
		(string-length
			(category vs:r7rs vs:strings)
			(type procedure))
		
		(string-reverse
			(type procedure))
		
		;+ string->symbol
		(string->symbol
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter))
		
		;+ symbol->string
		(symbol->string
			(category vs:r7rs vs:strings vs:symbols vs:conversions)
			(type converter))
		
		;+ char->integer
		(char->integer
			(category vs:r7rs vs:characters)
			(type converter))
		
		;+ integer->char
		(integer->char
			(category vs:r7rs vs:characters)
			(type converter))
		
		(string->keyword
			(type procedure))
		
		(keyword->string
			(type procedure))
		
		(symbol->keyword
			(type procedure))
		
		(keyword->symbol
			(type procedure))
		
		;+ string-upcase
		(string-upcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure))
		
		;+ string-downcase
		(string-downcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure))
		
		;+ string-foldcase
		(string-foldcase
			(category vs:r7rs vs:strings vs:conversions)
			(type procedure))
		
		(symbol-upcase
			(type procedure))
		
		(symbol-downcase
			(type procedure))
		
		(symbol-foldcase
			(type procedure))
		
		;+ char-upcase
		(char-upcase
			(category vs:r7rs vs:characters)
			(type procedure))
		
		;+ char-downcase
		(char-downcase
			(category vs:r7rs vs:characters)
			(type procedure))
		
		;+ char-foldcase
		(char-foldcase
			(category vs:r7rs vs:characters)
			(type procedure))
		
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
		
		;+ string-ref
		(string-ref
			(category vs:r7rs vs:strings)
			(type accessor))
		
		;+ string-set!
		(string-set!
			(category vs:r7rs vs:strings)
			(type mutator!))
		
		;+ make-string
		(make-string
			(category vs:r7rs vs:strings)
			(type constructor))
		
		;+ string
		(string
			(category vs:r7rs vs:strings)
			(type constructor))
		
		;+ string-append
		(string-append
			(category vs:r7rs vs:strings)
			(type constructor))
		
		;+ string-fill!
		(string-fill!
			(category vs:r7rs vs:strings)
			(type mutator!))
		
		;+ string-copy!
		(string-copy!
			(category vs:r7rs vs:strings)
			(type mutator!))
		
		;+ string-copy
		;+ substring
		(string-copy
			(category vs:r7rs vs:strings)
			(type accessor)
			(alias substring))
		
		(string-reverse!
			(type procedure))
		
		;+ string->list
		(string->list
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter))
		
		;+ list->string
		(list->string
			(category vs:r7rs vs:strings vs:lists vs:conversions)
			(type converter))
		
		;+ string->vector
		(string->vector
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter))
		
		;+ vector->string
		(vector->string
			(category vs:r7rs vs:strings vs:vectors vs:conversions)
			(type converter))
		
		;+ string->utf8
		(string->utf8
			(category vs:r7rs vs:bytes vs:strings)
			(type converter))
		
		;+ utf8->string
		(utf8->string
			(category vs:r7rs vs:bytes vs:strings)
			(type converter))
		
		;+ string->number
		(string->number
			(category vs:r7rs vs:strings vs:conversions)
			(type converter))
		
		;+ number->string
		(number->string
			(category vs:r7rs vs:strings vs:conversions)
			(type converter))
		
		;+ digit-value
		(digit-value
			(category vs:r7rs vs:characters)
			(type converter))
		
		
		
		
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
		
		;+ call-with-values
		(call-with-values
			(category vs:r7rs vs:functions vs:values)
			(type procedure))
		
		(map-in-order
			(type procedure))
		
		(call
			(type procedure))
		
		;+ apply
		(apply
			(category vs:r7rs vs:functions)
			(type procedure))
		
		;+ map
		(map
			(category vs:r7rs vs:lists vs:functions vs:conversions vs:loops)
			(type map))
		
		;+ for-each
		(for-each
			(category vs:r7rs vs:lists vs:functions vs:loops)
			(type for-each))
		
		;+ vector-map
		(vector-map
			(category vs:r7rs vs:vectors vs:functions vs:conversions vs:loops)
			(type map))
		
		;+ vector-for-each
		(vector-for-each
			(category vs:r7rs vs:vectors vs:functions vs:loops)
			(type for-each))
		
		(bytevector-u8-map
			(type map))
		
		(bytevector-u8-for-each
			(type for-each))
		
		;+ string-map
		(string-map
			(category vs:r7rs vs:strings vs:functions vs:conversions vs:loops)
			(type map))
		
		;+ string-for-each
		(string-for-each
			(category vs:r7rs vs:strings vs:functions vs:loops)
			(type for-each))
		
		;+ values
		(values
			(category vs:r7rs vs:functions vs:values)
			(type constructor))
		
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
		
		
		;+ command-line
		(command-line
			(category vs:r7rs)
			(type procedure))
		
		(command-line->vector
			(type procedure))
		
		(command-line-length
			(type procedure))
		
		;+ get-environment-variables
		(get-environment-variables
			(category vs:r7rs)
			(type procedure))
		
		(get-environment-variables->vector
			(type procedure))
		
		(get-environment-fingerprint
			(type procedure))
		
		
		;+ current-second
		(current-second
			(category vs:r7rs)
			(type procedure))
		
		;+ current-jiffy
		(current-jiffy
			(category vs:r7rs)
			(type procedure))
		
		;+ jiffies-per-second
		(jiffies-per-second
			(category vs:r7rs)
			(type procedure))
		
		
		;+ raise
		(raise
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure))
		
		;+ error-object-message
		(error-object-message
			(category vs:r7rs vs:errors)
			(type accessor))
		
		;+ error-object-irritants
		(error-object-irritants
			(category vs:r7rs vs:errors)
			(type accessor))
		
		(error-object-irritants->vector
			(type procedure))
		
		(error-object-irritants->values
			(type procedure))
		
		(command-line-ref
			(type procedure))
		
		;+ get-environment-variable
		(get-environment-variable
			(category vs:r7rs)
			(type procedure))
		
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
		
		;+ error
		(error
			(category vs:r7rs vs:errors)
			(type constructor))
		
		(make-error
			(type procedure))
		
		;+ make-parameter
		(make-parameter
			(category vs:r7rs vs:parameters)
			(type constructor))
		
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
		
		;+ exit
		(exit
			(category vs:r7rs)
			(type procedure))
		
		;+ emergency-exit
		(emergency-exit
			(category vs:r7rs)
			(type procedure))
		
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
		
		
		;+ current-input-port
		(current-input-port
			(category vs:r7rs vs:parameters)
			(type parameter))
		
		;+ current-output-port
		(current-output-port
			(category vs:r7rs vs:parameters)
			(type parameter))
		
		;+ current-error-port
		(current-error-port
			(category vs:r7rs vs:parameters)
			(type parameter))
		
		;+ eof-object
		(eof-object
			(category vs:r7rs vs:ports vs:globals)
			(type constructor))
		
		;+ open-input-bytevector
		(open-input-bytevector
			(category vs:r7rs vs:ports:input vs:ports:open vs:bytes)
			(type procedure))
		
		;+ open-input-string
		(open-input-string
			(category vs:r7rs vs:ports:input vs:ports:open vs:strings)
			(type procedure))
		
		;+ get-output-bytevector
		(get-output-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure))
		
		;+ get-output-string
		(get-output-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure))
		
		;+ open-binary-input-file
		(open-binary-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure))
		
		;+ open-binary-output-file
		(open-binary-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure))
		
		;+ open-input-file
		(open-input-file
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type procedure))
		
		;+ open-output-file
		(open-output-file
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type procedure))
		
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
		
		;+ call-with-port
		(call-with-port
			(category vs:r7rs vs:ports vs:functions)
			(type procedure))
		
		(call-with-binary-input-file
			(type procedure))
		
		(call-with-binary-output-file
			(type procedure))
		
		;+ call-with-input-file
		(call-with-input-file
			(category vs:r7rs vs:ports:input vs:functions)
			(type procedure))
		
		;+ call-with-output-file
		(call-with-output-file
			(category vs:r7rs vs:ports:output vs:functions)
			(type procedure))
		
		(with-binary-input-file
			(type procedure))
		
		(with-binary-output-file
			(type procedure))
		
		;+ with-input-from-file
		(with-input-from-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure))
		
		;+ with-output-to-file
		(with-output-to-file
			(category vs:r7rs vs:parameters vs:functions)
			(type procedure))
		
		(port-descriptor-flag-ref
			(type procedure))
		
		(port-descriptor-flag-set!
			(type procedure))
		
		;+ open-output-bytevector
		(open-output-bytevector
			(category vs:r7rs vs:ports:output vs:ports:open vs:bytes)
			(type procedure))
		
		;+ open-output-string
		(open-output-string
			(category vs:r7rs vs:ports:output vs:ports:open vs:strings)
			(type procedure))
		
		;+ input-port-open?
		(input-port-open?
			(category vs:r7rs vs:ports:input vs:ports:open)
			(type predicate))
		
		;+ output-port-open?
		(output-port-open?
			(category vs:r7rs vs:ports:output vs:ports:open)
			(type predicate))
		
		;+ close-port
		(close-port
			(category vs:r7rs vs:ports)
			(type procedure))
		
		;+ close-input-port
		(close-input-port
			(category vs:r7rs vs:ports:input)
			(type procedure))
		
		;+ close-output-port
		(close-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure))
		
		;+ u8-ready?
		(u8-ready?
			(category vs:r7rs vs:ports:input vs:bytes)
			(type predicate))
		
		;+ peek-u8
		(peek-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure))
		
		;+ read-u8
		(read-u8
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure))
		
		;+ char-ready?
		(char-ready?
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type predicate))
		
		;+ peek-char
		(peek-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure))
		
		;+ read-char
		(read-char
			(category vs:r7rs vs:ports:input vs:strings vs:characters)
			(type procedure))
		
		;+ read-bytevector!
		(read-bytevector!
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure!))
		
		(read-bytevector-append!
			(type procedure))
		
		;+ read-bytevector
		(read-bytevector
			(category vs:r7rs vs:ports:input vs:bytes)
			(type procedure))
		
		(read-bytevector-chunk
			(type procedure))
		
		(read-bytevector-line
			(type procedure))
		
		(read-bytevector-zero
			(type procedure))
		
		(read-string-append!
			(type procedure))
		
		;+ read-string
		(read-string
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure))
		
		(read-string-chunk
			(type procedure))
		
		;+ read-line
		(read-string-line
			(category vs:r7rs vs:ports:input vs:strings)
			(type procedure)
			(alias read-line))
		
		(read-string-zero
			(type procedure))
		
		;+ read
		(read
			(category vs:r7rs vs:ports:input vs:ports:values)
			(type procedure))
		
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
		
		;+ write-u8
		(write-u8
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure))
		
		;+ write-bytevector
		(write-bytevector
			(category vs:r7rs vs:ports:output vs:bytes)
			(type procedure))
		
		(write-bytevector-line
			(type procedure))
		
		(write-bytevector-zero
			(type procedure))
		
		;+ write-char
		(write-char
			(category vs:r7rs vs:ports:output vs:strings vs:characters)
			(type procedure))
		
		;+ write-string
		(write-string
			(category vs:r7rs vs:ports:output vs:strings)
			(type procedure))
		
		(write-string-line
			(type procedure))
		
		(write-string-zero
			(type procedure))
		
		;+ write
		(write
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure))
		
		;+ write-shared
		(write-shared
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure))
		
		;+ write-simple
		(write-simple
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure))
		
		;+ display
		(display
			(category vs:r7rs vs:ports:output vs:ports:values)
			(type procedure))
		
		(write-line
			(type procedure))
		
		(display-line
			(type procedure))
		
		;+ newline
		(newline
			(category vs:r7rs vs:ports:output vs:bytes vs:strings)
			(type procedure))
		
		;+ flush-output-port
		(flush-output-port
			(category vs:r7rs vs:ports:output)
			(type procedure))
		
		(open-binary-temporary
			(type procedure))
		
		(open-temporary
			(type procedure))
		
		
		
		
		;; ---- file-system
		
		
		;+ file-exists?
		(file-exists?
			(category vs:r7rs vs:file-system)
			(type procedure))
		
		;+ delete-file
		(fs-file-delete
			(category vs:r7rs vs:file-system)
			(type procedure)
			(alias delete-file))
		
		
		
		
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
			(type accessor))
		
		(caaaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caaadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caaddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cadaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cadadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(caddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cadddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdaaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdaadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdadar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdaddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cddaar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cddadr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cdddar
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		(cddddr
			(category vs:r7rs vs:pairs vs:lists)
			(type accessor))
		
		
		
		
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
		
		
		;+ delay
		(delay
			(category vs:r7rs vs:promises vs:evaluator)
			(type syntax))
		
		;+ delay-force
		(delay-force
			(category vs:r7rs vs:promises vs:evaluator)
			(type syntax))
		
		
		
		
		;; ---- R7RS unsupported syntaxes
		
		
		;+ case-lambda
		(case-lambda
			(category vs:r7rs vs:lambda)
			(type syntax))
		
		;+ cond-expand
		(cond-expand
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax))
		
		;+ define-syntax
		(define-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax))
		
		;+ let-syntax
		(let-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax))
		
		;+ letrec-syntax
		(letrec-syntax
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax))
		
		;+ syntax-rules
		(syntax-rules
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax))
		
		;+ syntax-error
		(syntax-error
			(category vs:r7rs vs:syntaxes vs:unsupported)
			(type syntax))
		
		;+ import
		(import
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax))
		
		;+ include
		(include
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax))
		
		;+ include-ci
		(include-ci
			(category vs:r7rs vs:compiler vs:unsupported)
			(type syntax))
		
		
		
		
		;; ---- R7RS unimplemented procedures
		
		
		;+ make-promise
		(make-promise
			(category vs:r7rs vs:promises vs:evaluator)
			(type constructor))
		
		;+ force
		(force
			(category vs:r7rs vs:promises vs:evaluator)
			(type procedure))
		
		;+ eval
		(eval
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure))
		
		;+ environment
		(environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure))
		
		;+ null-environment
		(null-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure))
		
		;+ interaction-environment
		(interaction-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure))
		
		;+ scheme-report-environment
		(scheme-report-environment
			(category vs:r7rs vs:evaluator vs:unsupported)
			(type procedure))
		
		;+ load
		(load
			(category vs:r7rs vs:compiler vs:unsupported)
			(type procedure))
		
		
		
		
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
		
		
		;+ features
		(features
			(category vs:r7rs vs:evaluator vs:compiler vs:unsupported)
			(type procedure))
		
		
		;+ rationalize
		(rationalize
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ numerator
		(numerator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ denominator
		(denominator
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ make-rectangular
		(make-rectangular
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ real-part
		(real-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ imag-part
		(imag-part
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ make-polar
		(make-polar
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ angle
		(angle
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		;+ magnitude
		(magnitude
			(category vs:r7rs vs:arithmetic vs:unsupported)
			(type procedure))
		
		
		;+ call-with-current-continuation
		;+ call/cc
		(call-with-current-continuation
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure)
			(alias call/cc))
		
		;+ dynamic-wind
		(dynamic-wind
			(category vs:r7rs vs:continuations vs:unsupported)
			(type procedure))
		
		;+ with-exception-handler
		(with-exception-handler
			(category vs:r7rs vs:errors vs:evaluator)
			(type procedure))
		
		;+ raise-continuable
		(raise-continuable
			(category vs:r7rs vs:errors vs:evaluator vs:unsupported)
			(type procedure))
		
		
		
		
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

