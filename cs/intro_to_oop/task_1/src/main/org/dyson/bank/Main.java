package org.dyson.bank;

import java.util.Scanner;

public class Main {
	private static final Scanner inputScanner = new Scanner(System.in);

	public static void main(String[] args) {
		BankCLI bank = new BankCLI();
		boolean loggedIn = false;
		boolean quitting = false;

		while (!loggedIn && !quitting) {
			System.out.println("Do you have an account? (y/n/quit)");
			String response = inputScanner.nextLine();
			System.out.println();

			switch (response) {
				case "y" -> loggedIn = bank.login();
				case "n" -> bank.addAccount();
				case "quit" -> quitting = true;
				default -> System.out.println("Invalid option\n");
			}

			System.out.println();
		}

		while (!quitting) {
			System.out.println("Press 1 to check your balance");
			System.out.println("Press 2 to deposit money");
			System.out.println("Press 3 to withdraw money");
			System.out.println("Press 4 to exit");

			String option = inputScanner.nextLine();
			System.out.println();

			switch (option) {
				case "1" -> bank.checkBalance();
				case "2" -> {
					bank.deposit();
					bank.checkBalance();
				}
				case "3" -> {
					bank.withdraw();
					bank.checkBalance();
				}
				case "4" -> quitting = true;
				default -> System.out.println("Invalid option\n");
			}

			System.out.println();
		}
	}
}