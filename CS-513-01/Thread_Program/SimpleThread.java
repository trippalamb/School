public class SimpleThread {
    public static void main(String[] args) {
        System.out.println("Creating threads...");
        
        Counter counter = new Counter();
        
        MessageWriter msgWriterOne = new MessageWriter("Thread-1", 1000, 10, counter);
        MessageWriter msgWriterTwo = new MessageWriter("Thread-2", 2000, 10, counter);

        Thread threadOne = new Thread(msgWriterOne, msgWriterOne.get_name());
        Thread threadTwo = new Thread(msgWriterTwo, msgWriterTwo.get_name());
        
        System.out.println("Starting threads...");
        threadOne.start();
        threadTwo.start();
        
        System.out.println("Main thread continues execution...");
        
        // for testing thread interruption logic
        // testThreadInterrupt(threadOne, 5000);
        // testThreadInterrupt(threadTwo, 7000);

        try {
             threadOne.join();
             threadTwo.join();
         } catch (InterruptedException e) {
            System.out.println("Thread was interrupted while waiting: " + e.getMessage());
            e.printStackTrace();
        } catch (Exception e) {
            System.out.println("An unexpected error occurred: " + e.getMessage());
            System.exit(1);
        }
         
        System.out.println("\nmain program execution at end");
        System.exit(0);
    }

    /**
     * Tests thread interruption by sleeping for the specified delay 
     * and then interrupting the provided thread.
     * 
     * @param thread The thread to interrupt
     * @param sleepDelay Time to sleep before interrupting (in milliseconds)
     */
    public static void testThreadInterrupt(Thread thread, int sleepDelay) {
        try {

            Thread.sleep(sleepDelay);
            
            System.out.println("Interrupting the " + thread.getName() + " thread...");

            thread.interrupt();
            
        } catch (InterruptedException e) {
            System.out.println("Interruption occurred while waiting to interrupt thread: " + e.getMessage());
            e.printStackTrace();
        } catch (Exception e) {
            System.out.println("An unexpected error occurred while attempting to interrupt: " + e.getMessage());
            System.exit(1);
        }
    }
}