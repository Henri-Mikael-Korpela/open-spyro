# Homeworld names
@at 0x80010000
const LEVEL_NAME_DRAGONX       = "DRAGON X"      # Unused in game. May not be a level title, could be a placeholder name for a dragon name.
@at 0x8001000c
const LEVEL_NAME_THIGH_MASTERS = "THIGH MASTERS" # Unused in game
@at 0x8001001c
const LEVEL_NAME_GNASTYS_WORLD = "GNASTY'S WORLD"
@at 0x8001002c
const LEVEL_NAME_DREAM_WEAVERS = "DREAM WEAVERS"
@at 0x8001003c
const LEVEL_NAME_BEAST_MAKERS  = "BEAST MAKERS"
@at 0x8001004c
const LEVEL_NAME_MAGIC_CRAFTERS = "MAGIC CRAFTERS"
@at 0x8001005c
const LEVEL_NAME_PEACE_KEEPERS  = "PEACE KEEPERS"
@at 0x8001006c
const LEVEL_NAME_ARTISANS       = "ARTISANS"

@at 0x80010078
const INFO_RETURN_HOME = "RETURN HOME"

# Level names for Gnasty's World homeworld
@at 0x80010084
const LEVEL_NAME_GNASTYS_LOOT    = "GNASTY'S LOOT"
@at 0x80010094
const LEVEL_NAME_GNASTY_GNORC    = "GNASTY GNORC"
@at 0x800100a4
const LEVEL_NAME_TWILIGHT_HARBOR = "TWILIGHT HARBOR"
@at 0x800100b4
const LEVEL_NAME_GNORC_COVE      = "GNORC COVE"
@at 0x800100c0
const LEVEL_NAME_GNORC_GNEXUS    = "GNORC GNEXUS"

# Level names for Dream Weavers homeworld
@at 0x800100d0
const LEVEL_NAME_ICY_FLIGHT     = "ICY FLIGHT"
#@at 0x80075568
#const LEVEL_NAME_JACQUES       = "JACQUES" # Located in a different place in memory for some reason
@at 0x800100dc
const LEVEL_NAME_HAUNTED_TOWERS = "HAUNTED TOWERS"
@at 0x800100ec
const LEVEL_NAME_LOFTY_CASTLE   = "LOFTY CASTLE"
@at 0x800100fc
const LEVEL_NAME_DARK_PASSAGE   = "DARK PASSAGE"

# Level names for Beast Makers homeworld
@at 0x8001010c
const LEVEL_NAME_WILD_FLIGHT     = "WILD FLIGHT"
@at 0x80010118
const LEVEL_NAME_METALHEAD       = "METALHEAD"
@at 0x80010124
const LEVEL_NAME_TREE_TOPS       = "TREE TOPS"
@at 0x80010130
const LEVEL_NAME_MISTY_BOG       = "MISTY BOG"
@at 0x8001013c
const LEVEL_NAME_TERRACE_VILLAGE = "TERRACE VILLAGE"

# Level names for Magic Crafters homeworld
@at 0x8001014c
const LEVEL_NAME_CRYSTAL_FLIGHT = "CRYSTAL FLIGHT"
@at 0x8001015c
const LEVEL_NAME_BLOWHARD       = "BLOWHARD"
@at 0x80010168
const LEVEL_NAME_WIZARD_PEAK    = "WIZARD PEAK"
@at 0x80010174
const LEVEL_NAME_HIGH_CAVES     = "HIGH CAVES"
@at 0x80010180
const LEVEL_NAME_ALPINE_RIDGE   = "ALPINE RIDGE"

# Level names for Peace Keepers homeworld
@at 0x80010190
const LEVEL_NAME_NIGHT_FLIGHT = "NIGHT FLIGHT"
@at 0x800101a0
const LEVEL_NAME_DOCTOR_SHEMP = "DOCTOR SHEMP"
@at 0x800101b0
const LEVEL_NAME_ICE_CAVERN   = "ICE CAVERN"
@at 0x800101bc
const LEVEL_NAME_CLIFF_TOWN   = "CLIFF TOWN"
@at 0x800101c8
const LEVEL_NAME_DRY_CANYON   = "DRY CANYON"

# Level names for Artisans homeworld
@at 0x800101d4
const LEVEL_NAME_SUNNY_FLIGHT = "SUNNY FLIGHT"
#@at 0x80075570
#const LEVEL_NAME_TOASTY      = "TOASTY" # Located in a different place in memory for some reason
@at 0x800101e4
const LEVEL_NAME_TOWN_SQUARE  = "TOWN SQUARE"
@at 0x800101f0
const LEVEL_NAME_DARK_HOLLOW  = "DARK HOLLOW"
@at 0x800101fc
const LEVEL_NAME_STONE_HILL   = "STONE HILL"

@at 0x80010ac0
const GAME_MODE_DEMO_MODE = "DEMO MODE"

@at 0x80010acc
const LEVEL_TRANSITION_RETURNING_HOME = "RETURNING HOME..."
@at 0x80010ae0
const LEVEL_TRANSITION_CONFRONTING    = "CONFRONTING %s..."
@at 0x80010af4
const LEVEL_TRANSITION_ENTERING       = "ENTERING %s..."
@at 0x80010b04
const LEVEL_TRANSITION_TREASURE_FOUND = "TREASURE FOUND"
@at 0x80010b14
const LEVEL_TRANSITION_TOTAL_TREASURE = "TOTAL TREASURE"

@at 0x80075578
const LEVEL_TRANSITION_HOME = "HOME"

@at 0x8006f7d0
addr LEVEL_NAME_DRAGONX
addr LEVEL_NAME_ARTISANS
addr LEVEL_NAME_PEACE_KEEPERS
addr LEVEL_NAME_MAGIC_CRAFTERS
addr LEVEL_NAME_BEAST_MAKERS
addr LEVEL_NAME_DREAM_WEAVERS
addr LEVEL_NAME_GNASTYS_WORLD
addr LEVEL_NAME_THIGH_MASTERS

@at 0x8006f7f0
addr LEVEL_TRANSITION_HOME
addr LEVEL_NAME_STONE_HILL
addr LEVEL_NAME_DARK_HOLLOW
addr LEVEL_NAME_TOWN_SQUARE
#addr LEVEL_NAME_TOASTY
@at 0x8006f804
addr LEVEL_NAME_SUNNY_FLIGHT
addr LEVEL_TRANSITION_HOME

@at 0x8006f884
addr DRAGON_NAME_MARCO
addr BALLOONIST_NAME_GOSNOLD
addr BALLOONIST_NAME_TUCO
addr BALLOONIST_NAME_CRAY
addr BALLOONIST_NAME_AMOS
addr BALLOONIST_NAME_HAK

