package org.dyson.bank;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class Hasher {
	private final static String salt = "VGhpcyBpcyBqdXN0IHNvbWUgdGV4dCBpbiBiYXNlNjQ" +
			"sIHdoaWNoIG1ha2VzIGl0IGxvb2sgbXVjaCBjb29sZXIuIEFsc28gaGVyZSdzIHNvbWUgb" +
			"W9yZSB3b3JkcyBiZWNhdXNlIHRleHQgbGVuZ3RoIGRlZmluaXRlbHkgbWFrZXMgc2VjdXJ" +
			"pdHkgc3Ryb25nZXIK";

	public static byte[] hash(String text) {
		try {
			MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
			byte[] textBytes = (text + salt).getBytes();
			return sha256.digest(textBytes);
		} catch (NoSuchAlgorithmException e) {
			System.err.println("FATAL ERROR: SHA-256 algorithm not found");
			System.err.println("Using un-hashed password");
			System.err.println("THIS IS INSECURE!!!");
			return text.getBytes();
		}
	}
}
