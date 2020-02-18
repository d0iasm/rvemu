use plain::Plain;

use crate::*;

const EI_NIDENT: usize = 16;

// Elf32 types
type Elf32Byte = u8;
type Elf32Half = u16;
type Elf32Word = u32;
type Elf32Addr = u32;
type Elf32Off = u32;

// Elf64 types
type Elf64Byte = u8;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64XWord = u64;
type Elf64Addr = u64;
type Elf64Off = u64;

/// 32-bit ELF header (Ehdr)
pub struct Elf32Ehdr {
    e_indent: [Elf32Byte; EI_NIDENT], // Elf identification
    e_type: Elf32Half,                // Object file type
    e_machine: Elf32Half,             // Machine type
    e_version: Elf32Word,             // Object file version
    e_entry: Elf32Addr,               // Entry point address
    e_phoff: Elf32Off,                // Program header offset
    e_shoff: Elf32Off,                // Section header offset
    e_flags: Elf32Word,               // Processor-specific flags
    e_ehsize: Elf32Half,              // ELF header size
    e_phentsize: Elf32Half,           // Size of program header entry
    e_phnum: Elf32Half,               // Number of program header entries
    e_shentsize: Elf32Half,           // Size of section header entry
    e_shnum: Elf32Half,               // Number of section header entries
    e_shstrndx: Elf32Half,            // Section name string table index
}

/// 64-bit ELF header (Ehdr)
#[repr(C)]
#[derive(Default, Debug)]
pub struct Elf64Ehdr {
    /// Elf identification
    e_indent: [Elf64Byte; EI_NIDENT],
    /// Object file type
    e_type: Elf64Half,
    /// Machine type
    e_machine: Elf64Half,
    /// Object file version
    e_version: Elf64Word,
    /// Entry point address
    e_entry: Elf64Addr,
    /// Program header offset
    e_phoff: Elf64Off,
    /// Section header offset
    e_shoff: Elf64Off,
    /// Processor-specific flags
    e_flags: Elf64Word,
    /// ELF header size
    e_ehsize: Elf64Half,
    /// Size of program header entry
    e_phentsize: Elf64Half,
    /// Number of program header entries
    e_phnum: Elf64Half,
    /// Size of section header entry
    e_shentsize: Elf64Half,
    /// Number of section header entries
    e_shnum: Elf64Half,
    /// Section name string table index
    e_shstrndx: Elf64Half,
}

unsafe impl Plain for Elf64Ehdr {}

impl Elf64Ehdr {
    pub fn new(binary: &[u8]) -> &Self {
        if binary.len() < 64 {
            output(&format!(
                "expected binary size is more than 512 bits, but got {}",
                binary.len() * 8
            ))
        }
        plain::from_bytes(binary).expect("failed to get ELF header from a raw binary")
    }

    pub fn verify(&self) -> bool {
        true
    }
}

// 32-bit program header (Phdr)
pub struct Elf32Phdr {
    p_type: Elf32Word,   // Type of segment
    p_offset: Elf32Off,  // Offset in file
    p_vaddr: Elf32Addr,  // Virtual address in memory
    p_paddr: Elf32Addr,  // Physical address in memory (if applicable)
    p_filesz: Elf32Word, // Size of segment in file
    p_memsz: Elf32Word,  // Size of segment in memory
    p_flags: Elf32Word,  // Segment attributes
    p_align: Elf32Word,  // Alignment of segment
}

// 64-bit program header (Phdr)
pub struct Elf64Phdr {
    p_type: Elf64Word,    // Type of segment
    p_flags: Elf64Word,   // Segment attributes
    p_offset: Elf64Off,   // Offset in file
    p_vaddr: Elf64Addr,   // Virtual address in memory
    p_paddr: Elf64Addr,   // Physical address in memory (if applicable)
    p_filesz: Elf64XWord, // Size of segment in file
    p_memsz: Elf64XWord,  // Size of segment in memory
    p_align: Elf64XWord,  // Alignment of segment
}

// 32-bit section header (Shdr)
pub struct Elf32Shdr {
    sh_name: Elf32Word,      // Section name
    sh_type: Elf32Word,      // Section type
    sh_flags: Elf32Word,     // Section attributes
    sh_addr: Elf32Addr,      // Virtual memory address
    sh_offset: Elf32Off,     // Offset in file
    sh_size: Elf32Word,      // Size in section
    sh_link: Elf32Word,      // Link to other section
    sh_info: Elf32Word,      // Miscellaneous information
    sh_addralign: Elf32Word, // Address alignment boundary
    sh_entsize: Elf32Word,   // Size of entries, if section has table
}

// 64-bit section header (Shdr)
pub struct Elf64Shdr {
    sh_name: Elf64Word,       // Section name
    sh_type: Elf64Word,       // Section type
    sh_flags: Elf64XWord,     // Section attributes
    sh_addr: Elf64Addr,       // Virtual memory address
    sh_offset: Elf64Off,      // Offset in file
    sh_size: Elf64XWord,      // Size in section
    sh_link: Elf64Word,       // Link to other section
    sh_info: Elf64Word,       // Miscellaneous information
    sh_addralign: Elf64XWord, // Address alignment boundary
    sh_entsize: Elf64XWord,   // Size of entries, if section has table
}

// 32-bit string and symbol tables
pub struct Elf32Sym {
    st_name: Elf32Word,  // Symbol name
    st_value: Elf32Addr, // Symbol value
    st_size: Elf32Word,  // Size of object
    st_info: Elf32Byte,  // Type and binding attributes
    st_other: Elf32Byte, // Visibility
    st_shndx: Elf32Half, // Section table index
}

// 64-bit string and symbol tables
pub struct Elf64Sym {
    st_name: Elf64Word,  // Symbol name
    st_info: Elf64Byte,  // Type and binding attributes
    st_other: Elf64Byte, // Visibility
    st_shndx: Elf64Half, // Section table index
    st_value: Elf64Addr, // Symbol value
    st_size: Elf64XWord, // Size of object
}
