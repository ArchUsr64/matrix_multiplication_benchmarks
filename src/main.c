unsigned matrix_multiply_c_ffi(int* m1, int* m2, int* result, unsigned size) {
  for(int i = 0; i < size; i++)
    for(int j = 0; j < size; j++)
      for(int k = 0; k < size; k++)
        result[i * size + j] += m1[i * size + k] * m2[k * size + j];
  return 5;
}
