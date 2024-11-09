package Java_M2.src;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.Scanner;

/**
 * Baseball Player Statistics Calculator Program
 * This program reads player statistics from an input file, processes the data,
 * and generates a formatted report with player batting averages and OPS scores.
 * 
 * The program prompts the user for input and output file names, reads player data,
 * calculates statistics, and generates a formatted report including individual player
 * statistics and team summary.
 * 
 * Tripp Lamb
 * 2024 Nov 9th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * 
 * @author Tripp.Lamb
 * @version 1.0
 */
public class Main {
    
    /**
     * Prints a formatted report of player statistics to the specified PrintWriter.
     * 
     * @param fileOut PrintWriter to write the report to
     * @param players List of players to include in the report
     * @param title Title for the report section
     */
    /**
     * Tests the cloning functionality of the Player class.
     * Demonstrates the difference between shallow copy and deep copy (clone).
     * 
     * @param player The player to test cloning with
     */
    private static void test_cloning(Player player) {
        System.out.println("\nCLONING TEST RESULTS:");
        System.out.println("Original player: " + player);
        
        // Test shallow copy (reference copy)
        System.out.println("\nTesting shallow copy:");
        Player shallowCopy = player; //the shallow copy of player
        String[] newNames1 = {"Shallow", "Copy"};
        shallowCopy.setNames(newNames1);
        System.out.println("Modified shallow copy: " + shallowCopy);
        System.out.println("Original player (changed!): " + player);
        
        // Test deep copy (clone)
        System.out.println("\nTesting deep copy (clone):");
        try {
            Player deepCopy = (Player)player.clone(); //the deep copy of player
            String[] newNames2 = {"Deep", "Copy"};
            deepCopy.setNames(newNames2);
            System.out.println("Modified deep copy: " + deepCopy);
            System.out.println("Original player (unchanged): " + player);
        } catch (CloneNotSupportedException e) {
            System.err.println("Cloning failed: " + e.getMessage());
        }
        System.out.println();
    }

    /**
     * Calculates the team's overall batting average from a list of players.
     * The batting average is computed as the mean of all individual player
     * batting averages in the team.
     * 
     * @param players An ArrayList containing Player objects whose batting averages
     *               will be used in the calculation
     * @return double The team's overall batting average as a decimal value
     * @throws IllegalArgumentException if players list is empty
     */
    private static double calcBattingAverage(ArrayList<Player> players) {
        double sum = 0.0;
        for (Player player : players) {
            sum += player.getBattingAverage();
        }
        return sum / players.size();
    }

    /**
     * Prints a formatted report of player statistics to the specified PrintWriter.
     * 
     * @param fileOut PrintWriter to write the report to
     * @param players List of players to include in the report
     * @param title Title for the report section
     */
    private static void printReport(PrintWriter fileOut, ArrayList<Player> players) {
        fileOut.println("    PLAYER NAME      :    AVERAGE    OPS");
        fileOut.println("---------------------------------------------");
        
        for (Player player : players) {
            fileOut.println(player.toString());
        }
        
        fileOut.printf("\nOVERALL BATTING AVERAGE is %.3f%n", calcBattingAverage(players));

        fileOut.println("\nBASEBALL TEAM REPORT --- " + players.size() + " PLAYERS FOUND IN FILE");

    }
    
    /**
     * Main program entry point. Handles user input/output, file processing,
     * and report generation.
     * 
     * @param args Command line arguments (not used)
     */
    public static void main(String[] args) {
        
        Scanner console = new Scanner(System.in); // a scanner object which reads user input
        ArrayList<Player> players = new ArrayList<>(); //an arraylist of player data
        
        System.out.println("Welcome to the player statistics calculator test program.\n");
        
        System.out.print("Enter the name of the input data file: ");
        String inputFileName = console.nextLine(); //the name of the input file
        
        System.out.print("\nEnter the name of the output data file (no extension): ");
        String outputFileName_base = console.nextLine(); //the name of the output file
        System.out.println();
        
        String outputFileName_byName = outputFileName_base + "_byname.txt";
        String outputFileName_ops = outputFileName_base + "_byops.txt";

        try {
            Scanner fileIn = new Scanner(new File(inputFileName)); // a scanner object which reads the player data
            PrintWriter fileOut_byName = new PrintWriter(new File(outputFileName_byName)); //a printwriter object which writes the name sort player report
            PrintWriter fileOut_byOps = new PrintWriter(new File(outputFileName_ops)); //a printwriter object which writes the ops sort player report
            
            System.out.println("Reading the data from: " + inputFileName);
            
            while (fileIn.hasNextLine()) {
                String line = fileIn.nextLine(); //current line of data from the player data file
                Player player = new Player(line); //current player object
                if (player.isInitialized()) {
                    players.add(player);
                }
            }

            // Sort by name (using Comparable interface) and print report
            Collections.sort(players);
            printReport(fileOut_byName, players);

            // Sort by OPS (using Comparator) and print report
            Collections.sort(players, new Comparator<Player>() {
                @Override
                public int compare(Player p1, Player p2) {
                    // Sort in descending order (highest to lowest)
                    return Double.compare(p2.getOPS(), p1.getOPS());
                }
            });
            printReport(fileOut_byOps, players);

            // Final summary
            
            fileIn.close();
            fileOut_byName.close();
            fileOut_byOps.close();
            
            System.out.println("The output is in: \n - <" + outputFileName_byName + "> \n - <" + outputFileName_ops + ">\n");
            System.out.println("End of Program");
            
        } catch (FileNotFoundException e) {
            System.err.println("Error: " + e.getMessage());
        }
        
        // Test cloning with the first player
        // leave commented unless testing.
        // if (!players.isEmpty()) {
        //     test_cloning(players.get(0));
        // }

        console.close();
    }
}