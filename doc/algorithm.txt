
## Speedtest / Ookla algorithm

As defined in  

Understanding Broadband Speed Measurements  

Review of different speed measurement tests, including M-Lab’s NDT test,  
and examination of the benefits of different approaches  
S. Bauer, D. Clark, W. Lehr  
2010  

https://www.measurementlab.net/publications/understanding-broadband-speed-measurements.pdf

Download test
-------------

1. Small binary files are downloaded from the web server to the client to estimate the connection speed
2. Based on this result, one of several file sizes is selected to use for the real download test
3. The test is performed with cache prevention via random strings appended to each download
4. Up to 8 parallel HTTP threads can be used for the test
5. Throughput samples are received at up to 30 times per second
6. These samples are then aggregated into 20 slices (each being 5% of the samples)
7. The fastest 10% and slowest 30% of the
slices are then discarded
8. The remaining slices are averaged
together to determine the final result.

Upload test
-----------

1. A small amount of random data is generated in the client and sent to the web server to estimate the connection speed
2. Based on this result, an appropriately sized set of randomly generated data is selected for upload
3. The upload test is then performed in configurable chunk sizes (pushed to a server-side script via a POST)
4. The test can be done using up to 8 parallel HTTP threads (configurable)
5. Chunks are sorted by speed, and the
fastest half is averaged to eliminate anomalies and determine the result

 
