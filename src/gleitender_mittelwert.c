// cc gleitender_mittelwert.c -o gleitender_mittelwert
#include<stdio.h>

int main(void) {
    // double a[] = {10.0, 13.0, 42.0, 123.0, 43.0, 66.0, 32.0, 1245.0, };
    double a[] = {1.0, 2.0, 3.0, 6.0, 7.0, 8.0, };
    int anzahl = sizeof(a)/sizeof(double);

    // Alter Wert ist am Anfang der erster Eintrag im Array
    double alter_wert = a[0];
    for(int i = 0; i < anzahl - 1; i++) {
        double temp = a[i];
        a[i] = (alter_wert + a[i] + a[i + 1]) / 3.0;
        alter_wert = temp;
        printf("a[%d] = %f\n", i, a[i]);
    }
    // Letzter Fall
    a[anzahl - 1] = (2.0 * a[anzahl - 1] + alter_wert)/3.0;
    printf("a[%d] = %f\n", anzahl - 1, a[anzahl - 1]);

    return 0;
}
