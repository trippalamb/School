package Java_M2.src;

// Player.java
import java.util.Comparator;

public class Player implements Comparable<Player>, Cloneable {
    private static final int STAT_LENGTH = 8;
    
    private boolean initialized;
    private String firstName;
    private String lastName;
    private int[] stats;
    
    private double battingAverage;
    private double onBase;
    private double slugging;
    private double ops;
    
    // Comparator for OPS sorting (highest to lowest)
    public static final Comparator<Player> OPS_COMPARATOR = (p1, p2) -> {
        // Reverse order for descending sort
        return Double.compare(p2.getOps(), p1.getOps());
    };
    
    // Default constructor
    public Player() {
        this.firstName = "unknown";
        this.lastName = "unknown";
        this.stats = new int[STAT_LENGTH];
        this.initialized = false;
    }
    
    // Constructor that takes a line of input
    public Player(String line) {
        this();
        if (line != null && !line.trim().isEmpty()) {
            String[] parts = line.trim().split("\\s+");
            if (parts.length >= 10) { // 2 for names + 8 for stats
                String[] names = {parts[0], parts[1]};
                int[] stats = new int[STAT_LENGTH];
                try {
                    for (int i = 0; i < STAT_LENGTH; i++) {
                        int value = Integer.parseInt(parts[i + 2]);
                        if (value < 0) {
                            return; // Skip initialization if negative value found
                        }
                        stats[i] = value;
                    }
                    setAll(names, stats);
                } catch (NumberFormatException e) {
                    // Skip initialization if parsing fails
                    return;
                }
            }
        }
    }
    
    // Constructor with names and stats arrays
    public Player(String[] names, int[] stats) {
        this();
        setAll(names, stats);
    }
    
    public void setAll(String[] names, int[] stats) {
        this.firstName = names[0];
        this.lastName = names[1];
        this.stats = new int[STAT_LENGTH];
        System.arraycopy(stats, 0, this.stats, 0, STAT_LENGTH);
        this.initialized = true;
        calcStatistics();
    }
    
    private void calcStatistics() {
        double hits = stats[2] + stats[3] + stats[4] + stats[5];
        this.battingAverage = hits / (double) stats[1];
        this.onBase = (hits + stats[6] + stats[7]) / (double) stats[0];
        this.slugging = (stats[2] + 2.0 * stats[3] + 3.0 * stats[4] + 4.0 * stats[5]) / (double) stats[1];
        this.ops = this.onBase + this.slugging;
    }
    
    @Override
    public String toString() {
        return String.format("%20s : %9.3f%9.3f", 
            lastName + ", " + firstName, 
            battingAverage, 
            ops);
    }
    
    // Implementation of Comparable interface
    @Override
    public int compareTo(Player other) {
        int lastNameCompare = this.lastName.compareTo(other.lastName);
        if (lastNameCompare != 0) {
            return lastNameCompare;
        }
        return this.firstName.compareTo(other.firstName);
    }
    
    // Implementation of Cloneable interface
    @Override
    public Player clone() throws CloneNotSupportedException {
        Player cloned = (Player) super.clone();
        // Deep copy of the stats array
        cloned.stats = stats.clone();
        return cloned;
    }
    
    // Getters
    public boolean isInitialized() {
        return initialized;
    }
    
    public double getBattingAverage() {
        return battingAverage;
    }
    
    public double getOps() {
        return ops;
    }
}