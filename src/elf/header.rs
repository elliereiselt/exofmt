use crate::stringable_consts_blocks::stringable_consts_block;

pub struct Header {
    pub e_ident: [u8; 16],
    /// Identifies object file type
    pub e_type: u16,
    /// Specifies target instruction set architecture
    pub e_machine: u16,
    /// Set to `1` for the original version of ELF
    pub e_version: u32,
    /// This is the memory address of the entry point from where the process starts executing
    pub e_entry: u64,
    /// Points to the start of the program header table
    pub e_phoff: u64,
    /// Points to the start of the section header table
    pub e_shoff: u64,
    /// Interpretation of this field depends on the target architecture
    pub e_flags: u32,
    /// Contains the size of the header, normally 64 Bytes for 64-bit and 52 Bytes for 32-bit format
    pub e_ehsize: u16,
    /// Contains the size of a program header table entry
    pub e_phentsize: u16,
    /// Contains the number of entries in the program header table
    pub e_phnum: u16,
    /// Contains the size of a section header table entry
    pub e_shentsize: u16,
    /// Contains the number of entries in the section header table
    pub e_shnum: u16,
    /// Contains index of the section header table entry that contains the section names
    pub e_shstrndx: u16,
}

// Identity offsets
pub const EI_MAG0: usize = 0x00;
pub const EI_MAG1: usize = 0x01;
pub const EI_MAG2: usize = 0x02;
pub const EI_MAG3: usize = 0x03;
/// File ident class byte offset, this byte is set to either `1` or `2` to signify 32-bit or 64-bit format, respectively
pub const EI_CLASS: usize = 0x04;
/// File ident data byte offset, this byte is set to either `1` or `2` to signify little or big endianness, respectively
pub const EI_DATA: usize = 0x05;
/// File ident version byte offset, this byte is set to `1` for the original and current version of ELF
pub const EI_VERSION: usize = 0x6;
/// File ident OS ABI byte offset, identifies the target operating system ABI
pub const EI_OSABI: usize = 0x07;
/// File ident ABI version byte offset, depends on ABI for interpretation
pub const EI_ABIVERSION: usize = 0x08;

// OS ABI types
stringable_consts_block! {
    const stringable: u8 {
        /// UNIX System V ABI/None
        pub OSABI_NONE = 0;
        /// HP-UX operating system
        pub OSABI_HPUX = 1;
        /// NetBSD
        pub OSABI_NETBSD = 2;
        /// GNU/Linux
        pub OSABI_GNU = 3; // I hate this and agree with all threads from ~10 years ago when this was redefined. OASBI should have continued to refer to the _kernel_ while using notes to find userland. Whatever.
        /// GNU/Hurd
        pub OSABI_HURD = 4;
        /// Solaris
        pub OSABI_SOLARIS = 6;
        /// AIX
        pub OSABI_AIX = 7;
        /// IRIX
        pub OSABI_IRIX = 8;
        /// FreeBSD
        pub OSABI_FREEBSD = 9;
        /// TRU64 UNIX
        pub OSABI_TRU64 = 10;
        /// Novell Modesto
        pub OSABI_MODESTO = 11;
        /// OpenBSD
        pub OSABI_OPENBSD = 12;
        /// OpenVMS
        pub OSABI_OPENVMS = 13;
        /// Hewlett-Packard Non-Stop Kernel
        pub OSABI_NSK = 14;
        /// AROS
        pub OSABI_AROS = 15;
        /// FenixOS
        pub OSABI_FENIXOS = 16;
        /// Nuxi CloudABI
        pub OSABI_CLOUDABI = 17;
    }

    const ignore: u8 {
        /// Historical alias for OSABI_GNU.
        pub OSABI_LINUX = 3;
        /// First architecture-specific OS ABI
        pub OSABI_FIRST_ARCH = 64;
        /// Standalone (embedded) application
        pub OSABI_STANDALONE = 255;
        /// Last Architecture-specific OS ABI
        pub OSABI_LAST_ARCH = 255;
    }

    pub fn osabi_to_str(value: u8) -> &'static str {
        match value {
            unknown => {
                if unknown >= OSABI_FIRST_ARCH && unknown <= OSABI_LAST_ARCH {
                    "OSABI_UNKNOWN_ARCH_SPECIFIC_RANGE"
                } else {
                    "OSABI_UNKNOWN"
                }
            }
        }
    }
}

