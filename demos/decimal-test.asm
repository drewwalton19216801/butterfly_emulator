  .org $C000

reset:
  lda #$ff
  sta $8002

  lda #$01
  sta temp
  jmp add_to_99

add_to_99:
  ; Set decimal mode
  sed
  adc #$01
  cld
  sta temp
  jsr blink
  ; Jump to subtract_to_0 if we get to 50
  cmp #$50
  beq subtract_to_0
  jmp add_to_99

blink:
  ldx temp
  stx $8000
  rts

subtract_to_0:
  sed
  sbc #$01
  cld
  sta temp
  jsr blink ; Blink the LED
  cmp #$00 ; Jump to add_to_99 when we get to 0
  beq add_to_99 
  jmp subtract_to_0

blinkall:
  ldx #$ff
  stx $8000
  jmp blinkall

temp: .byte 0

  .org $fffc
  .word reset
