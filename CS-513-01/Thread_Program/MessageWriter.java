/*
 * This class implements the `Runnable` interface which is needed for a `Thread` instance.
 */
public class MessageWriter implements Runnable {
    private String _name;       // used to distinguish threads
    private int _delay;         // milliseconds to delay the thread
    private int _n;             // number of times to send a message
    private Counter _counter;   // each thread will share a reference to a common counter
    
    public MessageWriter(String name, int delay, int n, Counter counter) {
        _name = name;
        _delay = delay;
        _n = n;
        _counter = counter;
    }

    /**
     * This method is the entry point for the Thread. It will write a message
     * to the console every `_delay` milliseconds for a total of `_n` messages.
     */
    public void run(){
        int i = 0;
        while (i < _n) {  //loop until the local counter of messages has been successfully sent
            try {
                //attempt to sleep
                Thread.sleep(_delay);
            } catch (InterruptedException ex) {
                //safely catch interrupted exception and avoid incrementing the counters
                System.out.println(_name + " was interupted at message number " + _counter + ". Safely catching this exception and continuing. Not incrementing counter.");
                continue;
            }

            //if the thread is not interrupted, increment the local and global counter and print the success message
            i++;
             _counter.increment();
             System.out.println(_name + " has woken up and this is message number " + _counter);
        }
    }

    /**
     * Returns the name of this thread as set during construction.
     * @return The name of this thread.
     */
    public String get_name() {
        return _name;
    }
}