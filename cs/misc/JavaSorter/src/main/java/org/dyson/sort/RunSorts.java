package org.dyson.sort;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Supplier;

import org.apache.commons.lang3.ArrayUtils;
import org.jetbrains.annotations.Nullable;

import static java.util.Collections.shuffle;

public class RunSorts {
	public static void main(String @Nullable [] args) {
		int n;
		try {
			assert args != null;
			n = Integer.parseInt(args[0]);
		} catch (ArrayIndexOutOfBoundsException | NumberFormatException | NullPointerException e) {
			n = 1000;
		}

		List<Integer> nums = new ArrayList<>();
		for (int i = 0; i < n; i++) nums.add(i);
		shuffle(nums);

		Integer[] wrapperArray = nums.toArray(new Integer[0]);
		Sorter sorter = new Sorter(ArrayUtils.toPrimitive(wrapperArray));

		HashMap<String, Supplier<int[]>> methods = new HashMap<>();
		methods.put("bubbleSort", sorter::bubbleSort);
		methods.put("optimisedBubbleSort", sorter::optimisedBubbleSort);

		System.out.println("To sort " + n + " items:\n");
		for (String methodName : methods.keySet())
			Sorter.timeSort(methods.get(methodName), methodName);
	}
}