@at 0x8001973c
spy_show_level_transition2:
    lui v0, 26214
    lui a1, 32775
    lw a1, 22708(a1)
    ori v0, v0, 26215
    mult a1, v0
    addiu sp, sp, 65432
    sw ra, 100(sp)
    sw s2, 96(sp)
    sw s1, 92(sp)
    sw s0, 88(sp)
    sra v0, a1, 31
    mfhi t0
    sra a0, t0, 2
    subu a0, a0, v0
    addiu v0, a0, 65535
    sll v1, v0, 1
    addu v1, v1, v0
    sll v1, v1, 1
    sll v0, a0, 2
    addu v0, v0, a0
    sll v0, v0, 1
    subu a0, a1, v0
    bne a0, zero, 7
    addu v1, v1, a0
    lui a1, 32769
    addiu a1, a1, 2764
    jal 101365 # Go to rendering 3D letters from string format
    addiu a0, sp, 56
    j 26116
    nop
        LAB_800197b4:
    slti v0, a1, 60
    beq v0, zero, 3
    addiu v0, zero, 4
    beq a0, v0, 4
    sll v0, v1, 2
        LAB_800197c8:
    addiu v0, zero, 63
    bne a1, v0, 8
    sll v0, v1, 2
        LAB_800197d4:
    lui at, 32775
    addu at, at, v0
    # Entering a boss level
    lw a2, -2064(at)
    lui a1, 32769
    addiu a1, a1, 2784
    j 26114
    addiu a0, sp, 56
        LAB_800197f0:
    addiu a0, sp, 56
    lui at, 32775
    addu at, at, v0
    # Entering a regular level
    lw a2, -2064(at)
    lui a1, 32769
    addiu a1, a1, 2804
        LAB_80019808:
    jal 101365
    nop

# Dragon names
@at 0x800752e8
const DRAGON_NAME_REVILO  = "REVILO"  # In Jacques. Oliver backwards.
@at 0x800752f0
const DRAGON_NAME_RASHIDI = "RASHIDI" # Unused in the final game. Probably was or was intended to be located in Jacques.
@at 0x800752f8
const DRAGON_NAME_UNIKA   = "UNIKA"   # In Jacques.
@at 0x80075300
const DRAGON_NAME_LUTALO  = "LUTALO"  # In Haunted Towers.
@at 0x80075308
const DRAGON_NAME_COPANO  = "COPANO"  # In Haunted Towers.
@at 0x80075310
const DRAGON_NAME_KOSOKO  = "KOSOKO"  # In Haunted Towers.
@at 0x80075318
const DRAGON_NAME_MUDADA  = "MUDADA"  # In Lofty Castle.
@at 0x80075320
const DRAGON_NAME_BARUTI  = "BARUTI"  # In Lofty Castle.
@at 0x80075328
const DRAGON_NAME_USENI   = "USENI"   # In Lofty Castle.
@at 0x80075330
const DRAGON_NAME_OBASI   = "OBASI"   # In Dark Passage.
@at 0x80075338
const DRAGON_NAME_APARA   = "APARA"   # In Dark Passage.
@at 0x80075340
const DRAGON_NAME_BAKARI  = "BAKARI"  # In Dark Passage.
@at 0x80075348
const DRAGON_NAME_AZIZI   = "AZIZI"   # In Dark Passage.
@at 0x80075350
const DRAGON_NAME_KASIYA  = "KASIYA"  # In Dark Passage.
@at 0x80075358
const DRAGON_NAME_LATEEF  = "LATEEF"  # In Dream Weavers homeworld.
@at 0x80075360
const DRAGON_NAME_MAZI    = "MAZI"    # In Dream Weavers homeworld.
@at 0x80075370
const DRAGON_NAME_SADIKI  = "SADIKI"  # In Metalhead.
@at 0x80075378
const DRAGON_NAME_JED     = "JED"     # In Tree Tops.
@at 0x8007537c
const DRAGON_NAME_LYDE    = "LYLE"    # In Tree Tops.
@at 0x80075384
const DRAGON_NAME_ISAAK   = "ISAAK"   # In Tree Tops.
@at 0x8007538c
const DRAGON_NAME_ZEKE    = "ZEKE"    # In Misty Bog.
@at 0x80075394
const DRAGON_NAME_BUBBA   = "BUBBA"   # In Misty Bog.
@at 0x8007539c
const DRAGON_NAME_DAMON   = "DAMON"   # In Misty Bog.
@at 0x800753a4
const DRAGON_NAME_ROSCO   = "ROSCO"   # In Misty Bog.
@at 0x800753ac
const DRAGON_NAME_CYPRIN  = "CYPRIN"  # In Terrace Village.
@at 0x800753b4
const DRAGON_NAME_CLAUDE  = "CLAUDE"  # In Terrace Village.
@at 0x800753bc
const DRAGON_NAME_CLEETUS = "CLEETUS" # In Beast Makers homeworld.
@at 0x800753c4
const DRAGON_NAME_BRUNO   = "BRUNO"   # In Beast Makers homeworld.
@at 0x800753cc
const DRAGON_NAME_JETHRO  = "JETHRO"  # Unused in the final game. Probably was or was intended to be located in one of the Magic Crafters levels.
@at 0x800753d4
const DRAGON_NAME_FINLAY  = "FINLAY"  # Unused in the final game. Probably was or was intended to be located in one of the Magic Crafters levels.
@at 0x800753dc
const DRAGON_NAME_ALTAIR  = "ALTAIR"  # In Blowhard.
@at 0x800753e4
const DRAGON_NAME_LUCAS   = "LUCAS"   # In Wizard Peak.
@at 0x800753ec
const DRAGON_NAME_JARVIS  = "JARVIS"  # In Wizard Peak.
@at 0x800753f4
const DRAGON_NAME_HEXUS   = "HEXUS"   # In Wizard Peak.
@at 0x800753fc
const DRAGON_NAME_CEDRIC  = "CEDRIC"  # In High Caves.
@at 0x80075404
const DRAGON_NAME_AJAX    = "AJAX"    # In High Caves.
@at 0x8007540c
const DRAGON_NAME_CYRUS   = "CYRUS"   # In High Caves.
@at 0x80075414
const DRAGON_NAME_ZANDER  = "ZANDER"  # In Alpine Ridge.
@at 0x8007541c
const DRAGON_NAME_KELVIN  = "KELVIN"  # In Alpine Ridge.
@at 0x80075424
const DRAGON_NAME_ELDRID  = "ELDRID"  # In Alpine Ridge.
@at 0x8007542c
const DRAGON_NAME_ZANE    = "ZANE"    # In Alpine Ridge.
@at 0x80075434
const DRAGON_NAME_BOLDAR  = "BOLDAR"  # In Magic Crafters homeworld.
@at 0x8007543c
const DRAGON_NAME_ZANTOR  = "ZANTOR"  # In Magic Crafters homeworld.
@at 0x80075444
const DRAGON_NAME_COSMOS  = "COSMOS"  # In Magic Crafters homeworld.
@at 0x8007544c
const DRAGON_NAME_TRONDO  = "TRONDO"  # In Doctor Shemp.
@at 0x80075454
const DRAGON_NAME_ASHER   = "ASHER"   # In Ice Cavern.
@at 0x8007545c
const DRAGON_NAME_TODOR   = "TODOR"   # In Ice Cavern.
@at 0x80075464
const DRAGON_NAME_ULRIC   = "ULRIC"   # In Ice Cavern.
@at 0x8007546c
const DRAGON_NAME_RAGNAR  = "RAGNAR"  # In Ice Cavern.
@at 0x80075474
const DRAGON_NAME_ANDOR   = "ANDOR"   # In Ice Cavern.
@at 0x8007547c
const DRAGON_NAME_GALE    = "GALE"    # Unused in the final game. Probably was or was intended to be located in Cliff Town.
@at 0x80075484
const DRAGON_NAME_HALVOR  = "HALVOR"  # In Cliff Town.
@at 0x8007548c
const DRAGON_NAME_ENZO    = "ENZO"    # In Cliff Town.
@at 0x80075494
const DRAGON_NAME_MARCO   = "MARCO"   # In Cliff Town.
@at 0x8007549c
const DRAGON_NAME_THOR    = "THOR"    # In Town Square.
@at 0x800754a4
const DRAGON_NAME_IVOR    = "IVOR"    # In Dry Canyon.
@at 0x800754ac
const DRAGON_NAME_CONAN   = "CONAN"   # In Dry Canyon.
@at 0x800754b4
const DRAGON_NAME_MAXIMOS = "MAXIMOS" # In Dry Canyon.
@at 0x800754bc
const DRAGON_NAME_BORIS   = "BORIS"   # In Dry Canyon.
@at 0x800754c4
const DRAGON_NAME_TITAN   = "TITAN"   # In Peace Keepers homeworld.
@at 0x800754cc
const DRAGON_NAME_GUNNAR  = "GUNNAR"  # In Peace Keepers homeworld.
@at 0x800754d4
const DRAGON_NAME_MAGNUS  = "MAGNUS"  # In Peace Keepers homeworld.
@at 0x800754dc
const DRAGON_NAME_NEVIN   = "NEVIN"   # In Toasty.
@at 0x800754e4
const DRAGON_NAME_ALVAR   = "ALVAR"   # In Town Square.
@at 0x800754ec
const DRAGON_NAME_TOMAS   = "TOMAS"   # In Artisans homeworld.
@at 0x800754f4
const DRAGON_NAME_DEVLIN  = "DEVLIN"  # In Town Square.
@at 0x800754fc
const DRAGON_NAME_NILS    = "NILS"    # In Town Square.
@at 0x80075504
const DRAGON_NAME_ALBAN   = "ALBAN"   # In Dark Hollow.
@at 0x8007550c
const DRAGON_NAME_DARIUS  = "DARIUS"  # In Dark Hollow.
@at 0x80075514
const DRAGON_NAME_OSWIN   = "OSWIN"   # In Dark Hollow.
@at 0x8007551c
const DRAGON_NAME_GAVIN   = "GAVIN"   # In Stone Hill. Name is probably a reference to Andy Gavin,
                                      # who worked for Naughty Dog. Naughty Dog had a close
                                      # relationship with Insomniac Games at the time.
