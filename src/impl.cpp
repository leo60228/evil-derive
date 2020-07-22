#include "header.hpp"

int non_virtual::x() {
    return value;
}

non_virtual::non_virtual(int v) : value(v) {}

int base::x() {
    return value;
}

base::base(int v) : value(v) {}

int derived::x() {
    return value + 1;
}

derived::derived(int v) : base(v) {}

int call_x_on(base* x) {
    return x->x();
}
