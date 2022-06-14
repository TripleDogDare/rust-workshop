#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "ffi.h"

int main(void) {

    {
        uint32_t result = example1(1);
        assert(result == 2);
    }

    {
        struct Vec_uint32 vec = example2_1(2);
        uint32_t result = example2_2(&vec); // note taking a reference
        assert(result == 2);
        example2_drop(vec);
        // example2_drop(vec); // uncommenting this leads to a double-free and a segfault
    }

    {
        struct Vec_uint32 vec = example2_1(2);
        uint32_t result = example2_2_consume(vec);
        assert(result == 2);
        //example2_drop(vec); // uncommenting this leads to a double-free and a segfault
    }

    {
        struct Example3 s = example3_make();
        assert(s.x == 0 && !s.y);
        example3_fill(&s);
        assert(s.x == 3 && s.y);
    }

    {
        uint32_t x;
        struct Example4 *s = example4_make();

        x = example4_read(s);
        assert(x == 1);

        x = example4_read_box(&s);
        assert(x == 1);

        example4_mutate(s);

        x = example4_read(s);
        assert(x == 10);

        // x = example4_read(NULL); // uncommenting this results in a segfault

        example4_drop(s);
    }

    {
        uint32_t x;
        struct Example4 *s = example4_make();

        x = example4a_read(s);
        assert(x == 1);

        // Passing NULL is fine too
        x = example4a_read(NULL);
        assert(x == 4);

        example4_drop(s);
    }

    {
        uint32_t x;

        example5_value(&x);
        assert(x == 10);

        bool result;
        struct Example4 *s;

        result = example5_pointer(&s);
        assert(result);
        x = example4_read(s);
        assert(x == 1);
        example4_drop(s);

        result = example5_nullable(NULL);
        assert(!result);
    }

    return 0;
}
