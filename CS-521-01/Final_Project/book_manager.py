from dataclasses import dataclass
from database_manager import DatabaseManager
from typing import Optional, List, Tuple
from enum import Enum, auto

@dataclass
class Book:
    """
    Data class representing a book.
    
    Attributes:
        isbn (str): The ISBN of the book
        title (str): The title of the book
        author (str): The author of the book
        year (int): The publication year of the book
    """
    isbn: str
    title: str
    author: str
    year: int

class MenuChoice(Enum):
    """
    Enumeration of available menu choices.
    """
    ADD_BOOK = "add"
    SEARCH_BOOK = "search"
    LOGOUT = "logout"

class BookManager:
    """
    Manages book-related operations and user interactions.
    
    This class serves as the main interface between the user interface
    and the database operations.
    
    Attributes:
        db_manager (DatabaseManager): Instance of DatabaseManager for database operations
        current_user (Optional[str]): Currently logged in username, if any
    """

    def __init__(self):
        """Initialize BookManager with a new DatabaseManager instance."""
        self.db_manager = DatabaseManager()
        self.current_user: Optional[str] = None

    def login(self, username: str, password: str) -> bool:
        """
        Handle user login.
        
        Args:
            username (str): Username to login with
            password (str): Password to login with
        
        Returns:
            bool: True if login successful, False otherwise
        """
        if self.db_manager.verify_credentials(username, password):
            self.current_user = username
            return True
        return False

    def logout(self) -> None:
        """Clear current user session."""
        self.current_user = None

    def add_book(self, isbn: str, title: str, author: str, year: int) -> bool:
        """
        Add a new book to the system.
        
        Args:
            isbn (str): The ISBN of the book
            title (str): The title of the book
            author (str): The author of the book
            year (int): The publication year of the book
        
        Returns:
            bool: True if book was added successfully, False otherwise
        """
        if not self.current_user:
            return False
        return self.db_manager.add_book(isbn, title, author, year)

    def search_book(self, search_term: str) -> List[Tuple[str, str, str, int]]:
        """
        Search for books in the system.
        
        Args:
            search_term (str): Term to search for in titles, authors, or ISBNs
        
        Returns:
            List[Tuple[str, str, str, int]]: List of matching books
        """
        if not self.current_user:
            return []
        return self.db_manager.search_books(search_term)

    def display_menu(self) -> str:
        """
        Display the main menu and get user choice.
        
        Returns:
            str: User's menu choice
        """
        print("\n=== Book Management System ===")
        print(f"add    - Add Book")
        print(f"search - Search Book")
        print(f"logout - Log Out")
        return input("\nEnter your choice: ").lower().strip()

    def handle_choice(self, choice: str) -> bool:
        """
        Handle user menu choice.
        
        Args:
            choice (str): User's menu choice
        
        Returns:
            bool: True if program should exit, False otherwise
        """
        try:
            menu_choice = MenuChoice(choice)
        except ValueError:
            print(f"Invalid choice. Please choose from: {', '.join(c.value for c in MenuChoice)}")
            return False

        if menu_choice == MenuChoice.ADD_BOOK:
            isbn = input("Enter ISBN: ")
            title = input("Enter Title: ")
            author = input("Enter Author: ")
            try:
                year = int(input("Enter Year: "))
            except ValueError:
                print("Invalid year format.")
                return False
            
            if self.add_book(isbn, title, author, year):
                print("Book added successfully!")
            else:
                print("Failed to add book. ISBN might already exist.")
                
        elif menu_choice == MenuChoice.SEARCH_BOOK:
            search_term = input("Enter search term: ")
            results = self.search_book(search_term)
            
            if results:
                print("\n\nSearch Results:")
                for book in results:
                    print(f"ISBN: {book[0]}")
                    print(f"Title: {book[1]}")
                    print(f"Author: {book[2]}")
                    print(f"Year: {book[3]}")
                    print("-" * 30)
            else:
                print("No books found.")
                
        elif menu_choice == MenuChoice.LOGOUT:
            self.logout()
            return True
            
        return False