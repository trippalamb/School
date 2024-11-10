from book_manager import BookManager
import getpass

def main():
    """
    Main entry point of the application.
    
    Handles the main application loop including login and menu interactions.
    """
    book_manager = BookManager()
    exit_program = False
    
    while not exit_program:
        if not book_manager.current_user:
            print("\n=== Login ===")
            username = input("Username: ")
            password = getpass.getpass("Password: ")
            
            if book_manager.login(username, password):
                print(f"Welcome, {username}!")
            else:
                print("Invalid credentials!")
                continue
        
        exit_program = book_manager.handle_choice(book_manager.display_menu())

if __name__ == "__main__":
    main()