  .org $C000

; Temporary variable to store the results of multiplication
temp: .byte 0
last_known: .byte 0

reset:
  jsr enable_blinker
  lda #$01
  jmp loop

mult10:
  asl ; multiply by 2
  sta temp ; store the result
  asl ; multiply by 2 again
  asl ; multiply by 2 again
  clc
  adc temp ; add the result of the first multiplication
  jsr blink
  rts

enable_blinker:
  lda #$ff
  sta $8002
  rts

blink:
  ldx temp
  stx $8000
  rts

loop:
  jsr mult10 ; multiply by 10
  ; If zero flag is set, jump to blink_all
  beq blink_all
  jmp loop

blink_all:
  lda #$ff
  sta $8000
  jmp blink_all

  .org $fffc
  .word reset
