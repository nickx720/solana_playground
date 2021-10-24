# Todo Trying out the program,understanding Alice's transaction

```https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/```

**What is a program?**

It owns the transactions, and can only spend what it owns.

---

**What is used to store state?**

Accounts are used to store state.

---

**Do programs own state?**

Yes

---

**Whoc can debit an account and adjust its data?**

Account owner.

---

**What is entrypoint?**

All accounts to be written to or read must be passed into the entrypoint

---

**What is lib.rs used for?**

Registering modules

---

**What is entrypoint.rs used for?**

entrypoint to the program

---

**What is instruction.rs used for?**

Program API, (de) serialising instruction data

---

**What is state.rs?**

Program objects, (de) serialising state.

---

**What is error.rs?**

Program specific errors

---

**What is the flow of program?**

1. entrypoint is called
2. entrypoint fowards the arguements to the processor
3. instrunction.rs is used by the processor to decode the instrunction_data
4. Decoded data is used by the processor to decide which processing function to use to process the request
5. processor may use state.rs to encode/decode state.

---

**What is the data field for?**

data field is to save data inside the accounts.

---

**What holds the token accounts (data field)?**

Token programs.

---

**Does token program hold mint accounts?**

Yes

---

**Does each token account hold a reference to their token mint account?**

Yes

---

**How to transfer ownership of token account to another address?**

Using token programs

---

**Where is all solana internal account information stored?**

Fields on the account

---

**What is sysvars?**

Solana has sysvars, which are parameters of the Solana cluster you are on. It can be accessed through accounts and store parameters such as what the current fee or rent is. As of solana-program `1.6.5`, can be accessed without being passed into entrypoint as an account.

---

**What is Rent sysvar?**

Rent is deducted from an account's balance according to their space requirements.

---

**What happens when balance goes to zero for non rent-exampet accounts?**

The accounts disappear.

---

**Can any accounts be passed into the entrypoint? Different ones from those defined inside the API(instruction.rs)?**

Yes and it is the program's /programmer responsibility to check that `received accounts == expected accounts`

---