@at 0x80075524
const DRAGON_NAME_ASTOR   = "ASTOR"   # In Stone Hill.
@at 0x8007552c
const DRAGON_NAME_GILDAS  = "GILDAS"  # In Stone Hill.
@at 0x80075534
const DRAGON_NAME_LINDAR  = "LINDAR"  # In Stone Hill.
@at 0x8007553c
const DRAGON_NAME_NESTOR  = "NESTOR"  # In Artisans homeworld.
@at 0x80075544
const DRAGON_NAME_DELBIN  = "DELBIN"  # In Artisans homeworld.
@at 0x8007554c
const DRAGON_NAME_ARGUS   = "ARGUS"   # In Artisans homeworld.
@at 0x80075554
const DRAGON_NAME_SILVUS  = "SILVUS"  # Unused in the final game. Silvus was used in demo builds of the game.

# Balloonist names
@at 0x80075580
const BALLOONIST_NAME_HAK     = "HAK"     # In Gnorc Gnexus
@at 0x80075584
const BALLOONIST_NAME_AMOS    = "AMOS"    # In Dream Weavers
@at 0x8007558c
const BALLOONIST_NAME_CRAY    = "CRAY"    # In Beast Makers
@at 0x80075594
const BALLOONIST_NAME_TUCO    = "TUCO"    # In Magic Crafters
@at 0x8007559c
const BALLOONIST_NAME_GOSNOLD = "GOSNOLD" # In Peace Keepers
# const BALLOONIST_NAME_MARCO = "MARCO"   # In Artisans, shares the same name with a dragon

@at 0x80012204
main:
    addiu sp, sp, 65504
    sw ra, 28(sp)
    sw s2, 24(sp)
    sw s1, 20(sp)
    jal 93794
    sw s0, 16(sp)
    jal 18928
    addiu s2, zero, 1
    addiu s1, zero, 2
    addiu s0, zero, 4
    sb zero, 1540(gp)
    jal 52759
    nop
    lw v0, 1276(gp)
    sb s2, 1540(gp)
    sw v0, 1128(gp)
    slti v0, v0, 2
    beq v0, zero, 2
    nop
    sw s1, 1128(gp)
    lw v0, 1128(gp)
    nop
    slti v0, v0, 5
    bne v0, zero, 2
    nop
    sw s0, 1128(gp)
    lw v0, 1336(gp)
    sw zero, 1276(gp)
    bne v0, zero, -19
    nop
    jal 31575
    nop
    j 18571
    nop

    @at 0x8004363c
    jal 66522
    ori a0, zero, -1031
    bne v0, zero, 608
    nop
    lui a0, 32776
    addiu a0, a0, 35772
    lw v0, 0(a0)
    nop
    bltz v0, 602
    nop
    lui v1, 32776
    lw v1, -29992(v1)
    nop
    slti v0, v1, 61
    bne v0, zero, 16
    slti v0, v1, 16
    lui v0, 32775
    lw v0, 22312(v0)
    addiu v1, zero, 65535
    sw v1, 0(a0)
    sll v0, v0, 2
    lui at, 32775
    addu at, at, v0
    lw a0, -5568(at)
    lui v0, 32775
    lw v0, 22732(v0)
    nop
    jalr ra, v0
    addu a1, zero, zero
    j 69618
    nop

