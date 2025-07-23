# Step 1: Initialize the list of referees
referees = []
current_index = 0  # Starting point for batch selection
batch_size = 60  # Number of referees to select per weekend

# Step 2: Function to input referee details
def input_referee():
    name = input("Enter the name of the referee: ")
    id_number = input("Enter the ID number of the referee: ")
    sub_county = input("Enter the sub-county of the referee: ")
    referees.append({"name": name, "ID": id_number, "sub_county": sub_county})

# Step 3: Function to select a batch of referees for the weekend
def select_referees():
    global current_index

    if not referees:
        print("No referees available. Please add referees first.")
        return

    if current_index >= len(referees):  # If we reach the end of the list
        print("All referees have been selected. Resetting for a new cycle.")
        current_index = 0  # Reset index if all referees have been selected

    # Select a batch of referees
    end_index = min(current_index + batch_size, len(referees))
    selected_batch = referees[current_index:end_index]
    current_index = end_index  # Move to the next batch

    # Output the selected referees
    print(f"\nSelected Referees for this weekend (Batch {current_index // batch_size}):")
    for referee in selected_batch:
        print(f"Name: {referee['name']}, ID: {referee['ID']}, Sub-county: {referee['sub_county']}")

# Step 4: Main menu to input referees and select referees for the weekend
def main():
    global current_index
    while True:
        print("\nMenu:")
        print("1. Add a new referee")
        print("2. Select referees for this weekend")
        print("3. Show all referees")
        print("4. Quit")
        
        choice = input("Choose an option (1, 2, 3, 4): ")

        if choice == '1':
            input_referee()
        elif choice == '2':
            select_referees()
        elif choice == '3':
            print("\nAll Referees:")
            for idx, referee in enumerate(referees):
                print(f"{idx+1}. Name: {referee['name']}, ID: {referee['ID']}, Sub-county: {referee['sub_county']}")
        elif choice == '4':
            print("Exiting the program.")
            break
        else:
            print("Invalid choice. Please select a valid option.")

# Run the program
main()
