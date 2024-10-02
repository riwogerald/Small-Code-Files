# Function to calculate letter grade based on marks
def get_letter_grade(mark):
    if mark >= 80:
        return 'A'
    elif mark >= 65:
        return 'B'
    elif mark >= 50:
        return 'C'
    elif mark >= 40:
        return 'D'
    else:
        return 'F'

def main():
    marks = []
    sum_of_marks = 0

    # Input marks for 20 students
    print("Enter the marks of 20 students:")
    for i in range(1, 21):
        mark = int(input(f"Enter the mark for student {i}: "))
        marks.append(mark)
        sum_of_marks += mark

    # Calculate average
    average_mark = sum_of_marks / 20

    # Display the marks, sum, and average
    print("\nMarks and Grades for each student:")
    for i in range(20):
        grade = get_letter_grade(marks[i])
        print(f"Student {i + 1}: Mark = {marks[i]}, Grade = {grade}")

    print(f"\nThe sum of the marks is: {sum_of_marks}")
    print(f"The average mark is: {average_mark:.2f}")

# Run the program
main()