
(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop 100000 (- loop 1))
	)
	((zero? loop)
		(display (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second))))
	
	(make-vector 100 '())
)

