/* Copyright (c) 2017  SiFive Inc. All rights reserved.

   This copyrighted material is made available to anyone wishing to use,
   modify, copy, or redistribute it subject to the terms and conditions
   of the FreeBSD License.   This program is distributed in the hope that
   it will be useful, but WITHOUT ANY WARRANTY expressed or implied,
   including the implied warranties of MERCHANTABILITY or FITNESS FOR
   A PARTICULAR PURPOSE.  A copy of this license is available at
   http://www.opensource.org/licenses.
*/

#include "newlib.h"



#=========================================================================
# crt0.S : Entry point for RISC-V user programs
#=========================================================================

.text
.global _enter
.type   _enter, @function
.global _uart8250_init
.type   _uart8250_init, @function
.global _sp
  
_enter:
  # Initialize global pointer
.option push
.option norelax
1:auipc gp, %pcrel_hi(__global_pointer$)
  addi  gp, gp, %pcrel_lo(1b)
.option pop
  la    sp, _sp

clear_bss:
  # Clear the bss segment
  la      a0, metal_segment_bss_target_start    # Load start of bss
  la      a2, metal_segment_bss_target_end      # Load end of bss
  sub     a2, a2, a0                            # Get size of bss section
  li      a1, 0                                 # Load fill value
  call    memset

  # Exit handling
  la      a0, __libc_fini_array   # Register global termination functions
  call    atexit                  #  to be called upon exit
  call    __libc_init_array       # Run global initialization functions

  # Call UART initialization function
  lw      a0, 0(sp)                  # a0 = argc
  addi    a1, sp, __SIZEOF_POINTER__ # a1 = argv
  li      a2, 0                      # a2 = envp = NULL
  call    _uart8250_init

  # Call main
  lw      a0, 0(sp)                  # a0 = argc
  addi    a1, sp, __SIZEOF_POINTER__ # a1 = argv
  li      a2, 0                      # a2 = envp = NULL
  call    main
  tail    exit
  .size  _enter, .-_enter
