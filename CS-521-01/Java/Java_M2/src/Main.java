package Java_M2.src;

// Main.java
import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class Main {
    
    private static double calcBattingAverage(ArrayList<Player> players) {
        double sum = 0.0;
        for (Player player : players) {
            sum += player.getBattingAverage();
        }
        return sum / players.size();
    }
    
    private static void printReport(PrintWriter output, String title, ArrayList<Player> players) {
        output.println("\n" + title);
        output.println("    PLAYER NAME      :    AVERAGE    OPS");
        output.println("---------------------------------------------");
        for (Player player : players) {
            output.println(player.toString());
        }
    }
    
    public static void main(String[] args) {
        Scanner console = new Scanner(System.in);
        ArrayList<Player> players = new ArrayList<>();
        
        System.out.println("Welcome to the player statistics calculator test program.\n");
        
        // Get input filename
        System.out.print("Enter the name of the input data file: ");
        String inputFileName = console.nextLine();
        
        // Get output filename
        System.out.print("\nEnter the name of the output data file: ");
        String outputFileName = console.nextLine();
        System.out.println();
        
        try {
            // Open input file
            Scanner fileIn = new Scanner(new File(inputFileName));
            PrintWriter fileOut = new PrintWriter(new File(outputFileName));
            
            System.out.println("Reading the data from: " + inputFileName);
            
            // Read and process each line
            while (fileIn.hasNextLine()) {
                String line = fileIn.nextLine();
                Player player = new Player(line);
                if (player.isInitialized()) {
                    players.add(player);
                }
            }
            
            // Print report sorted by name (natural ordering)
            Collections.sort(players);
            printReport(fileOut, "PLAYERS ORDERED BY NAME:", players);
            
            // Print report sorted by OPS (highest to lowest)
            Collections.sort(players, Player.OPS_COMPARATOR);
            printReport(fileOut, "\nPLAYERS ORDERED BY OPS (HIGHEST TO LOWEST):", players);
            
            // Print summary
            fileOut.println("\nBASEBALL TEAM REPORT --- " + players.size() + " PLAYERS FOUND IN FILE");
            fileOut.printf("OVERALL BATTING AVERAGE is %.3f%n", calcBattingAverage(players));
            
            // Test cloning functionality
            if (!players.isEmpty()) {
                try {
                    Player original = players.get(0);
                    Player cloned = original.clone();
                    fileOut.println("\nCLONE TEST RESULTS:");
                    fileOut.println("Original player: " + original);
                    fileOut.println("Cloned player: " + cloned);
                    // Modify cloned player's stats to verify deep copy
                    String[] newNames = {"Test", "Clone"};
                    int[] newStats = {100, 90, 30, 10, 5, 5, 10, 2};
                    cloned.setAll(newNames, newStats);
                    fileOut.println("After modifying clone:");
                    fileOut.println("Original player: " + original);
                    fileOut.println("Modified clone: " + cloned);
                } catch (CloneNotSupportedException e) {
                    fileOut.println("\nError during clone test: " + e.getMessage());
                }
            }
            
            // Close files
            fileIn.close();
            fileOut.close();
            
            System.out.println("The output is in: " + outputFileName + "\n");
            System.out.println("End of Program");
            
        } catch (FileNotFoundException e) {
            System.err.println("Error: " + e.getMessage());
        }
        
        console.close();
    }
}