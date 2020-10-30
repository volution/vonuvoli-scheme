

(define mode
	(case (command-line-length)
		((0 1)
			'r7rs)
		((2)
			(string->symbol (command-line-ref 1)))
		(else
			(error "invalid-arguments-count"))))


(define input-port (current-input-port))
(define output-port (current-output-port))


(case mode
	
	
	((r7rs)
		(do
				((line (read-line input-port) (read-line input-port)))
				((eof-object? line))
			(write-string line output-port)
			(newline output-port)))
	
	
	
	
	((read-string-line-loop)
		(do
				((line (read-string-line input-port) (read-string-line input-port)))
				((eof-object? line))
			(write-string-line line output-port)))
	
	((read-string-line-fold)
		(read-string-line-fold
			input-port
			(lambda (line outcome)
				(write-string-line line output-port)
				outcome)
			#undefined))
	
	
	((read-bytevector-line-loop)
		(do
				((line (read-bytevector-line input-port) (read-bytevector-line input-port)))
				((eof-object? line))
			(write-bytevector-line line output-port)))
	
	((read-bytevector-line-fold)
		(read-bytevector-line-fold
			input-port
			(lambda (line outcome)
				(write-bytevector-line line output-port)
				outcome)
			#undefined))
	
	
	
	
	((read-string-chunk-loop)
		(do
				((chunk (read-string-chunk input-port) (read-string-chunk input-port)))
				((eof-object? chunk))
			(write-string chunk output-port)))
	
	((read-string-chunk-fold)
		(read-string-chunk-fold
			input-port
			(lambda (chunk outcome)
				(write-string chunk output-port)
				outcome)
			#undefined))
	
	
	((read-bytevector-chunk-loop)
		(do
				((chunk (read-bytevector-chunk input-port) (read-bytevector-chunk input-port)))
				((eof-object? chunk))
			(write-bytevector chunk output-port)))
	
	((read-bytevector-chunk-fold)
		(read-bytevector-chunk-fold
			input-port
			(lambda (chunk outcome)
				(write-bytevector chunk output-port)
				outcome)
			#undefined))
	
	
	(else
		(error "invalid-mode" mode)))

