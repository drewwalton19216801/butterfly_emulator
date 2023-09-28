  .org $C000

; Temporary variable to store the results of multiplication
temp: .byte 0

reset:
  lda #$01
  jmp loop

mult10:
  asl ; multiply by 2
  sta temp ; store the result
  asl ; multiply by 2 again
  asl ; multiply by 2 again
  clc
  adc temp ; add the result of the first multiplication
  rts

loop:
  jsr mult10 ; multiply by 10
  jmp loop

infinitenop:
  nop
  jmp infinitenop

  .org $fffc
  .word reset
