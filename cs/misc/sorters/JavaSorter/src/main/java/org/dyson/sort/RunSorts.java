package org.dyson.sort;

import java.util.function.Supplier;

import org.jetbrains.annotations.Contract;
import org.jetbrains.annotations.Nullable;

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
				10000;

		Sorter sorter = Sorter.shuffledArray(n);

		TimingThread[] timingThreads = {
				new TimingThread("arraysStreamSorted", sorter::arraysStreamSorted),
				new TimingThread("bubbleSort", sorter::bubbleSort),
				new TimingThread("optimisedBubbleSort", sorter::optimisedBubbleSort),
				new TimingThread("recursiveQuicksort", sorter::recursiveQuicksort),
				new TimingThread("inplaceQuicksort", sorter::inplaceQuicksort),
				new TimingThread("mergeSort", sorter::mergeSort),
				new TimingThread("insertionSort", sorter::insertionSort),
				new TimingThread("stalinSort", sorter::stalinSort),
		};

		System.out.println("To sort " + n + " items:\n");
		for (TimingThread thread : timingThreads)
			thread.start();
	}
}
