#include <cassert>
#include <ctime>
#include <string>

std::string generate_string(uint32_t size) {
    char chr[] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A',
                  'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                  'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
                  'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                  'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
                  't', 'u', 'v', 'w', 'x', 'y', 'z'};
    std::string ret;
    srand(time(NULL));
    for (int i = 0; i < size; i++) {
        int idx = rand() % 62;
        ret.push_back(chr[idx]);
    }
    assert(ret.length() == size);
    return ret;
}