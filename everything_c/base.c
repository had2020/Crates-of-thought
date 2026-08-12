#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Structs */
struct VmInstruction {
  int opcode;
  int arg1;
  int arg2;
  char code_name[32];
};

/* Functions */
void hi_and_return_void(char name[32]) { printf("hi, %s\n", name); }

// Void would be int if this returned an int.
void swap_w_xor(int *a0, int *a1) {
  *a0 ^= *a1;
  *a1 ^= *a0;
  *a0 ^= *a1;
}

int main() {
  /* Printing */
  char test_char = 'A';
  char msg0[] = "Hello"; // This is a string.
  int a0 = 40;
  double a1 = 3.14;

  printf("Test %d \n", a0);
  printf("My example %s", "this another string\n");
  printf("floating num %f\n", a1);

  /* Hetrotype operations */
  int64_t a2 = 5;
  double a3 = 4.0;

  printf("%f\n", a2 / a3);

  /* Constant vars*/
  const int64_t TEST_NUM = 0b0010;
  const intptr_t TEST_CPTR = 0xFF;

  /* Inputs */
  int fav_num;
  printf("Enter Fav_int_number: \n");
  scanf("%d", &fav_num);
  printf("So your a %der, what char do you pair? \n", fav_num);
  char fav_char;
  // scanf(" %c", &fav_char); // will fail without the whitespace before the %c!
  printf("history: %c, %d\n", fav_char, fav_num);

  char name[32];
  printf("Enter name:\n");
  // scanf("%s", name); // Will stop after whitespace.
  fgets(name, 32, stdin); // Most reliable.
  printf("Hello %s\n", name);

  /* Arrays */
  int numbys[] = {6, 7, 2, 1};
  printf("ex: %d \n", numbys[2]);

  int empty[10]; // Caps at para, but will return voids on uninited.

  /* Using heaps via malloc */
  int *vec_ptr = (int *)malloc(8);

  for (int i = 0; i < 8; i++) {
    vec_ptr[i] = i ^ (i * 2);
  }

  for (int i = 0; i < 8; i++) {
    printf("i(%d) => %d, \n", i, vec_ptr[i]);
  }

  /* Calling functions */
  hi_and_return_void(name);
  int var0 = 4;
  int var1 = 7;
  printf("result of swap 4 and 7,");
  swap_w_xor(&var0, &var1);
  printf("after swap: %d, %d \n", var0, var1);

  /* Branch statements */
  if (var0 != 4) {
    printf("var0 not 4!\n");
  } else if (var0 == 7 && var1 != 4) {
    printf("Perhaps the swap failed?");
  } else {
    printf("Not sure what occurred!");
  }

  /* Switch statements */
  int opcode;
  int8_t reg0 = 2;
  int8_t reg1 = 4;
  printf("Enter a Opcode");
  scanf("%d", &opcode);

  switch (opcode) {
  case 255: {
    printf("output: %d", reg0 + reg1);
    break;
  }
  case 0: {
    reg0 = 5;
    reg1 = 6;
    break;
  }
  default: {
    printf("Invaild Op: %d", opcode);
    break;
  }
  }

  /* Using Structs in CODE */
  struct VmInstruction instuction_var;
  instuction_var.opcode = 0x4F;
  instuction_var.arg1 = 0b1010;
  instuction_var.arg2 = 4;
  strcpy(instuction_var.code_name, "blank_op");

  struct VmInstruction inst_copy;
  inst_copy.opcode = instuction_var.opcode;

  /* While loops */
  int index = 1;
  while (index <= 5) {
    printf("%d ", index);
    index++;
  }

  // do while loops, execute code and then check the condition.
  int idx = 0;
  do {
    idx++;
  } while (idx < 2);

  /* Nested arrays with for loops */
  int result_4D = 0;
  int i, j, k, l;
  const int eles = 32;
  int array_4D[eles][eles][eles][eles] = {}; // all elements  zero
  for (i = 0; i < eles; i++) {
    for (j = 0; j < eles; j++) {
      for (k = 0; k < eles; k++) {
        for (l = 0; l < eles; l++) {
          result_4D += (i * j * k * l);
        }
      }
    }
  }
  printf("\n4D nested result: %d\n", result_4D);
  printf("result mem address = %p\n", &result_4D);

  /* Pointers */
  int *pfav_num = &fav_num;
  printf(
      "your fav num: %d\n",
      *pfav_num); // * derefs the ptr into its type and & turns it into a ptr.

  /* Return success:0 or fails 1 */
  return 0; // After running use echo $? for code.
}
