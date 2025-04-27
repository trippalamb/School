public class Counter {
    int _count = 0;
    
    public Counter() {}

    public void increment() {
        _count++;
    }

    @Override
    public String toString() {
        return "" + _count;
    }
}
