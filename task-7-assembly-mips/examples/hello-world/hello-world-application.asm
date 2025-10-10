##################################################################
#
#   MIPS assembly code example
#   - Hello World as application 
#
#   Author: Tobias Hansson <tohans@kth.se>
#   Modified by: Viola Söderlund <violaso@kth.se>
#
#   Created: 2020-10-23
#
#   See: MARS Syscall Sheet (https://courses.missouristate.edu/KenVollmar/mars/Help/SyscallHelp.html)
#   See: MIPS Documentation (https://ecs-network.serv.pacific.edu/ecpe-170/tutorials/mips-instruction-set)
#
##################################################################

### Data Declaration Section ###

.data                               
HW:     .asciiz "Hello World\n"     #define label HW as our hello world string

### Executable Code Section ###

.text

main:
    # print HW
    li      $v0, 4                  # magic code to print string
    la      $a0, HW                 # load address of string HW into $a0
    syscall                         # HW now printed

    # exit program
    li $v0, 10                      # set system call code to "terminate program"
    syscall                         # terminate program

##################################################################
#
#   NOTE:
#       Applications assembled and executed by MARS, or applications which terminate at EOF,
#       don't need the termination call. This termination call is a formality.
#
##################################################################
