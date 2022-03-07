package org.dyson.bank;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

class Console {
	public static String readLine() {
		BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

		try {
			return br.readLine();
		}
		catch (IOException ioe) {
			System.out.println("IO Error reading from command line.");
			return "";
		}
	}
}
