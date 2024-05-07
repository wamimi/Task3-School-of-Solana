## On-Chain Calculator Program
This repository contains an on-chain calculator program implemented using the Solana blockchain and the Anchor framework. The purpose of this program is to demonstrate how to create a simple calculator application where arithmetic operations can be performed on-chain.

## Features Implemented
The following features were intended to be implemented in this program:

Initialization of Calculator: The program allows initializing the calculator with two operands (x and y) and a designated update authority.
Updating Operands: The program provides functions to update the values of the operands x and y separately.
Updating Authority: A function is provided to update the authority of the calculator, allowing control over who can modify the calculator's data.
Arithmetic Operations:
Addition
Subtraction
Multiplication
Division
Event Emission: After each arithmetic operation, an event is emitted containing the operands x and y, the result of the operation, and the type of operation performed.
Implementation Details
## The implementation consists of the following main components:

Program: The Solana program logic is implemented using the Anchor framework in Rust. It defines the entry points for initializing the calculator, updating operands and authority, and performing arithmetic operations.
Accounts: The program defines the structure of the calculator account, which includes the operands (x and y) and the update authority.
Error Codes: Custom error codes are defined to handle cases where the caller does not have sufficient privileges to perform certain operations.
Events: Events are emitted after each arithmetic operation to notify listeners about the performed operation and its result.
## Usage
To interact with this on-chain calculator program, users can invoke the provided methods using Solana transactions. They can initialize the calculator, update operands and authority, and perform arithmetic operations using the specified entry points.

## Testing
The program includes a test suite written in TypeScript using the Anchor client library. The tests cover various scenarios, including initialization, operand updates, arithmetic operations, error handling, and event emission verification.
