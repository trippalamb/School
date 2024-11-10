import sqlite3
from typing import List, Optional, Tuple
import hashlib

class DatabaseManager:
    """
    Manages all database operations for the book management system.
    
    This class handles the initialization of the database, user authentication,
    and all CRUD operations related to books.
    
    Attributes:
        db_name (str): Name of the SQLite database file
    """

    def __init__(self, db_name: str = "library.db"):
        """
        Initialize DatabaseManager with a database name.
        
        Args:
            db_name (str, optional): Name of the database file. Defaults to "library.db".
        """
        self.db_name = db_name
        self.init_db()

    def connect(self) -> sqlite3.Connection:
        """
        Create and return a database connection.
        
        Returns:
            sqlite3.Connection: A connection object to the SQLite database
        """
        return sqlite3.connect(self.db_name)

    def init_db(self) -> None:
        """
        Initialize the database with required tables and default admin user.
        
        Creates two tables:
        - users: Stores user credentials
        - books: Stores book information
        
        Also creates a default admin user if one doesn't exist.
        """
        with self.connect() as conn:
            cursor = conn.cursor()
            
            # Create users table
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS users (
                    username TEXT PRIMARY KEY,
                    password TEXT NOT NULL
                )
            ''')
            
            # Create books table
            cursor.execute('''
                CREATE TABLE IF NOT EXISTS books (
                    isbn TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    author TEXT NOT NULL,
                    year INTEGER
                )
            ''')
            
            # Insert default admin user if not exists
            cursor.execute(
                "INSERT OR IGNORE INTO users (username, password) VALUES (?, ?)",
                ("admin", hashlib.sha256("password".encode()).hexdigest())
            )
            conn.commit()

    def verify_credentials(self, username: str, password: str) -> bool:
        """
        Verify user login credentials.
        
        Args:
            username (str): The username to verify
            password (str): The password to verify (will be hashed before comparison)
        
        Returns:
            bool: True if credentials are valid, False otherwise
        """
        with self.connect() as conn:
            cursor = conn.cursor()
            hashed_password = hashlib.sha256(password.encode()).hexdigest()
            cursor.execute(
                "SELECT username FROM users WHERE username=? AND password=?",
                (username, hashed_password)
            )
            return cursor.fetchone() is not None

    def add_book(self, isbn: str, title: str, author: str, year: int) -> bool:
        """
        Add a new book to the database.
        
        Args:
            isbn (str): The ISBN of the book
            title (str): The title of the book
            author (str): The author of the book
            year (int): The publication year of the book
        
        Returns:
            bool: True if book was added successfully, False if ISBN already exists
        """
        try:
            with self.connect() as conn:
                cursor = conn.cursor()
                cursor.execute(
                    "INSERT INTO books (isbn, title, author, year) VALUES (?, ?, ?, ?)",
                    (isbn, title, author, year)
                )
                return True
        except sqlite3.IntegrityError:
            return False

    def search_books(self, search_term: str) -> List[Tuple[str, str, str, int]]:
        """
        Search for books by title, author, or ISBN.
        
        Args:
            search_term (str): The term to search for in titles, authors, or ISBNs
        
        Returns:
            List[Tuple[str, str, str, int]]: List of tuples containing book information
                                           (isbn, title, author, year)
        """
        with self.connect() as conn:
            cursor = conn.cursor()
            cursor.execute(
                """
                SELECT * FROM books 
                WHERE title LIKE ? OR author LIKE ? OR isbn LIKE ? or year LIKE ?
                """,
                (f"%{search_term}%", f"%{search_term}%", f"%{search_term}%", f"%{search_term}%")
            )
            return cursor.fetchall()
