package org.dyson.bank;

import java.util.ArrayList;
import java.util.Random;

public class BankBackend {
	private final ArrayList<Account> accounts;
	private int currentAccountNumber;

	BankBackend() {
		this.accounts = new ArrayList<>();
		this.currentAccountNumber = -1;
	}

	/**
	 * Return the current account.
	 *
	 * @return The current account
	 * @throws NoSuchAccountException If the account doesn't exist (login failed)
	 */
	private Account getCurrentAccount() throws NoSuchAccountException {
		for (Account account : accounts) {
			if (account.getNumber() == this.currentAccountNumber) {
				return account;
			}
		}
		throw new NoSuchAccountException("No account with number " + this.currentAccountNumber);
	}

	/**
	 * Log a customer into the bank.
	 *
	 * @param accountNumber The user's account number
	 * @param password The user's password
	 * @return Whether the customer was logged in successfully
	 */
	public boolean login(int accountNumber, String password) {
		for (Account account : accounts) {
			if (account.getNumber() == accountNumber && account.checkPassword(password)) {
				this.currentAccountNumber = accountNumber;
				return true;
			}
		}
		this.currentAccountNumber = -1;
		return false;
	}

	/**
	 * Log the current customer out of their account.
	 */
	public void logout() {
		this.currentAccountNumber = -1;
	}

	/**
	 * Deposit money into the customer's account.
	 *
	 * @param amount The amount of money to deposit
	 * @throws ArithmeticException If the amount is negative
	 * @throws NoSuchAccountException If the account doesn't exist (login failed)
	 */
	public void deposit(float amount) throws ArithmeticException, NoSuchAccountException {
		if (amount < 0) throw new ArithmeticException("Can only deposit positive amounts");
		getCurrentAccount().setBalance(getCurrentAccount().getBalance() + amount);
	}

	/**
	 * Withdraw money from the customer's account.
	 *
	 * @param amount The amount of money to withdraw
	 * @throws ArithmeticException If the amount is negative or more than the balance
	 * @throws NoSuchAccountException If the account doesn't exist (login failed)
	 */
	public void withdraw(float amount) throws ArithmeticException, NoSuchAccountException {
		if (amount < 0) throw new ArithmeticException("Can only withdraw positive amounts");
		if (getCurrentAccount().getBalance() < amount) throw new ArithmeticException("Insufficient funds");
		getCurrentAccount().setBalance(getCurrentAccount().getBalance() - amount);
	}

	/**
	 * Return the balance of the account.
	 *
	 * @return The account balance
	 * @throws NoSuchAccountException If the account doesn't exist (login failed)
	 */
	public float getBalance() throws NoSuchAccountException {
		return getCurrentAccount().getBalance();
	}

	/**
	 * Add an account to the bank's database, using a random 10-digit number.
	 *
	 * @param password The user's chosen password
	 * @return The new account number
	 */
	public int addAccount(String password) {
		// We're generating a random 8-digit number here
		// Collisions are possible, but very unlikely
		int accountNumber = new Random().nextInt(89999999) + 10000000;
		accounts.add(new Account(accountNumber, password, 0));
		return accountNumber;
	}
}