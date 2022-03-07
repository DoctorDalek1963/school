package org.dyson.bank;

import java.util.Arrays;

public class Account {
	private final int accountNumber;
	private final byte[] hashedPassword;
	private float balance;

	//A new bank account should be defined with a given account number, password and balance
	Account (int number, String password, float balance) {
		this.accountNumber = number;
		this.hashedPassword = Hasher.hash(password);
		this.balance = balance;
	}

	public int getNumber() {
		return accountNumber;
	}

	public boolean checkPassword(String password) {
		return Arrays.equals(Hasher.hash(password), hashedPassword);
	}

	public float getBalance() {
		return balance;
	}

	public void setBalance(float newBalance) {
		balance = newBalance;
	}
}