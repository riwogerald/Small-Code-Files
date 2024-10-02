# Define constants
MAX_EMPLOYEES = 250

# Data structures for employee information
class Date:
    def __init__(self, day, month, year):
        self.day = day
        self.month = month
        self.year = year

    def __repr__(self):
        return f"{self.day:02d}/{self.month:02d}/{self.year}"

class Dependant:
    def __init__(self, first_name, last_name, sex, birth_date):
        self.first_name = first_name
        self.last_name = last_name
        self.sex = sex
        self.birth_date = birth_date

    def __repr__(self):
        return f"{self.first_name} {self.last_name}, {self.sex}, Born: {self.birth_date}"

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

class EmployeeInfo:
    def __init__(self, employee_key, data):
        self.employee_key = employee_key
        self.data = data

    def __repr__(self):
        return f"Employee Key: {self.employee_key}, Data: {self.data}"

# Initialize the personnel data
Personnel = []

# Adding the 75th employee to the system
def add_employee(employee_key, first_name, last_name, sex, designation, department, salary, birth_date, hire_date, marital_status, kin):
    new_employee = EmployeeData(first_name, last_name, sex, designation, department, salary, birth_date, hire_date, marital_status, kin)
    employee_info = EmployeeInfo(employee_key, new_employee)
    Personnel.append(employee_info)

# Define the spouse and child
spouse = Dependant("Grace", "Rehema", "Female", Date(7, 7, 1977))
child = Dependant("Furaha", "Amani", "Male", Date(1, 1, 2000))

# Add the new employee (75th employee) with their dependants
add_employee(
    employee_key=75,
    first_name="John",  # You can change the employee name
    last_name="Doe",
    sex="Male",
    designation="Manager",  # You can change the designation
    department="IT",
    salary=75000.0,
    birth_date=Date(15, 5, 1980),
    hire_date=Date(1, 9, 2024),
    marital_status="Married",
    kin=[spouse, child]
)

# Display the added employee information
for employee in Personnel:
    print(employee)

# Saving employee data to a file (optional, you can use it for storage purposes)
def save_employee_data(filename="employee_data.txt"):
    with open(filename, 'w') as file:
        for employee in Personnel:
            file.write(f"{employee}\n")

# Call the save function to store employee data
save_employee_data()
