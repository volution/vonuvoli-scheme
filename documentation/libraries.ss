
(library
	
	;! (identifier (vonuvoli r7rs))
	(identifier vonuvoli::r7rs)
	
	(title "R7RS functionality with Vonuvoli-Scheme extensions")
	
	(categories
		
		(r7rs)
		(r7rs:base (parent r7rs))
		(r7rs:case-lambda (parent r7rs))
		(r7rs:char (parent r7rs))
		(r7rs:complex (parent r7rs))
		(r7rs:cxr (parent r7rs))
		(r7rs:eval (parent r7rs))
		(r7rs:file (parent r7rs))
		(r7rs:inexact (parent r7rs))
		(r7rs:lazy (parent r7rs))
		(r7rs:load (parent r7rs))
		(r7rs:process-context (parent r7rs))
		(r7rs:r5rs (parent r7rs))
		(r7rs:read (parent r7rs))
		(r7rs:repl (parent r7rs))
		(r7rs:time (parent r7rs))
		(r7rs:write (parent r7rs))
		
		(arithmetic)
		(associations)
		(bytes)
		(booleans)
		(conversions)
		(globals)
		(file-system)
		(characters)
		(comparisons)
		(compiler)
		(contexts)
		(continuations)
		(control)
		(equivalence)
		(errors)
		(evaluator)
		(functions)
		(lambda)
		(lists)
		(loops)
		(modules)
		(pairs)
		(parameters)
		(ports)
		(ports:input (parent ports))
		(ports:output (parent ports))
		(ports:open (parent ports))
		(ports:values (parent ports))
		(promises)
		(quotation)
		(records)
		(strings)
		(symbols)
		(syntaxes)
		(system)
		(types)
		(unimplemented)
		(unsupported)
		(values)
		(vectors)
		
	)
	
	(definitions
		
		
		
		
		(define-syntax (category r7rs:base syntaxes unsupported) (type syntax))
		(let-syntax (category r7rs:base syntaxes unsupported) (type syntax))
		(letrec-syntax (category r7rs:base syntaxes unsupported) (type syntax))
		(syntax-rules (category r7rs:base syntaxes unsupported) (type syntax))
		(syntax-error (category r7rs:base syntaxes unsupported) (type syntax))
		
		(_ (category r7rs:base syntaxes) (type auxiliary-syntax))
		(... (category r7rs:base syntaxes) (type auxiliary-syntax))
		(=> (category r7rs:base syntaxes) (type auxiliary-syntax))
		(else (category r7rs:base syntaxes) (type auxiliary-syntax))
		
		(quote (category r7rs:base syntaxes quotation) (type syntax))
		(quasiquote (category r7rs:base syntaxes quotation) (type syntax))
		(unquote (category r7rs:base syntaxes quotation) (type syntax))
		(unquote-splicing (category r7rs:base syntaxes quotation) (type syntax))
		
		
		
		
		(lambda (category r7rs:base lambda) (type syntax))
		(case-lambda (category r7rs:case-lambda lambda) (type syntax))
		
		
		
		
		(define (category r7rs:base contexts) (type syntax))
		(let (category r7rs:base contexts) (type syntax))
		(let* (category r7rs:base contexts) (type syntax))
		(letrec (category r7rs:base contexts) (type syntax))
		(letrec* (category r7rs:base contexts) (type syntax))
		(set! (category r7rs:base contexts) (type syntax))
		
		(define-values (category r7rs:base contexts values) (type syntax))
		(let-values (category r7rs:base contexts values) (type syntax))
		(let*-values (category r7rs:base contexts values) (type syntax))
		
		(define-record-type (category r7rs:base contexts records) (type syntax))
		
		
		
		
		(begin (category r7rs:base control) (type syntax))
		
		(and (category r7rs:base control) (type syntax))
		(or (category r7rs:base control) (type syntax))
		
		(if (category r7rs:base control) (type syntax))
		(unless (category r7rs:base control) (type syntax))
		(when (category r7rs:base control) (type syntax))
		
		(cond (category r7rs:base control) (type syntax))
		(case (category r7rs:base control) (type syntax))
		
		(do (category r7rs:base control loops) (type syntax))
		
		
		
		
		(eq? (category r7rs:base equivalence) (type comparator=))
		(eqv? (category r7rs:base equivalence) (type comparator=))
		(equal? (category r7rs:base equivalence) (type comparator=))
		
		(not (category r7rs:base equivalence) (type predicate))
		
		
		
		
		(boolean? (category r7rs:base booleans types) (type type-predicate))
		(boolean=? (category r7rs:base booleans comparisons equivalence) (type comparator=))
		
		
		(symbol? (category r7rs:base symbols types) (type type-predicate))
		(symbol=? (category r7rs:base symbols comparisons equivalence) (type comparator=))
		
		
		
		
		(number? (category r7rs:base arithmetic types) (type type-predicate))
		(integer? (category r7rs:base arithmetic types) (type type-predicate))
		(real? (category r7rs:base arithmetic types) (type type-predicate))
		(rational? (category r7rs:base arithmetic types) (type type-predicate))
		(complex? (category r7rs:base arithmetic types) (type type-predicate))
		
		(exact? (category r7rs:base arithmetic types) (type type-predicate))
		(inexact? (category r7rs:base arithmetic types) (type type-predicate))
		(exact-integer? (category r7rs:base arithmetic types) (type type-predicate))
		
		(zero? (category r7rs:base arithmetic) (type predicate))
		(positive? (category r7rs:base arithmetic) (type predicate))
		(odd? (category r7rs:base arithmetic) (type predicate))
		(even? (category r7rs:base arithmetic) (type predicate))
		
		(= (category r7rs:base arithmetic comparisons) (type comparator=))
		(< (category r7rs:base arithmetic comparisons) (type comparator<))
		(> (category r7rs:base arithmetic comparisons) (type comparator>))
		(<= (category r7rs:base arithmetic comparisons) (type comparator<=))
		(>= (category r7rs:base arithmetic comparisons) (type comparator>=))
		
		(+ (category r7rs:base arithmetic) (type procedure))
		(- (category r7rs:base arithmetic) (type procedure))
		(* (category r7rs:base arithmetic) (type procedure))
		(/ (category r7rs:base arithmetic) (type procedure))
		
		(abs (category r7rs:base arithmetic) (type procedure))
		
		(floor/ (category r7rs:base arithmetic) (type procedure))
		(floor-quotient (category r7rs:base arithmetic) (type procedure))
		(floor-remainder (category r7rs:base arithmetic) (type procedure) (alias modulo))
		(truncate/ (category r7rs:base arithmetic) (type procedure))
		(truncate-quotient (category r7rs:base arithmetic) (type procedure) (alias quotient))
		(truncate-remainder (category r7rs:base arithmetic) (type procedure) (alias remainder))
		
		(floor (category r7rs:base arithmetic) (type procedure))
		(ceiling (category r7rs:base arithmetic) (type procedure))
		(truncate (category r7rs:base arithmetic) (type procedure))
		(round (category r7rs:base arithmetic) (type procedure))
		
		(min (category r7rs:base arithmetic) (type procedure))
		(max (category r7rs:base arithmetic) (type procedure))
		(gcd (category r7rs:base arithmetic) (type procedure))
		(lcm (category r7rs:base arithmetic) (type procedure))
		
		(expt (category r7rs:base arithmetic) (type procedure))
		(square (category r7rs:base arithmetic) (type procedure))
		(exact-integer-sqrt (category r7rs:base arithmetic) (type procedure))
		
		(rationalize (category r7rs:base arithmetic unsupported) (type procedure))
		(numerator (category r7rs:base arithmetic unsupported) (type procedure))
		(denominator (category r7rs:base arithmetic unsupported) (type procedure))
		
		(inexact (category r7rs:complex arithmetic) (type converter))
		(exact (category r7rs:complex arithmetic) (type converter))
		
		(make-rectangular (category r7rs:complex arithmetic unsupported) (type procedure))
		(real-part (category r7rs:complex arithmetic unsupported) (type procedure))
		(imag-part (category r7rs:complex arithmetic unsupported) (type procedure))
		(make-polar (category r7rs:complex arithmetic unsupported) (type procedure))
		(magnitude (category r7rs:complex arithmetic unsupported) (type procedure))
		(angle (category r7rs:complex arithmetic unsupported) (type procedure))
		
		(sqrt (category r7rs:inexact arithmetic) (type procedure))
		(exp (category r7rs:inexact arithmetic) (type procedure))
		(log (category r7rs:inexact arithmetic) (type procedure))
		
		(sin (category r7rs:inexact arithmetic) (type procedure))
		(cos (category r7rs:inexact arithmetic) (type procedure))
		(tan (category r7rs:inexact arithmetic) (type procedure))
		(asin (category r7rs:inexact arithmetic) (type procedure))
		(acos (category r7rs:inexact arithmetic) (type procedure))
		(atan (category r7rs:inexact arithmetic) (type procedure))
		
		(finite? (category r7rs:inexact arithmetic) (type predicate))
		(infinite? (category r7rs:inexact arithmetic) (type predicate))
		(nan? (category r7rs:inexact arithmetic) (type predicate))
		
		
		
		
		(pair? (category r7rs:base pairs lists types) (type type-predicate))
		(cons (category r7rs:base pairs lists) (type constructor))
		(car (category r7rs:base pairs lists) (type accessor))
		(cdr (category r7rs:base pairs lists) (type accessor))
		(set-car! (category r7rs:base pairs lists) (type mutator!))
		(set-cdr! (category r7rs:base pairs lists) (type mutator!))
		
		(caar (category r7rs:base pairs lists) (type accessor))
		(cadr (category r7rs:base pairs lists) (type accessor))
		
		(cdar (category r7rs:base pairs lists) (type accessor))
		(cddr (category r7rs:base pairs lists) (type accessor))
		
		(caaar (category r7rs:cxr pairs lists) (type accessor))
		(caadr (category r7rs:cxr pairs lists) (type accessor))
		(cadar (category r7rs:cxr pairs lists) (type accessor))
		(caddr (category r7rs:cxr pairs lists) (type accessor))
		
		(cdaar (category r7rs:cxr pairs lists) (type accessor))
		(cdadr (category r7rs:cxr pairs lists) (type accessor))
		(cddar (category r7rs:cxr pairs lists) (type accessor))
		(cdddr (category r7rs:cxr pairs lists) (type accessor))
		
		(caaaar (category r7rs:cxr pairs lists) (type accessor))
		(caaadr (category r7rs:cxr pairs lists) (type accessor))
		(caadar (category r7rs:cxr pairs lists) (type accessor))
		(caaddr (category r7rs:cxr pairs lists) (type accessor))
		(cadaar (category r7rs:cxr pairs lists) (type accessor))
		(cadadr (category r7rs:cxr pairs lists) (type accessor))
		(caddar (category r7rs:cxr pairs lists) (type accessor))
		(cadddr (category r7rs:cxr pairs lists) (type accessor))
		
		(cdaaar (category r7rs:cxr pairs lists) (type accessor))
		(cdaadr (category r7rs:cxr pairs lists) (type accessor))
		(cdadar (category r7rs:cxr pairs lists) (type accessor))
		(cdaddr (category r7rs:cxr pairs lists) (type accessor))
		(cddaar (category r7rs:cxr pairs lists) (type accessor))
		(cddadr (category r7rs:cxr pairs lists) (type accessor))
		(cdddar (category r7rs:cxr pairs lists) (type accessor))
		(cddddr (category r7rs:cxr pairs lists) (type accessor))
		
		
		
		(null? (category r7rs:base lists types) (type type-predicate))
		(list? (category r7rs:base lists types) (type type-predicate))
		
		(list (category r7rs:base lists) (type constructor))
		(make-list (category r7rs:base lists) (type constructor))
		
		(length (category r7rs:base lists) (type procedure))
		(append (category r7rs:base lists) (type procedure))
		(list-copy (category r7rs:base lists) (type procedure))
		(reverse (category r7rs:base lists) (type procedure))
		
		(list-ref (category r7rs:base lists) (type accessor))
		(list-tail (category r7rs:base lists) (type accessor))
		(list-set! (category r7rs:base lists) (type mutator!))
		
		(member (category r7rs:base lists) (type procedure))
		(memq (category r7rs:base lists) (type procedure))
		(memv (category r7rs:base lists) (type procedure))
		
		(assoc (category r7rs:base lists associations) (type procedure))
		(assqc (category r7rs:base lists associations) (type procedure))
		(assvc (category r7rs:base lists associations) (type procedure))
		
		(map (category r7rs:base lists functions conversions loops) (type map))
		(for-each (category r7rs:base lists functions loops) (type for-each))
		
		
		
		
		(vector? (category r7rs:base vectors types) (type type-predicate))
		
		(vector (category r7rs:base vectors) (type constructor))
		(make-vector (category r7rs:base vectors) (type constructor))
		
		(vector-length (category r7rs:base vectors) (type procedure))
		(vector-append (category r7rs:base vectors) (type procedure))
		(vector-copy (category r7rs:base vectors) (type accessor))
		(vector-copy! (category r7rs:base vectors) (type mutator!))
		(vector-fill! (category r7rs:base vectors) (type mutator!))
		
		(vector-ref (category r7rs:base vectors) (type accessor))
		(vector-set! (category r7rs:base vectors) (type mutator!))
		
		(vector->list (category r7rs:base vectors lists conversions) (type converter))
		(list->vector (category r7rs:base vectors lists conversions) (type converter))
		
		(vector-map (category r7rs:base vectors functions conversions loops) (type map))
		(vector-for-each (category r7rs:base vectors functions loops) (type for-each))
		
		
		
		
		(string? (category r7rs:base strings types) (type type-predicate))
		
		(string (category r7rs:base strings) (type constructor))
		(make-string (category r7rs:base strings) (type constructor))
		
		(string-length (category r7rs:base strings) (type procedure))
		(string-append (category r7rs:base strings) (type constructor))
		(string-copy (category r7rs:base strings) (type accessor))
		(string-copy! (category r7rs:base strings) (type mutator!))
		(string-fill! (category r7rs:base strings) (type mutator!))
		(substring  (category r7rs:base strings) (type accessor))
		
		(string-ref (category r7rs:base strings) (type accessor))
		(string-set! (category r7rs:base strings) (type mutator!))
		
		(string=? (category r7rs:base strings comparisons equivalence) (type comparator=))
		(string<? (category r7rs:base strings comparisons equivalence) (type comparator<))
		(string>? (category r7rs:base strings comparisons equivalence) (type comparator>))
		(string<=? (category r7rs:base strings comparisons equivalence) (type comparator<=))
		(string>=? (category r7rs:base strings comparisons equivalence) (type comparator>=))
		
		(string-ci=? (category r7rs:char strings comparisons equivalence) (type comparator=))
		(string-ci<? (category r7rs:char strings comparisons) (type comparator<))
		(string-ci>? (category r7rs:char strings comparisons) (type comparator>))
		(string-ci<=? (category r7rs:char strings comparisons) (type comparator<=))
		(string-ci>=? (category r7rs:char strings comparisons) (type comparator>=))
		
		(number->string (category r7rs:base strings conversions) (type converter))
		(string->number (category r7rs:base strings conversions) (type converter))
		
		(symbol->string (category r7rs:base strings symbols conversions) (type converter))
		(string->symbol (category r7rs:base strings symbols conversions) (type converter))
		
		(list->string (category r7rs:base strings lists conversions) (type converter))
		(string->list (category r7rs:base strings lists conversions) (type converter))
		
		(vector->string (category r7rs:base strings vectors conversions) (type converter))
		(string->vector (category r7rs:base strings vectors conversions) (type converter))
		
		(string-map (category r7rs:base strings functions conversions loops) (type map))
		(string-for-each (category r7rs:base strings functions loops) (type for-each))
		
		(string-upcase (category r7rs:char strings conversions) (type procedure))
		(string-downcase (category r7rs:char strings conversions) (type procedure))
		(string-foldcase (category r7rs:char strings conversions) (type procedure))
		
		
		
		
		(bytevector? (category r7rs:base bytes) (type type-predicate))
		
		(bytevector (category r7rs:base bytes) (type constructor))
		(make-bytevector (category r7rs:base bytes) (type constructor))
		
		(bytevector-length (category r7rs:base bytes) (type procedure))
		(bytevector-append (category r7rs:base bytes) (type procedure))
		(bytevector-copy (category r7rs:base bytes) (type procedure))
		(bytevector-copy! (category r7rs:base bytes) (type procedure!))
		
		(bytevector-u8-ref (category r7rs:base bytes) (type accessor))
		(bytevector-u8-set! (category r7rs:base bytes) (type mutator!))
		
		(utf8->string (category r7rs:base bytes strings) (type converter))
		(string->utf8 (category r7rs:base bytes strings) (type converter))
		
		
		
		
		(port? (category r7rs:base ports types) (type type-predicate))
		
		(binary-port? (category r7rs:base ports) (type predicate))
		(textual-port? (category r7rs:base ports) (type predicate))
		
		(input-port? (category r7rs:base ports:input) (type predicate))
		(input-port-open? (category r7rs:base ports:input ports:open) (type predicate))
		
		(output-port? (category r7rs:base ports:output) (type predicate))
		(output-port-open? (category r7rs:base ports:output ports:open) (type predicate))
		
		
		(open-input-bytevector (category r7rs:base ports:input ports:open bytes) (type procedure))
		(open-output-bytevector (category r7rs:base ports:output ports:open bytes) (type procedure))
		(get-output-bytevector (category r7rs:base ports:output bytes) (type procedure))
		
		(open-input-string (category r7rs:base ports:input ports:open strings) (type procedure))
		(open-output-string (category r7rs:base ports:output ports:open strings) (type procedure))
		(get-output-string (category r7rs:base ports:output strings) (type procedure))
		
		
		(close-port (category r7rs:base ports) (type procedure))
		(close-input-port (category r7rs:base ports:input) (type procedure))
		(close-output-port (category r7rs:base ports:output) (type procedure))
		
		
		(u8-ready? (category r7rs:base ports:input bytes) (type predicate))
		(peek-u8 (category r7rs:base ports:input bytes) (type procedure))
		(read-u8 (category r7rs:base ports:input bytes) (type procedure))
		(write-u8 (category r7rs:base ports:output bytes) (type procedure))
		
		(read-bytevector (category r7rs:base ports:input bytes) (type procedure))
		(read-bytevector! (category r7rs:base ports:input bytes) (type procedure!))
		(write-bytevector (category r7rs:base ports:output bytes) (type procedure))
		
		
		(char-ready? (category r7rs:base ports:input strings characters) (type predicate))
		(peek-char (category r7rs:base ports:input strings characters) (type procedure))
		(read-char (category r7rs:base ports:input strings characters) (type procedure))
		(write-char (category r7rs:base ports:output strings characters) (type procedure))
		
		(read-string (category r7rs:base ports:input strings) (type procedure))
		(read-line (category r7rs:base ports:input strings) (type procedure))
		
		
		(newline (category r7rs:base ports:output bytes strings) (type procedure))
		(flush-output-port (category r7rs:base ports:output) (type procedure))
		
		
		(read (category r7rs:read ports:input ports:values) (type procedure))
		(write (category r7rs:write ports:output ports:values) (type procedure))
		(write-simple (category r7rs:write ports:output ports:values) (type procedure))
		(write-shared (category r7rs:write ports:output ports:values) (type procedure))
		(display (category r7rs:write ports:output ports:values) (type procedure))
		
		
		(open-input-file (category r7rs:file ports:input ports:open) (type procedure))
		(open-binary-input-file (category r7rs:file ports:input ports:open) (type procedure))
		(open-output-file (category r7rs:file ports:output ports:open) (type procedure))
		(open-binary-output-file (category r7rs:file ports:output ports:open) (type procedure))
		
		(call-with-port (category r7rs:base ports functions) (type procedure))
		(call-with-input-file (category r7rs:file ports:input functions) (type procedure))
		(call-with-output-file (category r7rs:file ports:output functions) (type procedure))
		
		
		(eof-object (category r7rs:base ports globals) (type constant))
		(eof-object? (category r7rs:base ports globals) (type predicate))
		
		
		
		
		(file-exists? (category r7rs:file file-system) (type procedure))
		(delete-file (category r7rs:file file-system) (type procedure))
		
		
		
		
		(exit (category r7rs:process-context) (type procedure))
		(emergency-exit (category r7rs:process-context) (type procedure))
		
		(command-line (category r7rs:process-context) (type procedure))
		(get-environment-variable (category r7rs:process-context) (type procedure))
		(get-environment-variables (category r7rs:process-context) (type procedure))
		
		(current-second (category r7rs:time) (type procedure))
		(current-jiffy (category r7rs:time) (type procedure))
		(jiffies-per-second (category r7rs:time) (type procedure))
		
		
		
		
		(char? (category r7rs:base characters types) (type type-predicate))
		
		(char=? (category r7rs:base characters comparisons equivalence) (type comparator=))
		(char<? (category r7rs:base characters comparisons) (type comparator<))
		(char>? (category r7rs:base characters comparisons) (type comparator>))
		(char<=? (category r7rs:base characters comparisons) (type comparator<=))
		(char>=? (category r7rs:base characters comparisons) (type comparator>=))
		
		(char-ci=? (category r7rs:char characters comparisons equivalence) (type comparator=))
		(char-ci<? (category r7rs:char characters comparisons) (type comparator<))
		(char-ci>? (category r7rs:char characters comparisons) (type comparator>))
		(char-ci<=? (category r7rs:char characters comparisons) (type comparator<=))
		(char-ci>=? (category r7rs:char characters comparisons) (type comparator>=))
		
		(char->integer (category r7rs:base characters) (type converter))
		(integer->char (category r7rs:base characters) (type converter))
		(digit-value (category r7rs:char characters) (type converter))
		
		(char-alphabetic? (category r7rs:char characters) (type predicate))
		(char-upper-case? (category r7rs:char characters) (type predicate))
		(char-lower-case? (category r7rs:char characters) (type predicate))
		(char-numeric? (category r7rs:char characters) (type predicate))
		(char-whitespace? (category r7rs:char characters) (type predicate))
		
		(char-upcase (category r7rs:char characters) (type procedure))
		(char-downcase (category r7rs:char characters) (type procedure))
		(char-foldcase (category r7rs:char characters) (type procedure))
		
		
		
		(procedure? (category r7rs:base functions types) (type type-predicate))
		
		(apply (category r7rs:base functions) (type procedure))
		
		(values (category r7rs:base functions values) (type constructor))
		(call-with-values (category r7rs:base functions values) (type procedure))
		
		
		
		
		(error-object? (category r7rs:base errors) (type type-predicate))
		(read-error? (category r7rs:base errors) (type predicate))
		(file-error? (category r7rs:base errors) (type predicate))
		
		(error (category r7rs:base errors) (type constructor))
		(error-object-message (category r7rs:base errors) (type accessor))
		(error-object-irritants (category r7rs:base errors) (type accessor))
		
		
		
		
		(guard (category r7rs:base errors evaluator) (type syntax))
		(with-exception-handler (category r7rs:base errors evaluator) (type procedure))
		
		(raise (category r7rs:base errors evaluator) (type procedure))
		(raise-continuable (category r7rs:base errors evaluator unsupported) (type procedure))
		
		
		
		
		(parameterize (category r7rs:base parameters) (type syntax))
		(make-parameter (category r7rs:base parameters) (type constructor))
		
		(current-input-port (category r7rs:base parameters) (type parameter))
		(current-output-port (category r7rs:base parameters) (type parameter))
		(current-error-port (category r7rs:base parameters) (type parameter))
		
		(with-input-from-file (category r7rs:file parameters functions) (type procedure))
		(with-output-from-file (category r7rs:file parameters functions) (type procedure))
		
		
		
		
		(delay (category r7rs:lazy promises evaluator) (type syntax))
		(delay-force (category r7rs:lazy promises evaluator) (type syntax))
		
		(promise? (category r7rs:lazy promises evaluator) (type type-predicate))
		(make-promise (category r7rs:lazy promises evaluator) (type constructor))
		(force (category r7rs:lazy promises evaluator) (type procedure))
		
		
		
		
		(eval (category r7rs:eval evaluator unsupported) (type procedure))
		(environment (category r7rs:eval evaluator unsupported) (type procedure))
		
		(interaction-environment (category r7rs:r5rs r7rs:repl evaluator unsupported) (type procedure))
		(scheme-report-environment (category r7rs:r5rs evaluator unsupported) (type procedure))
		(null-environment (category r7rs:r5rs evaluator unsupported) (type procedure))
		
		
		
		
		(call-with-current-continuation (category r7rs:base continuations unsupported) (type procedure) (alias call/cc))
		(dynamic-wind (category r7rs:base continuations unsupported) (type procedure))
		
		
		
		
		(cond-expand (category r7rs:base compiler unsupported) (type syntax))
		(features (category r7rs:base evaluator compiler unsupported) (type procedure))
		
		(include (category r7rs:base compiler unsupported) (type syntax))
		(include-ci (category r7rs:base compiler unsupported) (type syntax))
		
		(import (category r7rs:base compiler unsupported) (type syntax))
		
		(load (category r7rs:load compiler unsupported) (type procedure))
		
		
		
		
	)
	
)

