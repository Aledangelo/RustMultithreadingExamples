# RustMultithreadingExamples
A collection of Rust MultiThreading Example codes for educational purpose

* Example 1: simple example where two threads work on the same resource (vector);
* Example 2: Producer/Consumer that work on the same resource in mutual exclusion;
* Example 3: Exchange of messages from 1 Producer to 1 Consumer using mpsc channel;
* Example 4: Exchange of messages from more Producer to 1 Consumer using mpsc channel;
* Example 5: Multiple Producer and Multiple Consumer that work on the same resource in mutual exclusion;
* Example 6: Reader/Writer in pipeline with Producer/Consumer.
    * The generating thread will have to generate (10 times) an element (a structure containing a pair of random integers between 0 and 10), and put it in the sync channel. If the channel is full, the thread must pause until a buffer of the channel becomes free.
    * The updater thread will have to periodically (every second, 10 times) fetch an element from the channel head. If the channel is empty, the thread must wait until an item becomes available.
    * After fetching an element, and before fetching a further one, the updating thread will have to copy the element into a data structure shared with the recipient threads. The updater thread will have to pause if there are target threads consulting the buffer.
    * The recipient threads will have to periodically (every 2 seconds, 6 times) consult the current value of the structure, which will print on the screen the single integer values and their sum. If the updater thread is modifying the data structure, the recipient threads will have to pause until the modification is finished.