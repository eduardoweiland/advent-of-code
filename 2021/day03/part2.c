#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define REPORT_LINES 1000
#define MAX_BITS 12

#define BIT_AT(pos, val)     (((val) & (1 << (pos))) >> (pos))
#define SET_BIT(pos, val)    ((val) | (1 << (pos)))

void read_input(uint16_t *out);
void find_o2_and_co2(uint16_t *report, uint16_t *o2_val, uint16_t *co2_val);
void check_lines(uint16_t *report, uint8_t bit, uint16_t *mask, uint16_t *val, uint8_t(*meets_criteria)(uint16_t, uint16_t));
uint8_t meets_criteria_o2(uint16_t count_ones, uint16_t considered);
uint8_t meets_criteria_co2(uint16_t count_ones, uint16_t considered);

int main(void)
{
    uint16_t report[REPORT_LINES];
    uint16_t o2_val = 0;
    uint16_t co2_val = 0;

    read_input(report);
    find_o2_and_co2(report, &o2_val, &co2_val);
    printf("%u\n", o2_val * co2_val);

    return EXIT_SUCCESS;
}

void read_input(uint16_t *out)
{
    char line[MAX_BITS + 2];

    for (uint16_t i = 0; i < REPORT_LINES; ++i) {
        if (fgets(line, sizeof(line), stdin) == NULL) {
            break;
        }

        out[i] = strtoul(line, NULL, 2);
    }
}

void find_o2_and_co2(uint16_t *report, uint16_t *o2_val, uint16_t *co2_val)
{
    uint16_t o2_mask = 0;
    uint16_t co2_mask = 0;

    for (int8_t bit = MAX_BITS - 1; bit >= 0; --bit) {
        check_lines(report, bit, &o2_mask, o2_val, meets_criteria_o2);
        check_lines(report, bit, &co2_mask, co2_val, meets_criteria_co2);
    }
}

void check_lines(
    uint16_t *report,
    uint8_t bit,
    uint16_t *mask,
    uint16_t *val,
    uint8_t(*meets_criteria)(uint16_t, uint16_t)
)
{
    uint16_t count_ones = 0;
    uint16_t considered = 0;
    uint16_t last_val = 0;

    for (uint16_t line = 0; line < REPORT_LINES; ++line) {
        if ((report[line] & *mask) == (*val & *mask)) {
            ++considered;
            last_val = report[line];
            count_ones += BIT_AT(bit, report[line]);
        }
    }

    if (considered == 1) {
        *val = last_val;
        *mask = (1 << MAX_BITS) - 1;
        return;
    }

    if (meets_criteria(count_ones, considered)) {
        *val = SET_BIT(bit, *val);
    }

    *mask = SET_BIT(bit, *mask);
}

uint8_t meets_criteria_o2(uint16_t count_ones, uint16_t considered)
{
    if (count_ones >= ceil(considered / 2.0)) {
        return 1;
    }
    return 0;
}

uint8_t meets_criteria_co2(uint16_t count_ones, uint16_t considered)
{
    if (count_ones < ceil(considered / 2.0)) {
        return 1;
    }
    return 0;
}
