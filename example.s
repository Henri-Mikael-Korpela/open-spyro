spy_render_text_as_3d_letters_large:
    @at 0x800181ac
    addiu sp, sp, 65488
    sw s1, 20(sp)
spy_render_text_as_3d_for_rescuing_dragon:
    @at 0x80018728
    lui v0, 0x8007
    lw v0, 0x70C0(v0)
    addiu sp, sp, 0xFFC8
    sw ra, 0x0030(sp)
    sw s3, 44(sp)
    sw s2, 40(sp)
    sw s1, 36(sp)
    sw s0, 32(sp)
    lw v0, 0(v0)
    nop
    lw v0, 56(v0)
    nop
spy_render_text_as_3d_letters_format:
    @at 0x80062fd4
    sw a1, 4(sp)
    sw a2, 8(sp)
    sw a3, 12(sp)
    addiu sp, sp, 64952
    sw s3, 564(sp)
    addu s3, a0, zero
    addiu v0, sp, 592
    sw ra, 580(sp)
    sw s6, 576(sp)
    sw s5, 572(sp)
    sw s4, 568(sp)
    sw s2, 560(sp)
    sw s1, 556(sp)
    sw s0, 552(sp)
    sw a1, 588(sp)
    sw v0, 544(sp)