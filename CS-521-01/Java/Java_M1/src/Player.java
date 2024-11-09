package Java_M1.src;

/**
 * Represents a baseball player with their name and statistics.
 */
public class Player {
    private static final int STAT_LENGTH = 8; // number of statistics
    
    private boolean initialized; // indicates if the player has been initialized
    private String firstName; // player's first name
    private String lastName; // player's last name
    private int[] stats; // player's statistics (see function documentation for order and stats)
    
    private double battingAverage; // player's batting average
    private double onBase; // player's on base percentage
    private double slugging; // player's slugging percentage
    private double ops; // player's on base + slugging
    
        
    /**
     * Constructs a new Player object with default values.
     */
    public Player() {
        this.firstName = "unknown";
        this.lastName = "unknown";
        this.stats = new int[STAT_LENGTH];
        this.initialized = false;
    }
    
    /**
     * Constructs a new Player object from a string representation of the player's data.
     * 
     * If the string is null or empty, or if the statistics cannot be parsed, an error message is printed.
     * 
     * @param line the string representation of the player's data
     */
    public Player(String line) {
        this();
        if (line != null && !line.trim().isEmpty()) {
            String[] parts = line.trim().split("\\s+"); // split the line into parts
            if (parts.length == 10) { // 2 for names + 8 for stats
                String[] names = {parts[0], parts[1]}; //temporarily holds the player's names
                int[] stats = new int[STAT_LENGTH]; //temporarily array for stats
                try {
                    for (int i = 0; i < STAT_LENGTH; i++) {
                        stats[i] = Integer.parseInt(parts[i + 2]);
                    }
                    setAll(names, stats);
                } catch (NumberFormatException e) {
                    System.err.println("Error parsing stats for player: " + names[0] + " " + names[1]);
                }
            }
            else{
                System.err.println("Player stats line has the wrong number of elements for player: " + parts[0] + " " + parts[1]);
            }
        }
    }
        
    /**
     * Constructs a new Player object from arrays of names and statistics.
     * 
     * The names array is expected to have two elements, representing the player's first and last names.
     * The stats array is expected to have 8 elements, representing the player's statistics in the order:
     * - Games played
     * - At bats
     * - Runs scored
     * - Hits
     * - Doubles
     * - Home runs
     * - Runs batted in
     * - Stolen bases
     * 
     * @param names an array of 2 strings, representing the player's first and last names
     * @param stats an array of 8 integers, representing the player's statistics
     */
    public Player(String[] names, int[] stats) {
        this();
        setAll(names, stats);
    }
        
    
    /**
     * Sets the player's name and statistics from the given arrays.
     * <p>
     * The first element of the names array is used as the player's first name,
     * and the second element is used as the player's last name. The stats array
     * should contain 8 elements, representing the following statistics in order:
     * - Games played
     * - At bats
     * - Runs scored
     * - Hits
     * - Doubles
     * - Home runs
     * - Runs batted in
     * - Stolen bases
     * 
     * The initialized flag is set to true after calling this method.
     * 
     * @param names An array of 2 strings, representing the player's first and last names
     * @param stats An array of 8 integers, representing the player's statistics
     */
    public void setAll(String[] names, int[] stats) {
        this.firstName = names[0];
        this.lastName = names[1];
        this.stats = new int[STAT_LENGTH];
        System.arraycopy(stats, 0, this.stats, 0, STAT_LENGTH);
        this.initialized = true;
        calcStatistics();
    }
    
    /**
     * Calculates the player's statistics from the given stats array.
     * 
     * The following statistics are calculated:
     * - Batting average
     * - On base percentage
     * - Slugging percentage
     * - OPS (on base + slugging)
     * 
     */
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
    
    /**
     * Returns a string representation of the player
     * 
     * @return a string representation of the player
     */
    @Override
    public String toString() {
        return String.format("%20s : %9.3f%9.3f", 
            lastName + ", " + firstName, 
            battingAverage, 
            ops);
    }
    
/**
 * Checks if the player has been initialized.
 *
 * @return true if the player has been initialized with names and stats, false otherwise.
 */
    public boolean isInitialized() {
        return initialized;
    }
    
    /**
     * Returns the player's batting average
     * 
     * @return the player's batting average
     */
    public double getBattingAverage() {
        return battingAverage;
    }
}