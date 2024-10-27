package Java_M1.src;

public class Player {
    private static final int STAT_LENGTH = 8;
    
    private boolean initialized;
    private String firstName;
    private String lastName;
    private int[] stats;
    
    private double battingAverage;
    private double onBase;
    private double slugging;
    private double ops;
    
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
                        stats[i] = Integer.parseInt(parts[i + 2]);
                    }
                    setAll(names, stats);
                } catch (NumberFormatException e) {
                    System.err.println("Error parsing stats for player: " + names[0] + " " + names[1]);
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
        double hits = stats[2] + stats[3] + stats[4] + stats[5]; // singles + doubles + triples + home runs
        
        // Batting average = hits / at bats
        this.battingAverage = hits / (double) stats[1];
        
        // On base percentage = (hits + walks + HBP) / plate appearances
        this.onBase = (hits + stats[6] + stats[7]) / (double) stats[0];
        
        // Slugging = (singles + 2*doubles + 3*triples + 4*home runs) / at bats
        this.slugging = (stats[2] + 2.0 * stats[3] + 3.0 * stats[4] + 4.0 * stats[5]) / (double) stats[1];
        
        // OPS = on base + slugging
        this.ops = this.onBase + this.slugging;
    }
    
    @Override
    public String toString() {
        return String.format("%20s : %9.3f%9.3f", 
            lastName + ", " + firstName, 
            battingAverage, 
            ops);
    }
    
    public boolean isInitialized() {
        return initialized;
    }
    
    public double getBattingAverage() {
        return battingAverage;
    }
}