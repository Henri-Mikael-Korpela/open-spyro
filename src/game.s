# Homeworld names
@at 0x80010000
let level_title_dragonx       = "DRAGON X"
@at 0x8001000c
let level_title_thigh_masters = "THIGH MASTERS"
@at 0x8001001c
let level_title_gnastys_world = "GNASTY'S WORLD"
@at 0x8001002c
let level_title_dream_weavers = "DREAM WEAVERS"
@at 0x8001003c
let level_title_beast_makers  = "BEAST MAKERS"
@at 0x8001004c
let level_name_magic_crafters = "MAGIC CRAFTERS"
@at 0x8001005c
let level_name_peace_keepers  = "PEACE KEEPERS"
@at 0x8001006c
let level_name_artisans       = "ARTISANS"

@at 0x80010078
let info_return_home = "RETURN HOME"

# Level names for Gnasty's World homeworld
@at 0x80010084
let level_name_gnastys_loot    = "GNASTY'S LOOT"
@at 0x80010094
let level_name_gnasty_gnorc    = "GNASTY GNORC"
@at 0x800100a4
let level_name_twilight_harbor = "TWILIGHT HARBOR"
@at 0x800100b4
let level_name_gnorc_cove      = "GNORC COVE"
@at 0x800100c0
let level_name_gnorc_gnexus    = "GNORC GNEXUS"

# Level names for Dream Weavers homeworld
@at 0x800100d0
let level_name_icy_flight     = "ICY FLIGHT"
@at 0x800100dc
let level_name_haunted_towers = "HAUNTED TOWERS"
@at 0x800100ec
let level_name_lofty_castle   = "LOFTY CASTLE"
@at 0x800100fc
let level_name_dark_passage   = "DARK PASSAGE"

# Level names for Beast Makers homeworld
@at 0x8001010c
let level_name_wild_flight     = "WILD FLIGHT"
@at 0x80010118
let level_name_metalhead       = "METALHEAD"
@at 0x80010124
let level_name_tree_tops       = "TREE TOPS"
@at 0x80010130
let level_name_misty_bog       = "MISTY BOG"
@at 0x8001013c
let level_name_terrace_village = "TERRACE VILLAGE"

# Level names for Magic Crafters homeworld
@at 0x8001014c
let level_name_crystal_flight = "CRYSTAL FLIGHT"
@at 0x8001015c
let level_name_blowhard       = "BLOWHARD"
@at 0x80010168
let level_name_wizard_peak    = "WIZARD PEAK"
@at 0x80010174
let level_name_high_caves     = "HIGH CAVES"
@at 0x80010180
let level_name_alpine_ridge   = "ALPINE RIDGE"

# Level names for Peace Keepers homeworld
@at 0x80010190
let level_name_night_flight = "NIGHT FLIGHT"
@at 0x800101a0
let level_name_doctor_shemp = "DOCTOR SHEMP"
@at 0x800101b0
let level_name_ice_cavern   = "ICE CAVERN"
@at 0x800101bc
let level_name_cliff_town   = "CLIFF TOWN"
@at 0x800101c8
let level_name_dry_canyon   = "DRY CANYON"

# Level names for Artisans homeworld
# Note that Toasty is missing for some reason,
# it's not close to the level names for this homeworld in memory
@at 0x800101d4
let level_name_sunny_flight = "SUNNY FLIGHT"
@at 0x800101e4
let level_name_town_square  = "TOWN SQUARE"
@at 0x800101f0
let level_name_dark_hollow  = "DARK HOLLOW"
@at 0x800101fc
let level_name_stone_hill   = "STONE HILL"

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