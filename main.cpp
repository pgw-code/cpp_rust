#include <iostream>
#include <string>
#include <memory>
#include <cstring>

extern "C" {
    struct LinkedList;
    LinkedList* linked_list_new();
    void linked_list_add_at_beginning(LinkedList* list, int value);
    void linked_list_add_at_end(LinkedList* list, int value);
    char* linked_list_print(LinkedList* list);
    void linked_list_free(LinkedList* list);
    void linked_list_free_string(char* s);
}

class LinkedListWrapper {
    LinkedList* list;
public:
    LinkedListWrapper() {
        list = linked_list_new();
    }

    ~LinkedListWrapper() {
        linked_list_free(list);
    }

    void addAtBeginning(int value) {
        linked_list_add_at_beginning(list, value);
    }

    void addAtEnd(int value) {
        linked_list_add_at_end(list, value);
    }

    void printList() {
        char* result = linked_list_print(list);
        if (result) {
            std::cout << result << std::endl;
            linked_list_free_string(result);
        }
    }
};

int main() {
    LinkedListWrapper list;
    list.addAtEnd(10);
    list.addAtEnd(20);
    list.addAtBeginning(5);

    std::cout << "List: ";
    list.printList();

    return 0;
}
