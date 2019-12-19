main:
  li t3,10
  li t4,1
  blt t4,t3,label1 # if (1 < n)
  j label3
label1:
  li t4,1 # int fib=1;
  li t5,1 # int fibPrev=1;
  li t0,2 # int i=2;
label2:
  bge t0,t3,label3 # if (i<n)
  mv t6,t4 # int temp=fib;
  add t4,t4,t5 # fib+=fibPrev;
  mv t5,t6 # fibPrev=temp;
  addi t0,t0,1 # i++
  j label2
label3:
  mv a0,t4
