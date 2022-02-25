package org.dyson.sort;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

import org.apache.commons.lang3.ArrayUtils;
import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.Nullable;

import static java.util.Collections.shuffle;

class TimingThread extends Thread {
	private final String methodName;
	private final Supplier<int[]> method;

	@Contract(pure = true)
	TimingThread(String methodName, Supplier<int[]> method) {
		this.methodName = methodName;
		this.method = method;
	}

	@Contract(pure = true)
	public void run() {
		Sorter.timeSort(methodName, method);
	}
}

public class RunSorts {
	@Contract(pure = true)
	public static void main(String @Nullable [] args) {
		int n = (args != null && args.length > 0 && args[0].matches("\\d+")) ?
				Integer.parseInt(args[0]) :
				1000;

		List<Integer> nums = new ArrayList<>();
		for (int i = 0; i < n; i++) nums.add(i);
		shuffle(nums);

		Integer[] wrapperArray = nums.toArray(new Integer[0]);
		Sorter sorter = new Sorter(ArrayUtils.toPrimitive(wrapperArray));

		TimingThread[] timingThreads = {
				new TimingThread("bubbleSort", sorter::bubbleSort),
				new TimingThread("optimisedBubbleSort", sorter::optimisedBubbleSort),
				new TimingThread("recursiveQuicksort", sorter::recursiveQuicksort),
		};

		System.out.println("To sort " + n + " items:\n");
		for (TimingThread thread : timingThreads)
			thread.start();
	}
}
