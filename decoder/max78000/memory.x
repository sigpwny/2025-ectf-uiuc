MEMORY {
   ROM         (rx) : ORIGIN = 0x00000000, LENGTH = 0x00010000 /* 64kB ROM */
   BOOTLOADER  (rx) : ORIGIN = 0x10000000, LENGTH = 0x0000E000 /* Bootloader flash */
   FLASH       (rx) : ORIGIN = 0x1000E000, LENGTH = 0x00032000 /* Location of team firmware */
   SECRETS     (rw) : ORIGIN = 0x10040000, LENGTH = 0x00016000 /* Reserved */
   RESERVED    (rw) : ORIGIN = 0x10056000, LENGTH = 0x00028000 /* Reserved */
   ROM_BL_PAGE (rw) : ORIGIN = 0x1007E000, LENGTH = 0x00002000 /* Reserved */
   RAM         (rwx): ORIGIN = 0x20000000, LENGTH = 0x00010000 /* 64kB RAM */
}

_stext = ORIGIN(FLASH) + 0x200; /* Jump point for bootloader */

SECTIONS {
   .flash_code :
   {
      . = ALIGN(4);
      *(.flashprog*)
      . = ALIGN(4);
   } > RAM AT>FLASH
}

INSERT AFTER .data;