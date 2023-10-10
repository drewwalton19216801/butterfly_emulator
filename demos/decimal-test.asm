  .org $C000

reset:
  lda #$00
  sta temp
  jmp add_to_20

add_to_20:
  clc ; Clear carry
  sed ; Set decimal mode
  adc #$01 ; Add 1 to A
  cld ; Clear decimal mode
  sta temp ; Store A in temp
  cmp #$20 ; Compare A to 20
  beq subtract_to_0 ; If A = 20, jump to subtract_to_0
  jmp add_to_20

subtract_to_0:
  sed ; Set decimal mode
  sbc #$01 ; Subtract 1 from A
  cld ; Clear decimal mode
  sta temp ; Store A in temp
  cmp #$00 ; Compare A to 0
  beq add_to_20 ; If A = 0, jump to add_to_20
  jmp subtract_to_0

temp: .byte 0

  .org $fffc
  .word reset
