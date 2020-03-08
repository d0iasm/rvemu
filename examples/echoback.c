int main() {
    while (1) {
        volatile char *uart = (volatile char *) 0x10000000;
        while ((uart[5] & 0x01) == 0);
        char c = uart[0];
        uart[0] = c;
    }
}
