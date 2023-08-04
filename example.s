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