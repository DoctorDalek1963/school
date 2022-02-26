package org.dyson.sort;

import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.NotNull;

import java.util.Arrays;
import java.util.function.Supplier;

public final class Sorter {
	private final int[] instanceArray;

	@Contract(pure = true)
	Sorter(int @NotNull [] array) {
		this.instanceArray = array;
	}

	@Contract(pure = true)
	public int @NotNull [] getInstanceArray() {
		return instanceArray.clone();
	}

	/**
	 * Use a simple bubble sort algorithm to sort the instance array.
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] bubbleSort() {
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
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] optimisedBubbleSort() {
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
	 * Use a recursive, FP-inspired quicksort algorithm to sort the instance array.
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] recursiveQuicksort() {
		return staticRecursiveQuicksort(getInstanceArray());
	}

	/**
	 * Sort the array using a recursive, FP-inspired quicksort algorithm.
	 * To use the instance array, call {@link #recursiveQuicksort()}.
	 *
	 * @param array The array to sort
	 * @return The sorted array
	 */
	@Contract(pure = true)
	private static int @NotNull [] staticRecursiveQuicksort(int @NotNull [] array) {
		if (array.length < 2) return array;

		int pivot = array[0];
		int[] slicedArray = Arrays.stream(array, 1, array.length).toArray();

		int[] lower = Arrays.stream(slicedArray).filter(e -> e < pivot).toArray();
		int[] higher = Arrays.stream(slicedArray).filter(e -> e >= pivot).toArray();

		System.arraycopy(staticRecursiveQuicksort(lower), 0, array, 0, lower.length);
		array[lower.length] = pivot;
		System.arraycopy(staticRecursiveQuicksort(higher), 0, array, lower.length + 1, higher.length);

		return array;
	}

	/**
	 * Time a sorting algorithm in the Sorter class.
	 *
	 * @param method The sorting method to time
	 * @param methodName The name of the method to time
	 */
	@Contract(pure = true)
	public static void timeSort(String methodName, @NotNull Supplier<int[]> method) {
		long start = System.nanoTime();
		int[] result = method.get();
		long end = System.nanoTime();

		String time = String.format("%.4f ms", (float) (end - start) / 1000000);

		if (checkSorted(result))
			System.out.println(methodName + " took " + time);
		else
			System.out.println(methodName + " FAILED in " + time);
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
