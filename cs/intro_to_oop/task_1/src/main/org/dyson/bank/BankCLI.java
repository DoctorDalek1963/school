package org.dyson.bank;

import java.util.Scanner;

public class BankCLI {
	private static final Scanner inputScanner = new Scanner(System.in);
	private final BankBackend backend;

	BankCLI() {
		this.backend = new BankBackend();
	}

	public boolean login() {
		System.out.print("Enter your account number: ");
		int accountNumber = inputScanner.nextInt();

		System.out.print("Enter your password: ");
		String password = Console.readLine();

		System.out.println();

		boolean login = backend.login(accountNumber, password);
		if (login) {
			System.out.println("Successfully logged in");
		} else {
			System.out.println("Login failed");
		}
		return login;
	}

	public void deposit() {
		System.out.print("Please enter the amount to deposit: ");
		float amount = inputScanner.nextFloat();

		try {
			backend.deposit(amount);
		} catch (NoSuchAccountException e) {
			System.out.println("Please login first");
		} catch (ArithmeticException e) {
			System.out.println(e.getMessage());
		}
	}

	public void withdraw() {
		System.out.print("Please enter the amount to withdraw: ");
		float amount = inputScanner.nextFloat();

		try {
			backend.withdraw(amount);
		} catch (NoSuchAccountException e) {
			System.out.println("Please login first");
		} catch (ArithmeticException e) {
			System.out.println(e.getMessage());
		}
	}

	public void checkBalance() {
		try {
			System.out.printf("You have £%.2f\n", backend.getBalance());
		} catch (NoSuchAccountException e) {
			System.out.println("Please login first");
		}
	}

	public void addAccount() {
		System.out.print("Please enter a password for your new account: ");
		String password = inputScanner.nextLine();
		int accountNumber = backend.addAccount(password);
		System.out.println("Your new account number is: " + accountNumber);
	}
}