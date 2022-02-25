package org.dyson.sort;

import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.NotNull;

import java.util.function.Supplier;

public final class Sorter {
	private final int[] instanceArray;

	@Contract(pure = true)
	Sorter(int[] array) {
		this.instanceArray = array;
	}

	@Contract(pure = true)
	public int[] getInstanceArray() {
		return instanceArray.clone();
	}

	/**
	 * Use a simple bubble sort algorithm to sort the instance array.
	 *
	 * @return The sorted array
	 */
	@Contract(pure = true)
	public int[] bubbleSort() {
		int[] array = getInstanceArray();

		for (int i = 0; i < array.length - 1; i++)
			for (int j = 0; j < array.length - 1; j++)
				if (array[j] > array[j + 1]) {
					int left = array[j];
					array[j] = array[j + 1];
					array[j + 1] = left;
				}

		return array;
	}

	/**
	 * Use an "optimised" bubble sort algorithm to sort the array.
	 * This algorithm uses a boolean to check if we've swapped any elements in each loop.
	 *
	 * @return The sorted array
	 */
	@Contract(pure = true)
	public int[] optimisedBubbleSort() {
		int[] array = getInstanceArray();
		boolean swapped = false;

		for (int i = 0; i < array.length - 1; i++) {
			for (int j = 0; j < array.length - 1; j++)
				if (array[j] > array[j + 1]) {
					int left = array[j];
					array[j] = array[j + 1];
					array[j + 1] = left;
					swapped = true;
				}
			if (!swapped)
				break;
			else
				swapped = false;
		}

		return array;
	}

	/**
	 * Time a sorting algorithm in the Sorter class.
	 *
	 * @param method The sorting method to time
	 * @param methodName The name of the method to time
	 */
	@Contract(pure = true)
	public static void timeSort(@NotNull Supplier<int[]> method, String methodName) {
		long start = System.nanoTime();
		int[] result = method.get();
		long end = System.nanoTime();

		String time = String.format("%.4f ms", (float) (end - start) / 1000000);

		if (checkSorted(result))
			System.out.println(methodName + " took " + time);
		else
			System.out.println(methodName + " FAILED in" + time);
	}

	/**
	 * Check whether a given array is sorted in ascending order.
	 *
	 * @param array The array to check
	 * @return Whether the array was sorted
	 */
	@Contract(pure = true)
	private static boolean checkSorted(int @NotNull [] array) {
		for (int i = 0; i < array.length - 1; i++) {
			if (array[i] > array[i + 1])
				return false;
		}
		return true;
	}
}
