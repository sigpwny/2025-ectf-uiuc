/* Development memory layout using direct startup from flash without bootloader */
MEMORY {
   ROM         (rx) : ORIGIN = 0x00000000, LENGTH = 0x00010000 /* 64kB ROM */
   FLASH       (rx) : ORIGIN = 0x10000000, LENGTH = 0x00038000 /* Team firmware */
   RESERVED_BL (rx) : ORIGIN = 0x10038000, LENGTH = 0x0000E000 /* Reserved */
   RESERVED    (rw) : ORIGIN = 0x10046000, LENGTH = 0x00038000 /* Reserved */
   ROM_BL_PAGE (rw) : ORIGIN = 0x1007E000, LENGTH = 0x00002000 /* Reserved */
   RAM         (rwx): ORIGIN = 0x20000000, LENGTH = 0x00010000 /* 64kB RAM */
}

SECTIONS {
   .flash_code :
   {
      . = ALIGN(4);
      *(.flashprog*)
      . = ALIGN(4);
   } > RAM AT>FLASH
}

INSERT AFTER .data;