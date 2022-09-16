package org.dyson.sort;

import org.apache.commons.lang3.ArrayUtils;
import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.NotNull;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;
import java.util.function.Supplier;
import java.util.stream.Stream;

import static java.util.Collections.shuffle;

public class Sorter {
	private final int[] instanceArray;

	@Contract(pure = true)
	Sorter(int @NotNull [] array) {
		this.instanceArray = array;
	}

	/**
	 * Create an array of integers from 1 to {@code length}, shuffle them, and use this as the instance array.
	 *
	 * @param length The length of the array
	 */
	@Contract(pure = true)
	public static @NotNull Sorter shuffledArray(int length) {
		List<Integer> nums = new ArrayList<>();
		for (int i = 0; i < length; i++) nums.add(i);
		shuffle(nums);

		Integer[] wrapperArray = nums.toArray(new Integer[0]);
		return new Sorter(ArrayUtils.toPrimitive(wrapperArray));
	}

	@Contract(pure = true)
	public int @NotNull [] getInstanceArray() {
		return instanceArray.clone();
	}

	/**
	 * Use {@link java.util.Arrays#stream(Object[])} to sort the instance array with {@link Stream#sorted()}.
	 *
	 * @return The sorted instance array
	 */
	public int @NotNull [] arraysStreamSorted() {
		return Arrays.stream(getInstanceArray()).sorted().toArray();
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
	 * Sort the instance array using an inplace quicksort algorithm.
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] inplaceQuicksort() {
		return staticInplaceQuicksort(getInstanceArray(), 0, -1);
	}

	/**
	 * Partition the given array for a quicksort. This is purely a utility method and should never be used on its own.
	 *
	 * @param array The array to partition
	 * @param pPivot The pivot index
	 * @param pStart The start index
	 * @param pEnd The end index
	 * @return The partition value
	 */
	private static int staticInplaceQuicksortPartition(int @NotNull [] array, int pPivot, int pStart, int pEnd) {
		int vStart = array[pStart];
		array[pStart] = array[pPivot];
		array[pPivot] = vStart;

		int pivot = array[pStart];
		int i = pStart + 1;
		int j = pStart + 1;

		while (j <= pEnd) {
			if (array[j] <= pivot) {
				int vJ = array[j];
				array[j] = array[i];
				array[i] = vJ;
				i++;
			}

			j++;
		}

		vStart = array[pStart];
		array[pStart] = array[i - 1];
		array[i - 1] = vStart;

		return i - 1;
	}

	/**
	 * Sort the given array using an inplace quicksort algorithm.
	 *
	 * This code was adapted from <a href="https://stackoverflow.com/a/17773584/12985838">this StackOverflow answer<a>,
	 * and I don't fully understand it.
	 *
	 * @param array The array to sort
	 * @param start The start index
	 * @param end The end index
	 * @return The sorted array
	 */
	@Contract("_, _, _ -> param1")
	private static int @NotNull [] staticInplaceQuicksort(int @NotNull [] array, int start, int end) {
		if (end < 0) end = array.length - 1;
		if (end - start < 1) return array;

		int i = staticInplaceQuicksortPartition(array, new Random().nextInt(start, end), start, end);
		staticInplaceQuicksort(array, start, i - 1);
		staticInplaceQuicksort(array, i + 1, end);
		return array;
	}

	/**
	 * Sort the instance array using a recursive merge sort algorithm.
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] mergeSort() {
		return staticMergeSort(getInstanceArray());
	}

	/**
	 * Sort the given array with a recursive merge sort algorithm.
	 * To use the instance array, call {@link #mergeSort()}.
	 *
	 * @param array The array to sort
	 * @return The sorted array
	 */
	@Contract(pure = true)
	private static int @NotNull [] staticMergeSort(int @NotNull [] array) {
		if (array.length < 2) return array;

		int mid = array.length / 2;

		int[] left = staticMergeSort(Arrays.stream(array, 0, mid).toArray());
		int[] right = staticMergeSort(Arrays.stream(array, mid, array.length).toArray());

		int li = 0;
		int ri = 0;
		int i = 0;

		// Merge the left and right arrays into the original array memory
		while (li < left.length && ri < right.length) {
			if (left[li] < right[ri]) {
				array[i] = left[li];
				li++;
			} else {
				array[i] = right[ri];
				ri++;
			}
			i++;
		}

		// Only one of the arrays will be non-empty, so one of these while loops won't even run
		while (li < left.length) {
			array[i] = left[li];
			li++;
			i++;
		}

		while (ri < right.length) {
			array[i] = right[ri];
			ri++;
			i++;
		}

		return array;
	}

	/**
	 * Sort the instance array using an inplace insertion sort algorithm.
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] insertionSort() {
		int[] array = getInstanceArray();
		int i;
		int nextItem;

		for (int j = 1; j < array.length; j++) {
			nextItem = array[j];
			i = j - 1;

			while (i >= 0 && array[i] > nextItem) {
				array[i + 1] = array[i];
				i--;
			}

			array[i + 1] = nextItem;
		}

		return array;
	}

	/**
	 * Remove all elements of the instance array that aren't in order
	 *
	 * @return The sorted instance array
	 */
	@Contract(pure = true)
	public int @NotNull [] stalinSort() {
		ArrayList<Integer> arrayList = new ArrayList<>(Arrays.asList(ArrayUtils.toObject(getInstanceArray())));
		int i = 0;
		int highest = 0;

		while (true) {
			try {
				if (arrayList.get(i) > highest){
					highest = arrayList.get(i);
					i++;
				} else {
					arrayList.remove(i);
				}
			} catch (IndexOutOfBoundsException e) {
				break;
			}
		}

		return ArrayUtils.toPrimitive(arrayList.toArray(new Integer[0]));
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
