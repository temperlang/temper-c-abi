#include <stdio.h>

typedef void* geometry_Rectangle;
extern double geometry_Rectangle_area(geometry_Rectangle rectangle);
extern geometry_Rectangle geometry_Rectangle_new(double width, double height);
// extern void geometry_String_free(const char* s);
extern void geometry_init(void);

int main(void) {
    geometry_init();
    geometry_Rectangle rectangle = geometry_Rectangle_new(2.0, 3.0);
    double area = geometry_Rectangle_area(rectangle);
    printf("Area: %g\n", area);
    // hello_String_free(description);
}
