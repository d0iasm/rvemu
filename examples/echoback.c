int main() {
    while (1) {
        volatile char *uart = (volatile char *) 0x10000000;
        while ((uart[5] & 0x01) == 0);
        char c = uart[0];
        if ('a' <= c && c <= 'z') {
            c = c + 'A' - 'a';
        }
        uart[0] = c;
    }
}

