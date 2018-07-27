    .global __syscall1
__syscall1:
  push    {r7, lr}
  mov     r7, r0
  mov     r0, r1
  svc     0
  pop     {r7, pc}