// File types
stringable_consts_block! {
    const stringable: u16 {
        /// Unknown object file type
        pub ET_NONE = 0x00;
        /// Relocatable file type
        pub ET_REL = 0x01;
        /// Executable file type
        pub ET_EXEC = 0x02;
        /// Shared object file type
        pub ET_DYN = 0x03;
        /// Core file type
        pub ET_CORE = 0x04;
    }

    const ignore: u16 {
        /// Start of operating system specific range (inclusive)
        pub ET_LOOS = 0xFE00;
        /// End of operating system specific range (inclusive)
        pub ET_HIOS = 0xFEFF;
        /// Start of processor specific range (inclusive)
        pub ET_LOPROC = 0xFF00;
        /// End of processor specific range (inclusive)
        pub ET_HIPROC = 0xFFFF;
    }

    pub fn et_to_str(value: u16) -> &'static str {
        match value {
            unknown => {
                if unknown >= ET_LOOS && unknown <= ET_HIOS {
                    "ET_UNKNOWN_OS_RANGE"
                } else if unknown >= ET_LOPROC && unknown <= ET_HIPROC {
                    "ET_UNKNOWN_PROC_RANGE"
                } else {
                    "ET_UNKNOWN"
                }
            }
        }
    }
}

// Version types
stringable_consts_block! {
    const stringable: u32 {
        pub EV_NONE = 0;
        pub EV_CURRENT = 1;
    }

    const ignore: u32 {}

    pub fn ev_to_str(value: u32) -> &'static str {
        match value {
            _unknown => "EV_UNKNOWN"
        }
    }
}

