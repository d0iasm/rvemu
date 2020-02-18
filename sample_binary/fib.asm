main:
addi    sp,sp,-48
sw      s0,40(sp)
addi    s0,sp,48
li      a5,10
sw      a5,-32(s0)
lw      a5,-32(s0)
addi	a4,a5,0 #sext.w  a4,a5
li      a5,1
blt     a5,a4,label1
lw      a5,-32(s0)
j       label4
label1:
li      a5,1
sw      a5,-20(s0)
li      a5,1
sw      a5,-24(s0)
li      a5,2
sw      a5,-28(s0)
label2:
lw      a4,-28(s0)
lw      a5,-32(s0)
addi 	a4,a4,0 #sext.w  a4,a4
addi 	a4,a4,0 #sext.w  a5,a5
bge     a4,a5,label3
lw      a5,-20(s0)
sw      a5,-36(s0)
lw      a4,-20(s0)
lw      a5,-24(s0)
add    	a5,a5,a4
sw      a5,-20(s0)
lw      a5,-36(s0)
sw      a5,-24(s0)
lw      a5,-28(s0)
addi   	a5,a5,1
sw      a5,-28(s0)
j       label2
label3:
lw      a5,-20(s0)
label4:
mv      a0,a5
lw      s0,40(sp)
addi    sp,sp,48
nop
