

(define input-port (current-input-port))
(define output-port (current-output-port))


(do
		((line (read-line input-port) (read-line input-port)))
		((eof-object? line))
	(write-string line output-port)
	(newline output-port))