// Machine types
stringable_consts_block! {
    const stringable: u16 {
        /// No machine
        pub EM_NONE = 0;
        /// AT&T WE 32100
        pub EM_M32 = 1;
        /// SPARC
        pub EM_SPARC = 2;
        /// Intel 386
        pub EM_386 = 3;
        /// Motorola 68000
        pub EM_68K = 4;
        /// Motorola 88000
        pub EM_88K = 5;
        /// Intel MCU
        pub EM_IAMCU = 6;
        /// Intel 80860
        pub EM_860 = 7;
        /// MIPS R3000
        pub EM_MIPS = 8;
        /// IBM System/370
        pub EM_S370 = 9;
        /// MIPS RS3000 Little-endian
        pub EM_MIPS_RS3_LE = 10;
        /// Hewlett-Packard PA-RISC
        pub EM_PARISC = 15;
        /// Fujitsu VPP500
        pub EM_VPP500 = 17;
        /// Enhanced instruction set SPARC
        pub EM_SPARC32PLUS = 18;
        /// Intel 80960
        pub EM_960 = 19;
        /// PowerPC
        pub EM_PPC = 20;
        /// PowerPC64
        pub EM_PPC64 = 21;
        /// IBM System/390
        pub EM_S390 = 22;
        /// IBM SPU/SPC
        pub EM_SPU = 23;
        /// NEC V800
        pub EM_V800 = 36;
        /// Fujitsu FR20
        pub EM_FR20 = 37;
        /// TRW RH-32
        pub EM_RH32 = 38;
        /// Motorola RCE
        pub EM_RCE = 39;
        /// ARM
        pub EM_ARM = 40;
        /// DEC Alpha
        pub EM_ALPHA = 41;
        /// Hitachi SH
        pub EM_SH = 42;
        /// SPARC V9
        pub EM_SPARCV9 = 43;
        /// Siemens TriCore
        pub EM_TRICORE = 44;
        /// Argonaut RISC Core
        pub EM_ARC = 45;
        /// Hitachi H8/300
        pub EM_H8_300 = 46;
        /// Hitachi H8/300H
        pub EM_H8_300H = 47;
        /// Hitachi H8S
        pub EM_H8S = 48;
        /// Hitachi H8/500
        pub EM_H8_500 = 49;
        /// Intel IA-64 processor architecture
        pub EM_IA_64 = 50;
        /// Stanford MIPS-X
        pub EM_MIPS_X = 51;
        /// Motorola ColdFire
        pub EM_COLDFIRE = 52;
        /// Motorola M68HC12
        pub EM_68HC12 = 53;
        /// Fujitsu MMA Multimedia Accelerator
        pub EM_MMA = 54;
        /// Siemens PCP
        pub EM_PCP = 55;
        /// Sony nCPU embedded RISC processor
        pub EM_NCPU = 56;
        /// Denso NDR1 microprocessor
        pub EM_NDR1 = 57;
        /// Motorola Star*Core processor
        pub EM_STARCORE = 58;
        /// Toyota ME16 processor
        pub EM_ME16 = 59;
        /// STMicroelectronics ST100 processor
        pub EM_ST100 = 60;
        /// Advanced Logic Corp. TinyJ embedded processor family
        pub EM_TINYJ = 61;
        /// AMD x86-64 architecture
        pub EM_X86_64 = 62;
        /// Sony DSP Processor
        pub EM_PDSP = 63;
        /// Digital Equipment Corp. PDP-10
        pub EM_PDP10 = 64;
        /// Digital Equipment Corp. PDP-11
        pub EM_PDP11 = 65;
        /// Siemens FX66 microcontroller
        pub EM_FX66 = 66;
        /// STMicroelectronics ST9+ 8/16 bit microcontroller
        pub EM_ST9PLUS = 67;
        /// STMicroelectronics ST7 8-bit microcontroller
        pub EM_ST7 = 68;
        /// Motorola MC68HC16 Microcontroller
        pub EM_68HC16 = 69;
        /// Motorola MC68HC11 Microcontroller
        pub EM_68HC11 = 70;
        /// Motorola MC68HC08 Microcontroller
        pub EM_68HC08 = 71;
        /// Motorola MC68HC05 Microcontroller
        pub EM_68HC05 = 72;
        /// Silicon Graphics SVx
        pub EM_SVX = 73;
        /// STMicroelectronics ST19 8-bit microcontroller
        pub EM_ST19 = 74;
        /// Digital VAX
        pub EM_VAX = 75;
        /// Axis Communications 32-bit embedded processor
        pub EM_CRIS = 76;
        /// Infineon Technologies 32-bit embedded processor
        pub EM_JAVELIN = 77;
        /// Element 14 64-bit DSP Processor
        pub EM_FIREPATH = 78;
        /// LSI Logic 16-bit DSP Processor
        pub EM_ZSP = 79;
        /// Donald Knuth's educational 64-bit processor
        pub EM_MMIX = 80;
        /// Harvard University machine-independent object files
        pub EM_HUANY = 81;
        /// SiTera Prism
        pub EM_PRISM = 82;
        /// Atmel AVR 8-bit microcontroller
        pub EM_AVR = 83;
        /// Fujitsu FR30
        pub EM_FR30 = 84;
        /// Mitsubishi D10V
        pub EM_D10V = 85;
        /// Mitsubishi D30V
        pub EM_D30V = 86;
        /// NEC v850
        pub EM_V850 = 87;
        /// Mitsubishi M32R
        pub EM_M32R = 88;
        /// Matsushita MN10300
        pub EM_MN10300 = 89;
        /// Matsushita MN10200
        pub EM_MN10200 = 90;
        /// picoJava
        pub EM_PJ = 91;
        /// OpenRISC 32-bit embedded processor
        pub EM_OPENRISC = 92;
        /// ARC International ARCompact processor (old spelling/synonym: EM_ARC_A5)
        pub EM_ARC_COMPACT = 93;
        /// Tensilica Xtensa Architecture
        pub EM_XTENSA = 94;
        /// Alphamosaic VideoCore processor
        pub EM_VIDEOCORE = 95;
        /// Thompson Multimedia General Purpose Processor
        pub EM_TMM_GPP = 96;
        /// National Semiconductor 32000 series
        pub EM_NS32K = 97;
        /// Tenor Network TPC processor
        pub EM_TPC = 98;
        /// Trebia SNP 1000 processor
        pub EM_SNP1K = 99;
        /// STMicroelectronics (www.st.com) ST200
        pub EM_ST200 = 100;
        /// Ubicom IP2xxx microcontroller family
        pub EM_IP2K = 101;
        /// MAX Processor
        pub EM_MAX = 102;
        /// National Semiconductor CompactRISC microprocessor
        pub EM_CR = 103;
        /// Fujitsu F2MC16
        pub EM_F2MC16 = 104;
        /// Texas Instruments embedded microcontroller msp430
        pub EM_MSP430 = 105;
        /// Analog Devices Blackfin (DSP) processor
        pub EM_BLACKFIN = 106;
        /// S1C33 Family of Seiko Epson processors
        pub EM_SE_C33 = 107;
        /// Sharp embedded microprocessor
        pub EM_SEP = 108;
        /// Arca RISC Microprocessor
        pub EM_ARCA = 109;
        /// Microprocessor series from PKU-Unity Ltd. and MPRC  of Peking University
        pub EM_UNICORE = 110;
        /// eXcess: 16/32/64-bit configurable embedded CPU
        pub EM_EXCESS = 111;
        /// Icera Semiconductor Inc. Deep Execution Processor
        pub EM_DXP = 112;
        /// Altera Nios II soft-core processor
        pub EM_ALTERA_NIOS2 = 113;
        /// National Semiconductor CompactRISC CRX
        pub EM_CRX = 114;
        /// Motorola XGATE embedded processor
        pub EM_XGATE = 115;
        /// Infineon C16x/XC16x processor
        pub EM_C166 = 116;
        /// Renesas M16C series microprocessors
        pub EM_M16C = 117;
        /// Microchip Technology dsPIC30F Digital Signal Controller
        pub EM_DSPIC30F = 118;
        /// Freescale Communication Engine RISC core
        pub EM_CE = 119;
        /// Renesas M32C series microprocessors
        pub EM_M32C = 120;
        /// Altium TSK3000 core
        pub EM_TSK3000 = 131;
        /// Freescale RS08 embedded processor
        pub EM_RS08 = 132;
        /// Analog Devices SHARC family of 32-bit DSP processors
        pub EM_SHARC = 133;
        /// Cyan Technology eCOG2 microprocessor
        pub EM_ECOG2 = 134;
        /// Sunplus S+core7 RISC processor
        pub EM_SCORE7 = 135;
        /// New Japan Radio (NJR) 24-bit DSP Processor
        pub EM_DSP24 = 136;
        /// Broadcom VideoCore III processor
        pub EM_VIDEOCORE3 = 137;
        /// RISC processor for Lattice FPGA architecture
        pub EM_LATTICEMICO32 = 138;
        /// Seiko Epson C17 family
        pub EM_SE_C17 = 139;
        /// The Texas Instruments TMS320C6000 DSP family
        pub EM_TI_C6000 = 140;
        /// The Texas Instruments TMS320C2000 DSP family
        pub EM_TI_C2000 = 141;
        /// The Texas Instruments TMS320C55x DSP family
        pub EM_TI_C5500 = 142;
        /// STMicroelectronics 64bit VLIW Data Signal Processor
        pub EM_MMDSP_PLUS = 160;
        /// Cypress M8C microprocessor
        pub EM_CYPRESS_M8C = 161;
        /// Renesas R32C series microprocessors
        pub EM_R32C = 162;
        /// NXP Semiconductors TriMedia architecture family
        pub EM_TRIMEDIA = 163;
        /// Qualcomm Hexagon processor
        pub EM_HEXAGON = 164;
        /// Intel 8051 and variants
        pub EM_8051 = 165;
        /// STMicroelectronics STxP7x family of configurable and extensible RISC processors
        pub EM_STXP7X = 166;
        /// Andes Technology compact code size embedded RISC processor family
        pub EM_NDS32 = 167;
        /// Cyan Technology eCOG1X family
        pub EM_ECOG1X = 168;
        /// Dallas Semiconductor MAXQ30 Core Micro-controllers
        pub EM_MAXQ30 = 169;
        /// New Japan Radio (NJR) 16-bit DSP Processor
        pub EM_XIMO16 = 170;
        /// M2000 Reconfigurable RISC Microprocessor
        pub EM_MANIK = 171;
        /// Cray Inc. NV2 vector architecture
        pub EM_CRAYNV2 = 172;
        /// Renesas RX family
        pub EM_RX = 173;
        /// Imagination Technologies META processor architecture
        pub EM_METAG = 174;
        /// MCST Elbrus general purpose hardware architecture
        pub EM_MCST_ELBRUS = 175;
        /// Cyan Technology eCOG16 family
        pub EM_ECOG16 = 176;
        /// National Semiconductor CompactRISC CR16 16-bit microprocessor
        pub EM_CR16 = 177;
        /// Freescale Extended Time Processing Unit
        pub EM_ETPU = 178;
        /// Infineon Technologies SLE9X core
        pub EM_SLE9X = 179;
        /// Intel L10M
        pub EM_L10M = 180;
        /// Intel K10M
        pub EM_K10M = 181;
        /// ARM AArch64
        pub EM_AARCH64 = 183;
        /// Atmel Corporation 32-bit microprocessor family
        pub EM_AVR32 = 185;
        /// STMicroeletronics STM8 8-bit microcontroller
        pub EM_STM8 = 186;
        /// Tilera TILE64 multicore architecture family
        pub EM_TILE64 = 187;
        /// Tilera TILEPro multicore architecture family
        pub EM_TILEPRO = 188;
        /// Xilinx MicroBlaze 32-bit RISC soft processor core
        pub EM_MICROBLAZE = 189;
        /// NVIDIA CUDA architecture
        pub EM_CUDA = 190;
        /// Tilera TILE-Gx multicore architecture family
        pub EM_TILEGX = 191;
        /// CloudShield architecture family
        pub EM_CLOUDSHIELD = 192;
        /// KIPO-KAIST Core-A 1st generation processor family
        pub EM_COREA_1ST = 193;
        /// KIPO-KAIST Core-A 2nd generation processor family
        pub EM_COREA_2ND = 194;
        /// Synopsys ARCompact V2
        pub EM_ARC_COMPACT2 = 195;
        /// Open8 8-bit RISC soft processor core
        pub EM_OPEN8 = 196;
        /// Renesas RL78 family
        pub EM_RL78 = 197;
        /// Broadcom VideoCore V processor
        pub EM_VIDEOCORE5 = 198;
        /// Renesas 78KOR family
        pub EM_78KOR = 199;
        /// Freescale 56800EX Digital Signal Controller (DSC)
        pub EM_56800EX = 200;
        /// Beyond BA1 CPU architecture
        pub EM_BA1 = 201;
        /// Beyond BA2 CPU architecture
        pub EM_BA2 = 202;
        /// XMOS xCORE processor family
        pub EM_XCORE = 203;
        /// Microchip 8-bit PIC(r) family
        pub EM_MCHP_PIC = 204;
        /// Reserved by Intel
        pub EM_INTEL205 = 205;
        /// Reserved by Intel
        pub EM_INTEL206 = 206;
        /// Reserved by Intel
        pub EM_INTEL207 = 207;
        /// Reserved by Intel
        pub EM_INTEL208 = 208;
        /// Reserved by Intel
        pub EM_INTEL209 = 209;
        /// KM211 KM32 32-bit processor
        pub EM_KM32 = 210;
        /// KM211 KMX32 32-bit processor
        pub EM_KMX32 = 211;
        /// KM211 KMX16 16-bit processor
        pub EM_KMX16 = 212;
        /// KM211 KMX8 8-bit processor
        pub EM_KMX8 = 213;
        /// KM211 KVARC processor
        pub EM_KVARC = 214;
        /// Paneve CDP architecture family
        pub EM_CDP = 215;
        /// Cognitive Smart Memory Processor
        pub EM_COGE = 216;
        /// iCelero CoolEngine
        pub EM_COOL = 217;
        /// Nanoradio Optimized RISC
        pub EM_NORC = 218;
        /// CSR Kalimba architecture family
        pub EM_CSR_KALIMBA = 219;
        /// AMD GPU architecture
        pub EM_AMDGPU = 224;
        /// RISC-V
        pub EM_RISCV = 243;
        /// Lanai 32-bit processor
        pub EM_LANAI = 244;
        /// Linux kernel bpf virtual machine
        pub EM_BPF = 247;
        /// NEC SX-Aurora VE
        pub EM_VE = 251;
        /// C-SKY 32-bit processor
        pub EM_CSKY = 252;
        /// LoongArch
        pub EM_LOONGARCH = 258;
    }

    const ignore: u16 {}

    pub fn em_to_str(value: u16) -> &'static str {
        match value {
            _unknown => "EM_UNKNOWN"
        }
    }
}
