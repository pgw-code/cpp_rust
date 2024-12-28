

# Rust-Backed Linked List with a C++ Wrapper

This project implements a **linked list** in Rust for safety and performance while providing a **C++ wrapper** to interact with it seamlessly. Rust handles all the memory management and ensures safety, while the C++ wrapper offers a familiar interface for C++ developers.

---

## Features

- **Rust Implementation**: The linked list is implemented entirely in Rust for safe memory management.
- **C++ Wrapper**: A clean C++ interface is provided for easy usage.
- **Functions Supported**:
  - Add a node at the beginning, end, or a specific position.
  - Remove a node from the beginning, end, or a specific position.
  - Print the list.

---

## Prerequisites

### Rust
- Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)

### C++ Compiler
- A C++ compiler that supports C++11 or later.
- GCC, Clang, or MSVC.

---

## Build Instructions

### 1. Clone the Repository



---

### 2. Build the Rust Library


   ```

1. Build the Rust library:
   ```bash
   cargo build --release
   ```

2. Locate the compiled shared library:
   - On **Linux**: `target/release/liblinked_list.so`
   - On **macOS**: `target/release/liblinked_list.dylib`
   - On **Windows**: `target/release/linked_list.dll`

---

### 3. Build the C++ Wrapper


1. Compile the C++ code and link it with the Rust library:
   ```bash
   g++ main.cpp -o main -L../rust/target/release -llinked_list -ldl
   ```

   - Replace `../rust/target/release` with the path to the Rust shared library if it's in a different location.

---

### 4. Run the Program

Run the compiled executable:
```bash
./main
```

---

## Example Usage

### C++ Interface

Here’s how you can use the linked list via the C++ wrapper:

```cpp
#include <iostream>
#include "LinkedListWrapper.h"

int main() {
    LinkedListWrapper list;

    list.addAtEnd(10);
    list.addAtEnd(20);
    list.addAtBeginning(5);
    list.addAtPosition(15, 2);

    std::cout << "List after additions: ";
    list.printList();

    list.removeFromEnd();
    list.removeFromBeginning();
    list.removeAtPosition(1);

    std::cout << "List after removals: ";
    list.printList();

    return 0;
}
```

### Output

```text
List after additions: 5 -> 10 -> 15 -> 20 -> None
List after removals: 10 -> None
```

---


## How It Works

1. **Rust Implementation**:
   - The linked list is implemented entirely in Rust.
   - Functions are exposed via `#[no_mangle]` and `extern "C"` for compatibility with C++.

2. **C++ Wrapper**:
   - The C++ wrapper provides a clean interface (`LinkedListWrapper`) for interacting with the Rust-backed linked list.
   - Memory management and safety are handled by Rust.

3. **Memory Management**:
   - Rust owns and manages all memory for the linked list.
   - The C++ wrapper simply calls Rust functions via FFI.

---

## Advantages

- **Memory Safety**: Rust ensures no memory leaks or undefined behavior.
- **Seamless Integration**: The C++ wrapper makes the Rust implementation easy to use for C++ developers.
- **Performance**: Rust's zero-cost abstractions ensure high performance.

---

## Future Enhancements

- Add support for more linked list operations (e.g., reversing the list, finding elements).
- Implement error handling to translate Rust's `Result` and `Option` into meaningful C++ error codes.
- Optimize for larger datasets by minimizing FFI calls.

---

## Contributing

Contributions are welcome! Please submit issues or pull requests via GitHub.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

