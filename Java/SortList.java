// SortList.java
import java.util.Arrays;

public class SortList {
    public static void bubbleSort(int[] arr) {
        int n = arr.length;
        for (int i = 0; i < n - 1; i++) {
            for (int j = 0; j < n - i - 1; j++) {
                if (arr[j] > arr[j + 1]) {
                    // Swap elements
                    int temp = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = temp;
                }
            }
        }
    }

    public static void main(String[] args) {
        int[] numbers = {5, 3, 8, 1, 9, 2};
        int[] numbersCopy = numbers.clone();
        
        System.out.println("Original array: " + Arrays.toString(numbers));
        
        Arrays.sort(numbers);
        System.out.println("Sorted using Arrays.sort(): " + Arrays.toString(numbers));
        
        bubbleSort(numbersCopy);
        System.out.println("Sorted using bubble sort: " + Arrays.toString(numbersCopy));
    }
}