@at 0x8005b8e0
start:
    lui v0, 32775
    addiu v0, v0, 22080
    lui v1, 32776
    addiu v1, v1, 43576
    sw zero, 0(v0)
    addiu v0, v0, 4
    sltu at, v0, v1
    bne at, zero, -4
    nop
    lui v0, 32775
    lw v0, 21928(v0)
    nop
    addi v0, v0, -8
    lui t0, 32768
    or sp, v0, t0
    lui a0, 32776
    addiu a0, a0, 43576
    sll a0, a0, 3
    srl a0, a0, 3
    lui v1, 32775
    lw v1, 21924(v1)
    nop
    subu a1, v0, v1
    subu a1, a1, a0
    lui at, 32775
    sw a1, 12484(at)
    or a0, a0, t0
    lui at, 32775
    sw a0, 12480(at)
    lui at, 32775
    sw ra, 22080(at)
    lui gp, 32775
    addiu gp, gp, 21092
    addu fp, sp, zero
    jal 95941
    addi a0, a0, 4
    lui ra, 32775
    lw ra, 22080(ra)
    nop
    jal 18561
    nop
@at 0x8005db14
InitHeap:
    addiu t2, zero, 160
    jr t2
    addiu t1, zero, 57

@at 0x80017fe4
spy_render_text_as_3d_letters_normal:
    addiu sp, sp, 65496
    sw s0, 16(sp)
    addu s0, a0, zero
    sw s1, 20(sp)
    addu s1, a1, zero
    sw s2, 24(sp)
    addu s2, a2, zero
    sw ra, 32(sp)
    sw s3, 28(sp)
    lbu v1, 0(s0)
    nop
    beq v1, zero, 92
    addu s3, a3, zero
    andi v1, v1, 255
    addiu v0, zero, 32
    beq v1, v0, 80
    addu a1, zero, zero
    lui a0, 32775
    lw a0, 22288(a0)
    nop
    addiu a0, a0, 65448
    lui at, 32775
    sw a0, 22288(at)
    jal 23109
    addiu a2, zero, 88
    lui a0, 32775
    lw a0, 22288(a0)
    addu a1, s1, zero
    jal 24000
    addiu a0, a0, 12
    lbu a0, 0(s0)
    nop
    addiu v0, a0, 65488
    sltiu v0, v0, 10
    beq v0, zero, 5
    addiu v0, a0, 212
    lui v1, 32775
    lw v1, 22288(v1)
    j 24653
    sh v0, 54(v1)
    addiu v0, a0, 65471
    sltiu v0, v0, 26
    beq v0, zero, 5
    addiu v0, a0, 361
    lui v1, 32775
    lw v1, 22288(v1)
    j 24653
    sh v0, 54(v1)
    andi v1, a0, 255
    addiu v0, zero, 47
    bne v1, v0, 5
    addiu v0, zero, 63
    lui v1, 32775
    lw v1, 22288(v1)
    j 24651
    addiu v0, zero, 277
    bne v1, v0, 5
    addiu v0, zero, 37
    lui v1, 32775
    lw v1, 22288(v1)
    j 24651
    addiu v0, zero, 278
    bne v1, v0, 5
    addiu v0, zero, 94
    lui v1, 32775
    lw v1, 22288(v1)
    j 24651
    addiu v0, zero, 272
    bne v1, v0, 5
    addiu v0, zero, 43
    lui v1, 32775
    lw v1, 22288(v1)
    j 24651
    addiu v0, zero, 321
    bne v1, v0, 5
    addiu v0, zero, 327
    lui v1, 32775
    lw v1, 22288(v1)
    j 24651
    addiu v0, zero, 317
    lui v1, 32775
    lw v1, 22288(v1)
    nop
    sh v0, 54(v1)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 127
    sb v0, 71(v1)
    lui v0, 32775
    lw v0, 22288(v0)
    nop
    sb s3, 79(v0)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 255
    sb v0, 80(v1)
    lw v0, 0(s1)
    addiu s0, s0, 1
    addu v0, v0, s2
    sw v0, 0(s1)
    lbu v1, 0(s0)
    nop
    bne v1, zero, -90
    nop
    lui v0, 32775
    lw v0, 22288(v0)
    lw ra, 32(sp)
    lw s3, 28(sp)
    lw s2, 24(sp)
    lw s1, 20(sp)
    lw s0, 16(sp)
    addiu sp, sp, 40
    jr ra
    nop
@at 0x800181ac
spy_render_text_as_3d_letters_large:
    addiu sp, sp, 65488
    sw s1, 20(sp)
    addu s1, a0, zero
    sw s0, 16(sp)
    addu s0, a1, zero
    sw s3, 28(sp)
    addu s3, a2, zero
    sw s4, 32(sp)
    addu s4, a3, zero
    sw s2, 24(sp)
    sw ra, 40(sp)
    sw s5, 36(sp)
    lbu v1, 0(s1)
    lw s5, 64(sp)
    beq v1, zero, 141
    addiu s2, zero, 1
    andi v1, v1, 255
    addiu v0, zero, 32
    beq v1, v0, 121
    addu a1, zero, zero
    lui a0, 32775
    lw a0, 22288(a0)
    nop
    addiu a0, a0, 65448
    lui at, 32775
    sw a0, 22288(at)
    jal 23109
    addiu a2, zero, 88
    lui a0, 32775
    lw a0, 22288(a0)
    addu a1, s0, zero
    jal 24000
    addiu a0, a0, 12
    lbu v1, 0(s1)
    addiu v0, zero, 33
    beq v1, v0, 3
    addiu v0, zero, 63
    bne v1, v0, 2
    nop
    addiu s2, zero, 1
    bne s2, zero, 11
    nop
    lui a0, 32775
    lw a0, 22288(a0)
    lw v1, 4(s3)
    lw v0, 16(a0)
    nop
    addu v0, v0, v1
    sw v0, 16(a0)
    lw v0, 8(s3)
    nop
    sw v0, 20(a0)
    lbu a0, 0(s1)
    nop
    addiu v0, a0, 65488
    sltiu v0, v0, 10
    beq v0, zero, 5
    addiu v0, a0, 212
    lui v1, 32775
    lw v1, 22288(v1)
    j 24797
    sh v0, 54(v1)
    addiu v0, a0, 65471
    sltiu v0, v0, 26
    beq v0, zero, 5
    addiu v0, a0, 361
    lui v1, 32775
    lw v1, 22288(v1)
    j 24797
    sh v0, 54(v1)
    andi v1, a0, 255
    addiu v0, zero, 33
    bne v1, v0, 6
    addiu v0, zero, 44
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 75
    j 24797
    sh v0, 54(v1)
    bne v1, v0, 6
    addiu v0, zero, 63
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 76
    j 24797
    sh v0, 54(v1)
    bne v1, v0, 6
    addiu v0, zero, 46
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 278
    j 24797
    sh v0, 54(v1)
    bne v1, v0, 6
    lui v1, 21845
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 327
    j 24797
    sh v0, 54(v1)
    lui a0, 32775
    lw a0, 22288(a0)
    addiu v0, zero, 76
    sh v0, 54(a0)
    lw v0, 0(s3)
    ori v1, v1, 21846
