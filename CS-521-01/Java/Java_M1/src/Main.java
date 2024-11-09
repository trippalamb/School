package Java_M1.src;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
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
 * 2024 Sep 8th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * 
 * @author Tripp.Lamb
 * @version 1.0
 */
public class Main {
    
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
     * Main program entry point. Handles user input/output, file processing,
     * and report generation.
     * 
     * @param args Command line arguments (not used)
     */
    public static void main(String[] args) {
        
        Scanner console = new Scanner(System.in); // a scanner object which reads user input
        ArrayList<Player> players = new ArrayList<>(); //an araylist of player data
        
        System.out.println("Welcome to the player statistics calculator test program.\n");
        
        System.out.print("Enter the name of the input data file: ");
        String inputFileName = console.nextLine(); //the name of the input file
        
        System.out.print("\nEnter the name of the output data file: ");
        String outputFileName = console.nextLine(); //the name of the output file
        System.out.println();
        
        try {

            Scanner fileIn = new Scanner(new File(inputFileName)); // a scanner object which reads the player data
            PrintWriter fileOut = new PrintWriter(new File(outputFileName)); //a printwriter object which writes the player report
            
            System.out.println("Reading the data from: " + inputFileName);
            
            fileOut.println("    PLAYER NAME      :    AVERAGE    OPS");
            fileOut.println("---------------------------------------------");
            
            while (fileIn.hasNextLine()) {
                String line = fileIn.nextLine(); //current line of data from the player data file
                Player player = new Player(line); //current player object
                if (player.isInitialized()) {
                    players.add(player);
                    fileOut.println(player.toString());
                }
            }
            
            fileOut.println("\n");
            fileOut.println("BASEBALL TEAM REPORT --- " + players.size() + " PLAYERS FOUND IN FILE");
            fileOut.printf("OVERALL BATTING AVERAGE is %.3f%n", calcBattingAverage(players));
            
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