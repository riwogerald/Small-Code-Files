# Class for storing Date information
class Date:
    def __init__(self, day, month, year):
        self.day = day
        self.month = month
        self.year = year

    def __repr__(self):
        return f"{self.day:02d}/{self.month:02d}/{self.year}"

# Class for storing dependants (spouse and children)
class Dependant:
    def __init__(self, first_name, last_name, sex, birth_date):
        self.first_name = first_name
        self.last_name = last_name
        self.sex = sex
        self.birth_date = birth_date

    def __repr__(self):
        return f"{self.first_name} {self.last_name}, {self.sex}, Born: {self.birth_date}"

# Class for storing employee data
class EmployeeData:
    def __init__(self, first_name, last_name, sex, designation, department, salary, birth_date, hire_date, marital_status, kin=[]):
        self.first_name = first_name
        self.last_name = last_name
        self.sex = sex
        self.designation = designation
        self.department = department
        self.salary = salary
        self.birth_date = birth_date
        self.hire_date = hire_date
        self.marital_status = marital_status
        self.kin = kin  # List of Dependants

    def __repr__(self):
        return f"{self.first_name} {self.last_name}, {self.sex}, {self.designation}, {self.department}, Salary: {self.salary}, Birth Date: {self.birth_date}, Hire Date: {self.hire_date}, Marital Status: {self.marital_status}, Kin: {self.kin}"

# Node class for the linked list
class EmployeeNode:
    def __init__(self, employee_data=None):
        self.employee_data = employee_data  # Data field
        self.next = None  # Pointer to the next node

# Linked List class for managing the list of employees
class EmployeeLinkedList:
    def __init__(self):
        self.head = None  # Initialize the list with no employees

    # Function to insert a new employee at the end of the list
    def insert_employee(self, employee_data):
        new_node = EmployeeNode(employee_data)
        if self.head is None:  # If the list is empty
            self.head = new_node
        else:
            current = self.head
            # Traverse the list to the last node
            while current.next:
                current = current.next
            # Add the new employee at the end
            current.next = new_node

    # Function to display all employees in the list
    def display_employees(self):
        current = self.head
        while current:
            print(current.employee_data)
            current = current.next

# Creating the employee linked list
employee_list = EmployeeLinkedList()

# First employee data - Wema Fadhili, Finance Director
first_employee_kin = [
    Dependant("Baraka", "Imani", "Male", Date(10, 10, 1975)),  # Spouse
    Dependant("Tumaini", "Kibali", "Male", Date(15, 6, 2010))  # Child
]

first_employee = EmployeeData(
    first_name="Wema",
    last_name="Fadhili",
    sex="Female",
    designation="Director",
    department="Finance",
    salary=120000.0,
    birth_date=Date(1, 1, 1970),
    hire_date=Date(1, 5, 2020),
    marital_status="Married",
    kin=first_employee_kin
)

# Insert the first employee into the linked list
employee_list.insert_employee(first_employee)

# Display the list of employees
employee_list.display_employees()
