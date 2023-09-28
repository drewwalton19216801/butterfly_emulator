  .org $C000

reset:
  lda #$ff
  sta $8002

  lda #$50
  sta $8000

loop:
  ror
  sta $8000

  jmp loop

temp: .byte 0

  .org $fffc
  .word reset
