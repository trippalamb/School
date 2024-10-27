package Java_M1.src;

// Main.java
import java.io.File;
import java.io.FileNotFoundException;
import java.io.PrintWriter;
import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    
    private static double calcBattingAverage(ArrayList<Player> players) {
        double sum = 0.0;
        for (Player player : players) {
            sum += player.getBattingAverage();
        }
        return sum / players.size();
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
            
            // Write header to output file
            fileOut.println("    PLAYER NAME      :    AVERAGE    OPS");
            fileOut.println("---------------------------------------------");
            
            // Read and process each line
            while (fileIn.hasNextLine()) {
                String line = fileIn.nextLine();
                Player player = new Player(line);
                if (player.isInitialized()) {
                    players.add(player);
                    fileOut.println(player.toString());
                }
            }
            
            // Write summary
            fileOut.println("\n");
            fileOut.println("BASEBALL TEAM REPORT --- " + players.size() + " PLAYERS FOUND IN FILE");
            fileOut.printf("OVERALL BATTING AVERAGE is %.3f%n", calcBattingAverage(players));
            
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