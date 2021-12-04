#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define MAX_BITS 12
#define MASK 0xFFF;

uint16_t calc_gamma(uint16_t *count_zeroes, uint16_t *count_ones);

int main(void)
{
    char line[MAX_BITS + 2];
    uint16_t count_zeroes[MAX_BITS];
    uint16_t count_ones[MAX_BITS];

    while (fgets(line, sizeof(line), stdin) != NULL) {
        for (uint8_t i = 0; i < MAX_BITS; ++i) {
            if (line[i] == '0') {
                count_zeroes[i]++;
            }
            else {
                count_ones[i]++;
            }
        }
    }

    uint16_t gamma = calc_gamma(count_zeroes, count_ones);
    uint16_t epsilon = ~gamma & MASK;

    printf("%u\n", gamma * epsilon);

    return EXIT_SUCCESS;
}

uint16_t calc_gamma(uint16_t *count_zeroes, uint16_t *count_ones)
{
    uint16_t gamma = 0;

    for (uint8_t i = 0; i < MAX_BITS; ++i) {
        if (count_ones[i] > count_zeroes[i]) {
            gamma |= 1 << (MAX_BITS - i - 1);
        }
    }

    return gamma;
}
