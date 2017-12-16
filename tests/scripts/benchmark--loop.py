
import time

timestamp_start = time.clock ()

loop = 10 * 1000 * 1000
while loop > 0 :
	loop = loop - 1
	
	# (0, 1)

print int ((time.clock () - timestamp_start) * 1000)
print loop

