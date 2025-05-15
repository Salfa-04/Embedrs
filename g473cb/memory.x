MEMORY
{
    /* FLASH and RAM are mandatory memory regions */

    /* STM32G473xB   */
    FLASH  : ORIGIN = 0x08000000, LENGTH = 128K

    /* FLASH  : ORIGIN = 0x08000000, LENGTH = 64K */
    /* FLASH1 : ORIGIN = 0x08040000, LENGTH = 64K */

    /* SRAM */
    SRAM1 : ORIGIN = 0x20000000, LENGTH = 80K
    SRAM2 : ORIGIN = 0x20014000, LENGTH = 16K

    /* CCM SRAM : ICODE        */
    CSRAM : ORIGIN = 0x10000000, LENGTH = 32K
    /* DCODE : Alias for ICODE */
    DSRAM : ORIGIN = 0x20018000, LENGTH = 32K
}

/* Define the RAM alias to SRAM1 */
REGION_ALIAS(RAM, SRAM1)

SECTIONS {
    .csram (NOLOAD) : ALIGN(4) {
        *(.csram .csram.*);
        . = ALIGN(4);
    } > CSRAM

    .dsram (NOLOAD) : ALIGN(4) {
        *(.dsram .dsram.*);
        . = ALIGN(4);
} > DSRAM

    .sram2 (NOLOAD) : ALIGN(4) {
        *(.sram2 .sram2.*);
        . = ALIGN(4);
    } > SRAM2
};
