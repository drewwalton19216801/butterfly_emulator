  .org $C000

reset:
  lda #$00
  sta temp
  jmp add_to_20

add_to_20:
  ; Set decimal mode
  sed
  adc #$01
  cld
  sta temp
  cmp #$20 ; Jump to subtract_to_0 if we get to 20
  beq subtract_to_0
  jmp add_to_20

subtract_to_0:
  sed
  sbc #$01
  cld
  sta temp
  cmp #$00 ; Jump to add_to_20 when we get to 0
  beq add_to_20
  jmp subtract_to_0

temp: .byte 0

  .org $fffc
  .word reset