@at 0x80018728
spy_render_text_as_3d_for_rescuing_dragon:
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
@at 0x8001a40c
spy_render_menu:
    lui v0, 32775
    lw v0, 22712(v0)
    addiu sp, sp, 65200
    sw ra, 332(sp)
    sw fp, 328(sp)
    sw s7, 324(sp)
    sw s6, 320(sp)
    sw s5, 316(sp)
    sw s4, 312(sp)
    sw s3, 308(sp)
    sw s2, 304(sp)
    sw s1, 300(sp)
    bne v0, zero, 104
    sw s0, 296(sp)
    jal 84080
    addu s3, zero, zero
    jal 26022
    addiu s4, zero, 224
    jal 89330
    addiu s1, zero, 512
    jal 82676
    nop
    jal 44659
    nop
    jal 97753
    addu a0, zero, zero
    jal 95985
    addu a0, zero, zero
    lui s5, 32775
    addiu s5, s5, 28384
    lui a0, 32775
    lw a0, 22664(a0)
    jal 98316
    addiu a0, a0, 92
    lui s0, 32776
    addiu s0, s0, 34288
    lui a0, 32775
    lw a0, 22664(a0)
    jal 98166
    addiu s2, s0, 65528
    jal 23009
    addiu a0, zero, 2048
    jal 98137
    addu a0, v0, zero
    jal 97753
    addu a0, zero, zero
    jal 95985
    addu a0, zero, zero
    lui a0, 32775
    lw a0, 22664(a0)
    jal 98316
    addiu a0, a0, 92
    lui v1, 65534
    ori v1, v1, 15872
    addiu a0, sp, 24
    addiu v0, zero, 512
    sh v0, 24(sp)
    addiu v0, zero, 256
    sh zero, 26(sp)
    sh v0, 28(sp)
    lw a1, 0(s0)
    addiu v0, zero, 225
    sh v0, 30(sp)
    jal 97955
    addu a1, a1, v1
    addiu a2, zero, 8
    LAB_8001a514:
    lui v1, 32775
    lw v1, 22664(v1)
    sll v0, s3, 7
    beq v1, s5, 2
    sh v0, 24(sp)
    addiu a2, zero, 248
    LAB_8001a52c:
    addiu a0, sp, 24
    addiu s3, s3, 1
    lw a1, 0(s2)
    addiu v0, zero, 128
    sh a2, 26(sp)
    sh v0, 28(sp)
    jal 97955
    sh s4, 30(sp)
    jal 97753
    addu a0, zero, zero
    lw a0, 0(s2)
    jal 24486
    addiu a1, zero, 28672
    lw a1, 0(s2)
    addiu a0, sp, 24
    sh s1, 24(sp)
    addiu s1, s1, 64
    addiu v0, zero, 64
    sh zero, 26(sp)
    sh v0, 28(sp)
    jal 97930
    sh s4, 30(sp)
    slti v0, s3, 4
    bne v0, zero, -30
    addiu a2, zero, 8
    addiu a0, sp, 24
    lui a1, 32775
    addiu a1, a1, 62224
    addiu v0, zero, 512
    sh v0, 24(sp)
    addiu v0, zero, 224
    sh v0, 26(sp)
    addiu v0, zero, 32
    sh v0, 28(sp)
    addiu v0, zero, 1
    jal 97930
    sh v0, 30(sp)
    jal 97753
    addu a0, zero, zero
    jal 95985
    addiu a0, zero, 65535
    lui at, 32775
    sw v0, 22864(at)
    j 29074
    nop
        LAB_8001a5e0:
    lui v0, 32775
    lw v0, 22664(v0)
    lui a0, 32775
    addiu a0, a0, 28384
    bne v0, a0, 2
    nop
    addiu a0, a0, 132
        LAB_8001a5fc:
    jal 98166
    addu s3, zero, zero
    lui a0, 1
    ori a0, a0, -16384
    lui a1, 65534
    ori a1, a1, 15872
    lui v1, 32776
    lw v1, -31256(v1)
    lui v0, 32776
    lw v0, -31248(v0)
    lui at, 32775
    sw zero, 22704(at)
    lui at, 32775
    sw v1, 22448(at)
    addu v1, v1, a0
    addu v0, v0, a1
    lui at, 32775
    sw v1, 22400(at)
    lui at, 32775
    sw v0, 22268(at)
    lui at, 32775
    sw v0, 22288(at)
    sll v1, s3, 7
        LAB_8001a658:
    addiu a3, s3, 136
    lui s0, 32775
    lw s0, 22448(s0)
    lui v0, 2304
    sw v0, 0(s0)
    addiu v0, zero, 44
    sb v0, 7(s0)
    addiu v0, zero, 76
    sb v0, 4(s0)
    addiu v0, zero, 128
    sb v0, 5(s0)
    addiu v0, zero, 8
    sh v1, 8(s0)
    sh v0, 10(s0)
    lhu v0, 8(s0)
    lhu a1, 10(s0)
    lhu a2, 8(s0)
    lhu v1, 10(s0)
    addiu v0, v0, 128
    sh v0, 16(s0)
    lhu v0, 8(s0)
    addiu v1, v1, 223
    sh v1, 26(s0)
    lhu v1, 10(s0)
    addiu s3, s3, 1
    sb zero, 13(s0)
    sh a1, 18(s0)
    lbu a1, 13(s0)
    addiu s4, zero, 64
    sb zero, 12(s0)
    sh a2, 24(s0)
    lbu a2, 12(s0)
    addiu v0, v0, 128
    sh v0, 32(s0)
    lbu v0, 12(s0)
    addiu v1, v1, 223
    sh v1, 34(s0)
    lbu v1, 13(s0)
    addu a0, s0, zero
    sb s4, 6(s0)
    sb a1, 21(s0)
    sb a2, 28(s0)
    addiu v0, v0, 128
    sb v0, 20(s0)
    lbu v0, 12(s0)
    addiu v1, v1, 65503
    sb v1, 29(s0)
    lbu v1, 13(s0)
    addiu v0, v0, 128
    addiu v1, v1, 65503
    sb v0, 36(s0)
    addiu v0, zero, 14368
    sb v1, 37(s0)
    sh v0, 14(s0)
    jal 23095
    sh a3, 22(s0)
    addiu a0, s0, 40
    slti v0, s3, 4
    lui at, 32775
    sw a0, 22448(at)
    bne v0, zero, -61
    sll v1, s3, 7
    lui v0, 768
    addiu v1, zero, 231
    sw v0, 40(s0)
    addiu v0, zero, 512
    sb s4, 47(s0)
    sh zero, 48(s0)
    sh v1, 50(s0)
    sh v0, 52(s0)
    sh v1, 54(s0)
    sb zero, 44(s0)
    sb zero, 45(s0)
    jal 23095
    sb zero, 46(s0)
    addiu a0, s0, 56
    lui v1, 32775
    lw v1, 22488(v1)
    addiu v0, zero, 2
    lui at, 32775
    sw a0, 22448(at)
    bne v1, v0, 939
    addiu v0, zero, 3
    addu s1, a0, zero
    addiu a1, zero, 1
    addu a2, zero, zero
    addiu a3, zero, 64
    jal 98716
    sw zero, 16(sp)
    jal 23095
    addu a0, s1, zero
    lui v0, 1280
    sw v0, 68(s0)
    addiu v0, zero, 42
    sb v0, 75(s0)
    addiu v0, zero, 67
    sb s4, 72(s0)
    sb s4, 73(s0)
    sb s4, 74(s0)
    lui v1, 32775
    lw v1, 22472(v1)
    sh v0, 78(s0)
    sh v0, 82(s0)
    addiu v0, zero, 1
    bne v1, v0, 12
    addiu s1, s0, 68
    addiu v0, zero, 84
    sh v0, 76(s0)
    lui v0, 32775
    lw v0, 22232(v0)
    addiu v1, zero, 428
    sh v1, 80(s0)
    sltu v0, zero, v0
    sb v0, 160(sp)
    sll v0, v0, 4
    j 27154
    addiu v0, v0, 182
        LAB_8001a82c:
    addiu t1, zero, 1
    addiu v0, zero, 140
    sb t1, 160(sp)
    sh v0, 76(s0)
    addiu v0, zero, 372
    sh v0, 80(s0)
    addiu v0, zero, 176
        LAB_8001a848:
    sh v0, 86(s0)
    sh v0, 90(s0)
    lhu v0, 8(s1)
    lhu v1, 12(s1)
    addu a0, s1, zero
    sh v0, 16(s1)
    jal 23095
    sh v1, 20(s1)
    addiu a0, zero, 224
    addiu a1, zero, 97
    addiu a2, zero, 288
    addiu v0, s1, 24
    lui at, 32775
    sw v0, 22448(at)
    jal 24851
    addiu a3, zero, 97
    lui v1, 32775
    lw v1, 22472(v1)
    addiu v0, zero, 1
    bne v1, v0, 23
    addiu a0, zero, 140
    addiu a0, zero, 84
    addiu a1, zero, 67
    addiu a2, zero, 428
    jal 24851
    addiu a3, zero, 67
    addiu a0, zero, 428
    addiu a1, zero, 67
    lbu t1, 160(sp)
    addiu a2, zero, 428
    sll s0, t1, 4
    addiu s0, s0, 182
    jal 24851
    addu a3, s0, zero
    addiu a0, zero, 428
    addu a1, s0, zero
    addiu a2, zero, 84
    jal 24851
    addu a3, s0, zero
    addiu a0, zero, 84
    addu a1, s0, zero
    j 27219
    addiu a2, zero, 84
        LAB_8001a8f4:
    addiu a1, zero, 67
    addiu a2, zero, 372
    jal 24851
    addiu a3, zero, 67
    addiu a0, zero, 372
    addiu a1, zero, 67
    lbu v0, 160(sp)
    addiu a2, zero, 372
    sll s0, v0, 3
    addu s0, s0, v0
    sll s0, s0, 1
    addiu s0, s0, 158
    jal 24851
    addu a3, s0, zero
    addiu a0, zero, 372
    addu a1, s0, zero
    addiu a2, zero, 140
    jal 24851
    addu a3, s0, zero
    addiu a0, zero, 140
    addu a1, s0, zero
    addiu a2, zero, 140
        LAB_8001a94c:
    jal 24851
    addiu a3, zero, 67
    lui a0, 32775
    addiu a0, a0, 21952
    addiu s0, sp, 64
    addu a1, s0, zero
    addiu a2, zero, 28
    addiu a3, zero, 11
    addiu v0, zero, 186
    sw v0, 64(sp)
    addiu v0, zero, 82
    sw v0, 68(sp)
    addiu v0, zero, 3072
    jal 24569
    sw v0, 72(sp)
    lui v0, 32775
    lw v0, 22472(v0)
    addiu fp, zero, 2
    # If current menu is Quit game menu
    bne v0, fp, 86
    addiu s6, zero, 1
    lui a0, 32769
    addiu a0, a0, 2852
    addu a1, s0, zero
    addiu s1, sp, 80
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 16
    addiu s7, zero, 1
    sw v0, 80(sp)
    addiu v0, zero, 5120
    sw v0, 88(sp)
    addiu v0, zero, 183
    sw v0, 64(sp)
    addiu v0, zero, 119
    addiu s4, zero, 4352
    addiu s3, zero, 11
    sw s7, 84(sp)
    sw v0, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui a0, 32775
    addiu a0, a0, 21960
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 192
    addiu s6, zero, 148
    sw v0, 64(sp)
    sw s6, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, zero, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 3
        LAB_8001aa40:
    lui a0, 32775
    addiu a0, a0, 21964
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 296
    sw v0, 64(sp)
    sw s6, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, s7, 5
    andi a1, s5, 255
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 2
    andi a1, s5, 255
        LAB_8001aa90:
    blez a1, 1706
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
        LAB_8001aaa4:
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    j 29007
    nop
        LAB_8001aaf0:
    bne v0, s6, 558
    addu a1, s0, zero
    lui a0, 32769
    addiu a0, a0, 2864
    addiu s1, sp, 96
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 15
    sw v0, 96(sp)
    addiu v0, zero, 5632
    addiu s7, zero, 107
    sw v0, 104(sp)
    addiu v0, zero, 108
    addiu s4, zero, 5120
    addiu s3, zero, 11
    sw s6, 100(sp)
    sw s7, 64(sp)
    sw v0, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, zero, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 11
    lui a0, 32769
    addiu a0, a0, 2880
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 121
    sw v0, 64(sp)
    addiu v0, zero, 124
    sw v0, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, s6, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 11
    lui a0, 32769
    addiu a0, a0, 2896
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 140
    sw s7, 64(sp)
    sw v0, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, fp, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 12
    lui v0, 32775
    lw v0, 22232(v0)
    nop
    beq v0, zero, 20
    addu a1, s0, zero
    lui a0, 32769
    addiu a0, a0, 2912
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 163
    sw v0, 64(sp)
    addiu v0, zero, 156
    sw v0, 68(sp)
    sw s4, 72(sp)
    jal 24683
    sw s3, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 3
    bne v1, v0, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 9
    lui a0, 32775
    addiu a0, a0, 21968
    addiu s3, sp, 64
    addu a1, s3, zero
    addiu s1, sp, 96
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 208
    addiu s6, zero, 4352
    lbu t1, 160(sp)
    addiu s4, zero, 11
    sw v0, 64(sp)
    sw s6, 72(sp)
    sll s0, t1, 4
    addiu v0, s0, 156
    sw v0, 68(sp)
    jal 24683
    sw s4, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 4
    bne v1, v0, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 6
    lui a0, 32775
    addiu a0, a0, 21976
    addu a1, s3, zero
    addu a2, s1, zero
    addiu a3, zero, 16
    addiu v0, zero, 238
    sw v0, 64(sp)
    addiu v0, s0, 172
    sw v0, 68(sp)
    sw s6, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 5
    bne v1, v0, 5
    andi a1, s5, 255
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 4
    andi a1, s5, 255
    blez a1, 21
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    lui v0, 32775
    lw v0, 22356(v0)
    nop
    bne v0, zero, 43
    addiu a1, sp, 64
    lui a0, 32775
    addiu a0, a0, 21984
    addiu a2, sp, 96
    addiu a3, zero, 16
    addiu v0, zero, 322
    sw v0, 64(sp)
    addiu v0, zero, 108
    sw v0, 68(sp)
    addiu v0, zero, 5120
    sw v0, 72(sp)
    addiu v0, zero, 11
    jal 24683
    sw v0, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, zero, 87
    addu a0, zero, zero
    lui s2, 32775
    lw s2, 22288(s2)
    addu s3, zero, zero
    lui a1, 32775
    addiu a1, a1, 52344
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a1
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slti v0, s3, 3
    bne v0, zero, -16
    addiu s2, s2, 88
    j 27594
    nop
    blez v0, 61
    addu s3, zero, zero
    lui s2, 32775
    addiu s2, s2, 52344
    addu s1, zero, zero
    addiu s0, zero, 320
    addu a1, zero, zero
    lui a0, 32775
    lw a0, 22288(a0)
    nop
    addiu a0, a0, 65448
    lui at, 32775
    sw a0, 22288(at)
    jal 23109
    addiu a2, zero, 88
    lui a0, 32775
    lw a0, 22288(a0)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 434
    sh v0, 54(a0)
    addiu v0, zero, 109
    sw v0, 16(a0)
    addiu v0, zero, 5120
    sw s0, 12(a0)
    bne v1, zero, 16
    sw v0, 20(a0)
    lui v0, 32775
    lw v0, 22712(v0)
    nop
    sll v0, v0, 3
    addu v0, v0, s1
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, s2
    lh v1, 0(v0)
    nop
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    j 27575
    sb v0, 70(a0)
    sb zero, 70(a0)
    addiu s1, s1, 12
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 127
    sb v0, 71(v1)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 11
    sb v0, 79(v1)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 255
    sb v0, 80(v1)
    lui v0, 32775
    lw v0, 22356(v0)
    addiu s3, s3, 1
    slt v0, s3, v0
    bne v0, zero, -55
    addiu s0, s0, 10
    lui v0, 32775
    lw v0, 22344(v0)
    nop
    bne v0, zero, 43
    addiu a1, sp, 64
    lui a0, 32775
    addiu a0, a0, 21984
    addiu a2, sp, 96
    addiu a3, zero, 16
    addiu v0, zero, 322
    sw v0, 64(sp)
    addiu v0, zero, 124
    sw v0, 68(sp)
    addiu v0, zero, 5120
    sw v0, 72(sp)
    addiu v0, zero, 11
    jal 24683
    sw v0, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 1
    bne v1, v0, 88
    addu a0, zero, zero
    lui s2, 32775
    lw s2, 22288(s2)
    addu s3, zero, zero
    lui a1, 32775
    addiu a1, a1, 52344
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a1
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slti v0, s3, 3
    bne v0, zero, -16
    addiu s2, s2, 88
    j 27704
    nop
    blez v0, 62
    addu s3, zero, zero
    lui s2, 32775
    addiu s2, s2, 52344
    addu s1, zero, zero
    addiu s0, zero, 320
    addu a1, zero, zero
    lui a0, 32775
    lw a0, 22288(a0)
    nop
    addiu a0, a0, 65448
    lui at, 32775
    sw a0, 22288(at)
    jal 23109
    addiu a2, zero, 88
    lui a0, 32775
    lw a0, 22288(a0)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 434
    sh v0, 54(a0)
    addiu v0, zero, 125
    sw v0, 16(a0)
    addiu v0, zero, 5120
    sw v0, 20(a0)
    addiu v0, zero, 1
    bne v1, v0, 16
    sw s0, 12(a0)
    lui v0, 32775
    lw v0, 22712(v0)
    nop
    sll v0, v0, 3
    addu v0, v0, s1
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, s2
    lh v1, 0(v0)
    nop
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    j 27685
    sb v0, 70(a0)
    sb zero, 70(a0)
    addiu s1, s1, 12
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 127
    sb v0, 71(v1)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 11
    sb v0, 79(v1)
    lui v1, 32775
    lw v1, 22288(v1)
    addiu v0, zero, 255
    sb v0, 80(v1)
    lui v0, 32775
    lw v0, 22344(v0)
    addiu s3, s3, 1
    slt v0, s3, v0
    bne v0, zero, -56
    addiu s0, s0, 10
    lui v1, 32775
    lw v1, 25152(v1)
    addiu v0, zero, 322
    sw v0, 64(sp)
    addiu v0, zero, 140
    sw v0, 68(sp)
    addiu v0, zero, 5120
    beq v1, zero, 11
    sw v0, 72(sp)
    addiu v0, zero, 11
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 21988
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    j 27732
    addiu a1, zero, 4
    addiu v0, zero, 11
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 21996
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    addiu a1, zero, 6
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 2
    bne v1, v0, 25
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    beq a1, zero, 21
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    lui v0, 32775
    lw v0, 22232(v0)
    nop
    beq v0, zero, 57
    addiu v0, zero, 322
    lui v1, 32775
    lw v1, 22436(v1)
    sw v0, 64(sp)
    addiu v0, zero, 156
    sw v0, 68(sp)
    addiu v0, zero, 5120
    beq v1, zero, 11
    sw v0, 72(sp)
    addiu v0, zero, 11
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 22004
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    j 27793
    addiu a1, zero, 2
    addiu v0, zero, 11
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 21984
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    addiu a1, zero, 3
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 3
    bne v1, v0, 25
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    beq a1, zero, 21
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    lbu t1, 160(sp)
    lui v1, 32775
    lw v1, 22804(v1)
    addiu v0, zero, 322
    sw v0, 64(sp)
    sll v0, t1, 4
    addiu v0, v0, 156
    sw v0, 68(sp)
    addiu v0, zero, 5120
    sw v0, 72(sp)
    addiu v0, zero, 2
    bne v1, v0, 10
    addiu v0, zero, 11
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 22008
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    j 27852
    addiu a1, zero, 6
    sw v0, 16(sp)
    lui a0, 32775
    addiu a0, a0, 22016
    addiu a1, sp, 64
    addiu a2, sp, 96
    jal 24683
    addiu a3, zero, 16
    addiu a1, zero, 7
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 4
    bne v1, v0, 1151
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    beq a1, zero, 1147
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    j 29007
    nop
    lui a0, 32769
    addiu a0, a0, 2924
    addiu s1, sp, 112
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 16
    sw v0, 112(sp)
    addiu v0, zero, 5120
    sw v0, 120(sp)
    addiu v0, zero, 199
    sw v0, 64(sp)
    addiu v0, zero, 110
    addiu s3, zero, 4352
    addiu s4, zero, 11
    sw s6, 116(sp)
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, zero, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 8
    lui a0, 32775
    addiu a0, a0, 22024
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 207
    sw v0, 64(sp)
    addiu v0, zero, 128
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, s6, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 7
    lui a0, 32769
    addiu a0, a0, 2936
    addu a1, s0, zero
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu s6, zero, 191
    addiu v0, zero, 146
    sw s6, 64(sp)
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v0, 32775
    lw v0, 22304(v0)
    nop
    bne v0, fp, 4
    nop
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 9
    lui v0, 32775
    lw v0, 22160(v0)
    nop
    beq v0, zero, 21
    addu a1, s0, zero
    lui a0, 32775
    addiu a0, a0, 22032
    addu a2, s1, zero
    addiu a3, zero, 18
    addiu v0, zero, 231
    sw v0, 64(sp)
    addiu v0, zero, 164
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 3
    bne v1, v0, 57
    andi a1, s5, 255
    lui s2, 32775
    lw s2, 22288(s2)
    j 28026
    addiu s5, zero, 4
    lui v0, 26214
    lui a0, 32775
    lw a0, 22892(a0)
    ori v0, v0, 26215
    mult a0, v0
    sra v0, a0, 31
    mfhi t1
    sra v1, t1, 2
    subu v1, v1, v0
    sll v0, v1, 2
    addu v0, v0, v1
    sll v0, v0, 1
    beq a0, v0, 20
    addu a2, s1, zero
    lui a0, 32769
    addiu a0, a0, 2948
    addiu a3, zero, 18
    addiu v0, zero, 183
    sw v0, 64(sp)
    addiu v0, zero, 164
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 3
    bne v1, v0, 24
    andi a1, s5, 255
    lui s2, 32775
    lw s2, 22288(s2)
    j 28026
    addiu s5, zero, 9
    lui a0, 32769
    addiu a0, a0, 2960
    addu a1, s0, zero
    addiu a3, zero, 18
    addiu v0, zero, 164
    sw s6, 64(sp)
    sw v0, 68(sp)
    sw s3, 72(sp)
    jal 24683
    sw s4, 16(sp)
    lui v1, 32775
    lw v1, 22304(v1)
    addiu v0, zero, 3
    bne v1, v0, 5
    andi a1, s5, 255
    lui s2, 32775
    lw s2, 22288(s2)
    addiu s5, zero, 8
    andi a1, s5, 255
    blez a1, 979
    addu s3, zero, zero
    lui a2, 32775
    addiu a2, a2, 52344
    addu a0, zero, zero
    lui v0, 32775
    lw v0, 22712(v0)
    addiu s3, s3, 1
    sll v0, v0, 3
    addu v0, v0, a0
    andi v0, v0, 255
    sll v0, v0, 1
    addu v0, v0, a2
    lh v1, 0(v0)
    addiu a0, a0, 12
    sll v0, v1, 1
    addu v0, v0, v1
    sra v0, v0, 9
    sb v0, 70(s2)
    slt v0, s3, a1
    bne v0, zero, -16
    addiu s2, s2, 88
    j 29007
    nop
    bne v1, v0, 955
    nop
    jal 44744
    nop
    addiu v1, zero, 440
    sw v1, 64(sp)
    addiu v1, zero, 44
    sw v1, 68(sp)
    addiu v1, zero, 2304
    addu a2, v0, zero
    slti v0, a2, 10
    bne v0, zero, 3
    sw v1, 72(sp)
    addiu v0, zero, 406
    sw v0, 64(sp)
    slti v0, a2, 100
    bne v0, zero, 5
    addiu s2, sp, 32
    lw v0, 64(sp)
    nop
    addiu v0, v0, 65502
    sw v0, 64(sp)
    addu a0, s2, zero
@at 0x80062fd4
spy_render_text_as_3d_letters_format:
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

@at 0x8006b670
# Erases the data in the given buffer according to given count.
# The erasure is done by writing 0 to the buffer.
bzero:
    beq a0, zero, 9
    addu v0, zero, zero
    bgtz a1, 3
    addu v0, a0, zero
    j 109990
    addu v0, zero, zero
    sb zero, 0(a0)
    addiu a1, a1, 65535
    bgtz a1, -3
    addiu a0, a0, 1